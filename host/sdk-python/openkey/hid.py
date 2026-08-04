"""Backend de transporte USB HID real via hidapi (CTAP HID)

Permite descobrir, abrir e comunicar com dispositivos OpenKey (ou qualquer
dispositivo FIDO2) através de relatórios USB HID de 64 bytes, seguindo a
especificação FIDO2 CTAPHID (Seção 11).

O módulo é opcional: se ``hidapi`` não estiver instalado, a importação continua
funcionando e apenas as funções que dependem de hardware real levantam
``TransportError`` com uma mensagem clara.
"""

import os
import time
from typing import Dict, List, Optional, Tuple

from openkey.exceptions import TransportError
from openkey.transport import (
    CTAPHID_PACKET_SIZE,
    CTAPHID_BROADCAST_CID,
    CTAPHID_CMD_BIT,
    CMD_INIT,
    CMD_ERROR,
    CMD_KEEPALIVE,
    CtapHidMessageAssembler,
    CtapHidPacket,
)

try:
    import hid
except ImportError:  # pragma: no cover - depende do ambiente
    hid = None

# VID/PID padrão da plataforma de referência (firmware RP2350 e docs de boards)
OPENKEY_VID = 0x16C0
OPENKEY_PID = 0x27DB

# Byte do Report ID no USB HID (usado no write/read do hidapi)
_HID_REPORT_ID = 0x00
_DEFAULT_TIMEOUT_MS = 5000


def _ensure_hid() -> None:
    """Garante que a biblioteca ``hidapi`` esteja instalada."""
    if hid is None:
        raise TransportError(
            "O transporte USB HID requer o pacote 'hidapi'. "
            "Instale com: pip install 'openkey-sdk[hid]'"
        )


def discover_devices(
    vid: Optional[int] = OPENKEY_VID,
    pid: Optional[int] = OPENKEY_PID,
    serial_number: Optional[str] = None,
) -> List[Dict]:
    """Descobre dispositivos HID conectados.

    Args:
        vid: Vendor ID para filtrar. ``None`` lista todos.
        pid: Product ID para filtrar. ``None`` lista todos.
        serial_number: Número de série para filtrar (``None`` ignora).

    Returns:
        Lista de dicionários com as informações de cada dispositivo HID
        (chaves típicas do hidapi: ``vendor_id``, ``product_id``,
        ``product_string``, ``manufacturer_string``, ``serial_number``,
        ``path``).
    """
    _ensure_hid()
    devices: List[Dict] = []
    for info in hid.enumerate():
        if vid is not None and info.get("vendor_id") != vid:
            continue
        if pid is not None and info.get("product_id") != pid:
            continue
        if serial_number is not None and info.get("serial_number") != serial_number:
            continue
        devices.append(dict(info))
    return devices


