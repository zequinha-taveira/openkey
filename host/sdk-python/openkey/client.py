"""Cliente principal do OpenKey SDK"""

from typing import Optional
from openkey.transport import (
    CMD_CBOR,
    CMD_INIT,
    CTAPHID_BROADCAST_CID,
)
from openkey.ctap2 import Ctap2Client, GetInfoResponse, CtapLogHook

class OpenKeyDevice:
    """Dispositivo OpenKey (físico USB ou emulado)"""

    def __init__(self, transport_backend=None, log_hook: Optional[CtapLogHook] = None):
        self._cid = CTAPHID_BROADCAST_CID
        self._backend = transport_backend
        self._log_hook = log_hook
        self._ctap2: Optional[Ctap2Client] = None

    @classmethod
    def from_hid(cls, vid=None, pid=None, serial_number=None, path=None, log_hook=None) -> "OpenKeyDevice":
        """Cria um dispositivo conectado via USB HID real (hidapi).

        Descobre/abre o dispositivo, executa CTAPHID_INIT e retorna uma
        instância pronta para uso.
        """
        from openkey.hid import open_device, OPENKEY_VID, OPENKEY_PID
        backend = open_device(
            vid=vid if vid is not None else OPENKEY_VID,
            pid=pid if pid is not None else OPENKEY_PID,
            serial_number=serial_number,
            path=path,
        )
        dev = cls(transport_backend=backend, log_hook=log_hook)
        dev._cid = backend.cid or CTAPHID_BROADCAST_CID
        dev._ctap2 = Ctap2Client(
            lambda cmd, payload: dev._send_ctaphid(CMD_CBOR, bytes([cmd]) + payload),
            log_hook=log_hook,
        )
        return dev

    def _send_ctaphid(self, cmd: int, payload: bytes) -> bytes:
        if self._backend:
            return self._backend.send_cmd(self._cid, cmd, payload)
        # Mock / fallback backend se nenhum transporte físico fornecido
        if cmd == CMD_INIT:
            # Init response: 8b nonce + 4b cid + protocol/version
            self._cid = 0x12345678
            nonce = payload[:8] if payload else b"\x00" * 8
            return nonce + (0x12345678).to_bytes(4, "big") + b"\x02\x01\x00\x00\x01"
        elif cmd == CMD_CBOR:
            # Retorna CTAP2_OK + CBOR getInfo mock
            ctap_cmd = payload[0] if payload else 0
            if ctap_cmd == 0x04:  # getInfo
                import cbor2
                cbor_data = cbor2.dumps({
                    1: ["FIDO_2_0", "FIDO_2_1"],
                    2: ["hmac-secret", "credProtect"],
                    3: b"\x01" * 16,
                    4: {"rk": True, "up": True, "plat": False, "clientPin": False},
                    5: 1200,
                    6: [1, 2],
                })
                return b"\x00" + cbor_data
            return b"\x00"
        return b""

    def connect(self) -> "OpenKeyDevice":
        """Inicializa o canal CTAPHID e descobre o dispositivo"""
        # Executa INIT
        self._send_ctaphid(CMD_INIT, b"\x01\x02\x03\x04\x05\x06\x07\x08")
        self._ctap2 = Ctap2Client(
            lambda cmd, payload: self._send_ctaphid(CMD_CBOR, bytes([cmd]) + payload),
            log_hook=self._log_hook,
        )
        return self

    def get_info(self) -> GetInfoResponse:
        """Obtém as informações do autenticador"""
        if not self._ctap2:
            self.connect()
        assert self._ctap2 is not None
        return self._ctap2.get_info()

    def reset(self) -> None:
        """Executa reset de fábrica"""
        if not self._ctap2:
            self.connect()
        assert self._ctap2 is not None
        self._ctap2.reset()
