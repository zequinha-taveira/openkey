"""DeviceController: ciclo de vida e operações do dispositivo (sem Qt).

A camada ``core`` segue o ADR-0013: nenhum import de Qt aqui. O
``DeviceController`` conversa com o hardware através de um *backend* injetável
(por padrão, o ``openkey-sdk``), o que permite testar a lógica headless com um
``FakeBackend``/``FakeDevice``.
"""

from typing import Callable, List, Optional

from openkey_manager.core.models import (
    ConnectionState,
    DeviceCandidate,
    DeviceInfo,
)


class DeviceError(Exception):
    """Erro de operação de dispositivo no OpenKey Manager."""


class DeviceBackend:
    """Abstração sobre o ``openkey-sdk`` (substituível em testes).

    Contrato duck-typed:
      - ``discover(vid, pid, serial_number) -> List[DeviceCandidate]``
      - ``open(candidate, *, vid, pid, serial_number, path) -> device``
      - ``close(device)``
      - ``get_info(device) -> GetInfoResponse``
      - ``reset(device)``
      - ``pin_client(device, protocol_version) -> PinClient``
      - ``ctap2(device) -> Ctap2Client``
    """

    def discover(
        self,
        vid: Optional[int] = None,
        pid: Optional[int] = None,
        serial_number: Optional[str] = None,
    ) -> List[DeviceCandidate]:
        from openkey import OPENKEY_PID, OPENKEY_VID, discover_devices

        raw_devices = discover_devices(
            vid=vid if vid is not None else OPENKEY_VID,
            pid=pid if pid is not None else OPENKEY_PID,
            serial_number=serial_number,
        )
        return [
            DeviceCandidate(
                vid=int(d.get("vendor_id", 0)),
                pid=int(d.get("product_id", 0)),
                serial_number=d.get("serial_number"),
                path=d.get("path"),
                product_string=d.get("product_string"),
                manufacturer_string=d.get("manufacturer_string"),
            )
            for d in raw_devices
        ]

    def open(
        self,
        candidate: Optional[DeviceCandidate] = None,
        *,
        vid: Optional[int] = None,
        pid: Optional[int] = None,
        serial_number: Optional[str] = None,
        path: Optional[bytes] = None,
    ):
        from openkey import OpenKeyDevice

        if candidate is not None:
            vid = vid if vid is not None else candidate.vid
            pid = pid if pid is not None else candidate.pid
            serial_number = (
                serial_number if serial_number is not None else candidate.serial_number
            )
            path = path if path is not None else candidate.path
        return OpenKeyDevice.from_hid(
            vid=vid, pid=pid, serial_number=serial_number, path=path
        )

    def close(self, device) -> None:
        backend = getattr(device, "_backend", None)
        if backend is not None:
            backend.close()

    def get_info(self, device):
        return device.get_info()

    def reset(self, device) -> None:
        device.reset()

    def pin_client(self, device, protocol_version: int):
        from openkey import PinClient

        return PinClient(self.ctap2(device), protocol_version=protocol_version)

    def credential_manager(self, device, pin: str, protocol_version: int):
        from openkey import CredentialManagementClient

        return CredentialManagementClient(
            self.ctap2(device), self.pin_client(device, protocol_version), pin=pin
        )

    def ctap2(self, device):
        if device._ctap2 is None:
            device.connect()
        return device._ctap2


