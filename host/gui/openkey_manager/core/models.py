"""Modelos de domínio do OpenKey Manager (sem dependências de Qt).

Estas classes são compartilhadas entre a camada ``core`` (lógica testável
headless) e a ``ui`` (PySide6), seguindo o ADR-0013.
"""

from dataclasses import dataclass, field
from enum import Enum
from typing import Dict, List, Optional


class ConnectionState(Enum):
    """Estado do ciclo de vida do dispositivo no OpenKey Manager."""

    DISCONNECTED = "disconnected"
    CONNECTING = "connecting"
    CONNECTED = "connected"
    ERROR = "error"


@dataclass
class DeviceCandidate:
    """Dispositivo detectado na descoberta (antes de abrir o canal)."""

    vid: int
    pid: int
    serial_number: Optional[str] = None
    path: Optional[bytes] = None
    product_string: Optional[str] = None
    manufacturer_string: Optional[str] = None

    @property
    def label(self) -> str:
        """Rótulo amigável para exibição em listas da GUI."""
        if self.serial_number:
            return f"OpenKey {self.serial_number}"
        return f"OpenKey {self.vid:04X}:{self.pid:04X}"

    @property
    def vid_pid(self) -> str:
        return f"{self.vid:04X}:{self.pid:04X}"

    def as_dict(self) -> dict:
        path = self.path
        return {
            "vid": self.vid,
            "pid": self.pid,
            "serial_number": self.serial_number,
            "path": bytes(path) if path is not None else None,
            "product_string": self.product_string,
            "manufacturer_string": self.manufacturer_string,
        }


@dataclass
class DeviceInfo:
    """Informações do autenticador (authenticatorGetInfo + identificação)."""

    aaguid: bytes
    versions: List[str] = field(default_factory=list)
    options: Dict[str, bool] = field(default_factory=dict)
    max_msg_size: int = 1200
    pin_uv_auth_protocols: List[int] = field(default_factory=list)
    extensions: List[str] = field(default_factory=list)
    vid: Optional[int] = None
    pid: Optional[int] = None
    serial_number: Optional[str] = None
    product_string: Optional[str] = None
    manufacturer_string: Optional[str] = None

    @property
    def aaguid_hex(self) -> str:
        return self.aaguid.hex()

    @property
    def label(self) -> str:
        if self.serial_number:
            return f"OpenKey {self.serial_number}"
        return f"OpenKey {self.aaguid_hex[:8]}"

    @property
    def supports_resident_keys(self) -> bool:
        return bool(self.options.get("rk"))

    @property
    def supports_pin(self) -> bool:
        return bool(self.options.get("clientPin") or self.options.get("pin"))

    @property
    def supports_credential_management(self) -> bool:
        return bool(
            self.options.get("credentialMgmt")
            or self.options.get("credentialManagement")
            or self.options.get("credential_mgmt")
        )


@dataclass
class Credential:
    """Credencial residente (agrega credentialInfo + user + rp)."""

    rp_id: str
    credential_id: bytes
    rp_name: Optional[str] = None
    user_id: Optional[bytes] = None
    user_name: Optional[str] = None
    user_display_name: Optional[str] = None

    @property
    def credential_id_hex(self) -> str:
        return self.credential_id.hex()

    @property
    def display_name(self) -> str:
        return (
            self.user_display_name
            or self.user_name
            or self.rp_name
            or self.rp_id
        )


@dataclass
class DiagnosticsReport:
    """Resultado do diagnóstico de integridade do dispositivo.

    Preenchido pelo ``DiagnosticsService`` (G10-T10). Os campos são mantidos
    aqui para que a camada ``ui`` dependa apenas do modelo.
    """

    device_connected: bool = False
    firmware_version: Optional[str] = None
    checks: Dict[str, bool] = field(default_factory=dict)
    details: Dict[str, str] = field(default_factory=dict)
    passed: bool = False
    generated_at: Optional[str] = None

    @property
    def passed_checks(self) -> int:
        return sum(1 for ok in self.checks.values() if ok)

    @property
    def failed_checks(self) -> int:
        return sum(1 for ok in self.checks.values() if not ok)


class UpdateStage(Enum):
    """Etapas do assistente de atualização de firmware (G10-T11)."""

    IDLE = "idle"
    DOWNLOADING = "downloading"
    TRANSFERRING = "transferring"
    VERIFYING = "verifying"
    DONE = "done"
    FAILED = "failed"


@dataclass
class UpdateSession:
    """Estado do assistente visual de atualização de firmware (G10-T11)."""

    stage: UpdateStage = UpdateStage.IDLE
    progress: float = 0.0
    message: str = ""
    error: Optional[str] = None
    target_version: Optional[str] = None