class HidTransportBackend:
    """Backend de transporte CTAPHID sobre USB HID real.

    Implementa o contrato ``send_cmd(cid, cmd, payload) -> bytes`` usado por
    ``OpenKeyDevice``. O canal (CID) é estabelecido automaticamente durante a
    troca ``CTAPHID_INIT`` e mantido para os comandos seguintes.
    """

    def __init__(
        self,
        vid: int = OPENKEY_VID,
        pid: int = OPENKEY_PID,
        serial_number: Optional[str] = None,
        path: Optional[bytes] = None,
        timeout_ms: int = _DEFAULT_TIMEOUT_MS,
    ):
        self.vid = vid
        self.pid = pid
        self.serial_number = serial_number
        self.path = path
        self.timeout_ms = timeout_ms
        self._device = None
        self._cid: Optional[int] = None

    @property
    def cid(self) -> Optional[int]:
        """CID (canal CTAPHID) atribuído após a inicialização."""
        return self._cid

    # ------------------------------------------------------------------
    # Gerenciamento de conexão
    # ------------------------------------------------------------------

    def open(self) -> "HidTransportBackend":
        """Abre o dispositivo HID (sem inicializar o canal CTAPHID)."""
        _ensure_hid()
        try:
            device = hid.device()
            if self.path is not None:
                device.open_path(self.path)
            else:
                device.open(self.vid, self.pid, self.serial_number)
        except Exception as exc:
            raise TransportError(
                f"Falha ao abrir dispositivo HID (VID=0x{self.vid:04X}, "
                f"PID=0x{self.pid:04X}): {exc}"
            ) from exc
        self._device = device
        return self

    def close(self) -> None:
        """Fecha o dispositivo HID, se aberto."""
        if self._device is not None:
            try:
                self._device.close()
            except Exception:
                pass
            self._device = None

    def __enter__(self) -> "HidTransportBackend":
        return self.open()

    def __exit__(self, *exc) -> None:
        self.close()

    def initialize(self, nonce: Optional[bytes] = None) -> bytes:
        """Executa ``CTAPHID_INIT`` e armazena o CID atribuído.

        Returns:
            Payload bruto da resposta INIT (nonce + CID + protocol version).
        """
        if self._device is None:
            raise TransportError("Dispositivo não aberto (chame open() antes)")
        nonce = nonce or os.urandom(8)
        if len(nonce) != 8:
            raise TransportError("Nonce do INIT deve ter exatamente 8 bytes")
        return self.send_cmd(CTAPHID_BROADCAST_CID, CMD_INIT, nonce)

    # ------------------------------------------------------------------
    # Contrato de envio (usado por OpenKeyDevice)
    # ------------------------------------------------------------------

    def send_cmd(self, cid: int, cmd: int, payload: bytes) -> bytes:
        """Envia um comando CTAPHID e retorna o payload da resposta.

        Para ``CMD_INIT`` executa o handshake completo e atualiza o CID interno.
        Para os demais comandos usa o CID estabelecido na inicialização.
        """
        if self._device is None:
            raise TransportError("Dispositivo não aberto (chame open() antes)")

        if cmd == CMD_INIT:
            target_cid = CTAPHID_BROADCAST_CID
        else:
            target_cid = self._cid
            if target_cid is None:
                raise TransportError(
                    "Canal CTAPHID não inicializado (chame initialize() antes)"
                )

        for packet in CtapHidMessageAssembler.fragment_message(target_cid, cmd, payload):
            self._write_report(packet)

        resp_cid, resp_cmd, resp_payload = self._read_response(target_cid)

        if cmd == CMD_INIT:
            if len(resp_payload) < 9:
                raise TransportError("Resposta CTAPHID_INIT inválida (tamanho curto)")
            assigned_cid = int.from_bytes(resp_payload[8:12], "big")
            if assigned_cid == CTAPHID_BROADCAST_CID:
                raise TransportError("Dispositivo retornou CID inválido (broadcast)")
            self._cid = assigned_cid

        return resp_payload

    # ------------------------------------------------------------------
    # Baixo nível (escrita/leitura de relatórios USB HID)
    # ------------------------------------------------------------------

    def _write_report(self, packet: bytes) -> None:
        """Escreve um relatório HID de 64 bytes no dispositivo."""
        if len(packet) != CTAPHID_PACKET_SIZE:
            raise TransportError("Pacote CTAPHID deve ter exatamente 64 bytes")
        try:
            written = self._device.write(bytes([_HID_REPORT_ID]) + packet)
        except Exception as exc:
            raise TransportError(f"Falha na escrita USB HID: {exc}") from exc
        if written != CTAPHID_PACKET_SIZE + 1:
            raise TransportError("Falha na escrita USB HID (bytes parciais)")

    def _read_report(self, timeout_ms: int) -> bytes:
        """Lê um relatório HID, normalizando o Report ID do primeiro byte."""
        try:
            data = self._device.read(CTAPHID_PACKET_SIZE + 1, timeout_ms)
        except Exception as exc:
            raise TransportError(f"Falha na leitura USB HID: {exc}") from exc
        if not data:
            raise TransportError("Timeout aguardando resposta USB HID")
        data = bytes(data)
        # hidapi costuma prefixar o Report ID (0x00); remova se vier como 1 byte extra
        if len(data) == CTAPHID_PACKET_SIZE + 1 and data[0] == _HID_REPORT_ID:
            data = data[1:]
        if len(data) < CTAPHID_PACKET_SIZE:
            data = data.ljust(CTAPHID_PACKET_SIZE, b"\x00")
        return data[:CTAPHID_PACKET_SIZE]

    def _read_response(self, cid: int) -> Tuple[int, int, bytes]:
        """Lê e remonta uma resposta CTAPHID, ignorando keepalives.

        Returns:
            Tupla ``(cid, cmd, payload)`` da resposta remontada.
        """
        deadline = time.monotonic() + (self.timeout_ms / 1000.0)

        # 1) Aguarda o início da resposta (primeiro pacote Init), ignorando
        #    keepalives e pacotes de outros canais.
        packets: List[bytes] = []
        expected_len: Optional[int] = None
        while True:
            remaining_ms = max(1, int((deadline - time.monotonic()) * 1000))
            report = self._read_report(remaining_ms)
            rcid, cmd_or_seq, payload, total_len = CtapHidPacket.parse_packet(report)

            if rcid != cid and rcid != CTAPHID_BROADCAST_CID:
                continue  # canal de outro processo/transação

            if cmd_or_seq == CMD_KEEPALIVE:
                continue  # status de processamento (0xBB) — aguardar resposta real

            if cmd_or_seq == CMD_ERROR:
                code = payload[0] if payload else 0
                raise TransportError(f"CTAPHID_ERROR recebido (0x{code:02X})")

            if not (cmd_or_seq & CTAPHID_CMD_BIT):
                continue  # pacote de continuação sem Init — ignorar

            packets.append(report)
            expected_len = total_len
            break

        # 2) Coleta os pacotes de continuação até completar o payload.
        while True:
            try:
                res_cid, res_cmd, res_payload = CtapHidMessageAssembler.assemble_message(
                    packets
                )
            except TransportError:
                res_payload = b""
            if expected_len is not None and len(res_payload) >= expected_len:
                return res_cid, res_cmd, res_payload[:expected_len]

            remaining_ms = max(1, int((deadline - time.monotonic()) * 1000))
            report = self._read_report(remaining_ms)
            rcid, cmd_or_seq, _, _ = CtapHidPacket.parse_packet(report)
            if rcid != cid and rcid != CTAPHID_BROADCAST_CID:
                continue
            if cmd_or_seq == CMD_KEEPALIVE:
                continue
            packets.append(report)


def open_device(
    vid: int = OPENKEY_VID,
    pid: int = OPENKEY_PID,
    serial_number: Optional[str] = None,
    path: Optional[bytes] = None,
) -> HidTransportBackend:
    """Abre e inicializa um dispositivo OpenKey (conveniência).

    Equivale a ``HidTransportBackend(...).open().initialize()``.

    Returns:
        Backend já inicializado (canal CTAPHID estabelecido).
    """
    backend = HidTransportBackend(
        vid=vid, pid=pid, serial_number=serial_number, path=path
    )
    backend.open()
    try:
        backend.initialize()
    except Exception:
        backend.close()
        raise
    return backend