class DeviceController:
    """Controlador central do dispositivo conectado.

    Notifica ouvintes (``listener(state, message)``) em cada transição de
    estado: CONNECTING -> CONNECTED / ERROR -> DISCONNECTED.
    """

    def __init__(self, backend: Optional[DeviceBackend] = None):
        self._backend = backend if backend is not None else DeviceBackend()
        self._device = None
        self._candidate: Optional[DeviceCandidate] = None
        self._info: Optional[DeviceInfo] = None
        self._state = ConnectionState.DISCONNECTED
        self._listeners: List[Callable[[ConnectionState, str], None]] = []

    # ------------------------------------------------------------------
    # Estado
    # ------------------------------------------------------------------

    @property
    def state(self) -> ConnectionState:
        return self._state

    @property
    def backend(self) -> DeviceBackend:
        """Backend de comunicação (compartilhado com o DiscoveryService)."""
        return self._backend

    @property
    def info(self) -> Optional[DeviceInfo]:
        return self._info

    @property
    def is_connected(self) -> bool:
        return self._state is ConnectionState.CONNECTED and self._device is not None

    def add_listener(self, callback: Callable[[ConnectionState, str], None]) -> None:
        """Registra um ouvinte de mudanças de estado."""
        self._listeners.append(callback)

    def _set_state(self, state: ConnectionState, message: str = "") -> None:
        self._state = state
        for listener in list(self._listeners):
            listener(state, message)

    # ------------------------------------------------------------------
    # Descoberta / conexão
    # ------------------------------------------------------------------

    def discover(
        self,
        vid: Optional[int] = None,
        pid: Optional[int] = None,
        serial_number: Optional[str] = None,
    ) -> List[DeviceCandidate]:
        return self._backend.discover(vid=vid, pid=pid, serial_number=serial_number)

    def connect(
        self,
        candidate: Optional[DeviceCandidate] = None,
        *,
        vid: Optional[int] = None,
        pid: Optional[int] = None,
        serial_number: Optional[str] = None,
        path: Optional[bytes] = None,
    ) -> DeviceInfo:
        """Abre o dispositivo, executa GetInfo e atualiza o estado."""
        self._set_state(ConnectionState.CONNECTING, "Abrindo dispositivo...")
        try:
            device = self._backend.open(
                candidate,
                vid=vid,
                pid=pid,
                serial_number=serial_number,
                path=path,
            )
        except Exception as exc:
            self._set_state(ConnectionState.ERROR, f"Falha ao abrir dispositivo: {exc}")
            raise DeviceError(str(exc)) from exc

        self._device = device
        self._candidate = candidate
        try:
            self._info = self._read_info()
        except Exception as exc:
            self._backend.close(device)
            self._device = None
            self._info = None
            self._set_state(ConnectionState.ERROR, f"Falha ao ler dispositivo: {exc}")
            raise DeviceError(str(exc)) from exc

        self._set_state(ConnectionState.CONNECTED, "Conectado")
        return self._info

    def disconnect(self) -> None:
        if self._device is not None:
            try:
                self._backend.close(self._device)
            finally:
                self._device = None
                self._candidate = None
                self._info = None
        self._set_state(ConnectionState.DISCONNECTED, "Desconectado")

    # ------------------------------------------------------------------
    # Operações
    # ------------------------------------------------------------------

    def get_info(self) -> DeviceInfo:
        if not self.is_connected:
            raise DeviceError("Nenhum dispositivo conectado")
        self._info = self._read_info()
        return self._info

    def reset(self) -> None:
        if not self.is_connected:
            raise DeviceError("Nenhum dispositivo conectado")
        self._backend.reset(self._device)

    def get_pin_retries(self) -> int:
        client = self._pin_client()
        return client.get_pin_retries()

    def setup_pin(self, new_pin: str) -> None:
        client = self._pin_client()
        client.get_key_agreement()
        client.set_pin(new_pin)

    def change_pin(self, current_pin: str, new_pin: str) -> None:
        client = self._pin_client()
        client.get_key_agreement()
        client.change_pin(current_pin, new_pin)

    def credential_manager(self, pin: str):
        """Cria um cliente de gestão de credenciais para o dispositivo.

        O ``pin`` é usado para obter o ``pinUvAuthToken`` (efêmero).
        """
        if not self.is_connected:
            raise DeviceError("Nenhum dispositivo conectado")
        return self._backend.credential_manager(
            self._device, pin, self._preferred_protocol()
        )

    # ------------------------------------------------------------------
    # Internos
    # ------------------------------------------------------------------

    def _read_info(self) -> DeviceInfo:
        raw = self._backend.get_info(self._device)
        candidate = self._candidate
        return DeviceInfo(
            aaguid=raw.aaguid,
            versions=list(raw.versions),
            options=dict(raw.options),
            max_msg_size=int(raw.max_msg_size),
            pin_uv_auth_protocols=list(raw.pin_uv_auth_protocols),
            extensions=list(raw.extensions),
            vid=candidate.vid if candidate else None,
            pid=candidate.pid if candidate else None,
            serial_number=candidate.serial_number if candidate else None,
            product_string=candidate.product_string if candidate else None,
            manufacturer_string=candidate.manufacturer_string if candidate else None,
        )

    def _preferred_protocol(self) -> int:
        if self._info and self._info.pin_uv_auth_protocols:
            return 2 if 2 in self._info.pin_uv_auth_protocols else self._info.pin_uv_auth_protocols[0]
        return 1

    def _pin_client(self):
        if not self.is_connected:
            raise DeviceError("Nenhum dispositivo conectado")
        return self._backend.pin_client(self._device, self._preferred_protocol())
