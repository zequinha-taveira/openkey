"""DiagnosticsService do OpenKey Manager (G10-T10).

Integra o ``host/diagnostics`` (serviço headless) com o ``DeviceController``
através de um adapter. Nenhuma dependência de Qt aqui (ADR-0013).
"""

from typing import Optional

from openkey_diagnostics.diagnostics import (
    DiagnosticsService as CoreDiagnosticsService,
)

from openkey_manager.core.device import DeviceController, DeviceError
from openkey_manager.core.models import DiagnosticsReport


class DiagnosticsError(Exception):
    """Erro ao executar o diagnóstico no OpenKey Manager."""


class _ControllerAdapter:
    """Adapta o ``DeviceController`` à interface do ``host/diagnostics``.

    O ``DeviceController.get_info()`` devolve um ``DeviceInfo`` cujos atributos
    (``aaguid``, ``versions``, ``options``, ``max_msg_size``,
    ``pin_uv_auth_protocols``) satisfazem o contrato duck-typed do serviço.
    """

    def __init__(self, controller: DeviceController):
        self._controller = controller

    def get_info(self):
        return self._controller.get_info()

    def get_pin_retries(self) -> int:
        return self._controller.get_pin_retries()

    def get_firmware_diagnostics(self) -> Optional[dict]:
        # O openkey-sdk ainda não expõe o protocolo de diagnóstico de firmware
        # (flash/RNG/secrets). Retornar None marca esses checks como "não
        # verificado" no relatório (G10-T12 expande quando houver suporte).
        return None


class DiagnosticsService:
    """Executa o diagnóstico do dispositivo conectado via host/diagnostics."""

    def __init__(self, controller: DeviceController):
        self._controller = controller
        self._core = CoreDiagnosticsService(_ControllerAdapter(controller))

    @property
    def controller(self) -> DeviceController:
        return self._controller

    def run(self) -> DiagnosticsReport:
        if not self._controller.is_connected:
            raise DeviceError("Nenhum dispositivo conectado")
        try:
            raw = self._core.run()
        except Exception as exc:
            raise DiagnosticsError(str(exc)) from exc
        return DiagnosticsReport(
            device_connected=raw.device_connected,
            firmware_version=raw.firmware_version,
            checks=dict(raw.checks),
            details=dict(raw.details),
            passed=raw.passed,
            generated_at=raw.generated_at,
        )
