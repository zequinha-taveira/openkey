"""Serviço de diagnóstico de dispositivos OpenKey (G10-T10).

O ``DiagnosticsService`` roda uma bateria de verificações de integridade e
conformidade FIDO2/CTAP2 sobre um *adapter* de dispositivo (duck-typed):

  - ``get_info()``                  -> objeto com ``aaguid``, ``versions``,
                                       ``options``, ``max_msg_size`` e
                                       ``pin_uv_auth_protocols``.
  - ``get_pin_retries()``           -> int com as tentativas restantes de PIN.
  - ``get_firmware_diagnostics()``  -> ``dict`` opcional com checks do firmware
                                       (ex.: flash/rng/secrets) ou ``None``
                                       quando o protocolo não é suportado.

Cada verificação produz um nome, um resultado (bool) e um detalhe textual. Os
checks do firmware não suportados são registrados apenas nos ``details`` e não
contam para o resultado ``passed``.
"""

from dataclasses import dataclass, field
from datetime import datetime, timezone
from typing import Dict, Optional


class DiagnosticsError(Exception):
    """Erro ao executar o diagnóstico."""


@dataclass
class DiagnosticsReport:
    """Resultado completo de uma execução de diagnóstico."""

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

    def as_dict(self) -> dict:
        return {
            "device_connected": self.device_connected,
            "firmware_version": self.firmware_version,
            "checks": dict(self.checks),
            "details": dict(self.details),
            "passed": self.passed,
            "generated_at": self.generated_at,
        }


def _now_iso() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="seconds")


class DiagnosticsService:
    """Executa as verificações de diagnóstico contra um adapter de dispositivo."""

    # Nomes de checks de integridade do firmware; registrados como "não
    # verificado" (apenas em ``details``) quando o adapter não os fornece.
    FIRMWARE_CHECKS = ("flash", "rng", "secrets")

    def __init__(self, adapter):
        self._adapter = adapter

    def run(self) -> DiagnosticsReport:
        report = DiagnosticsReport(generated_at=_now_iso())
        checks: Dict[str, bool] = {}
        details: Dict[str, str] = {}

        try:
            info = self._adapter.get_info()
        except Exception as exc:
            report.device_connected = False
            checks["getInfo"] = False
            details["getInfo"] = f"falha ao consultar GetInfo: {exc}"
            report.checks = checks
            report.details = details
            report.passed = False
            return report

        report.device_connected = True
        report.firmware_version = getattr(info, "firmware_version", None)

        checks["getInfo"] = True
        details["getInfo"] = "resposta OK"

        versions = list(getattr(info, "versions", None) or [])
        checks["versions"] = any(
            isinstance(v, str) and v.startswith("FIDO_2") for v in versions
        )
        details["versions"] = ", ".join(versions) if versions else "nenhuma versão anunciada"

        aaguid = getattr(info, "aaguid", None)
        checks["aaguid"] = isinstance(aaguid, (bytes, bytearray)) and len(aaguid) == 16
        details["aaguid"] = (
            bytes(aaguid).hex() if isinstance(aaguid, (bytes, bytearray)) else str(aaguid)
        )

        options = getattr(info, "options", None)
        checks["options"] = isinstance(options, dict)
        details["options"] = (
            str(dict(options)) if isinstance(options, dict) else "opções ausentes"
        )

        max_msg = getattr(info, "max_msg_size", 0)
        checks["maxMsgSize"] = isinstance(max_msg, int) and max_msg > 0
        details["maxMsgSize"] = str(max_msg)

        supports_pin = bool(options and (options.get("clientPin") or options.get("pin")))
        if supports_pin:
            protocols = list(getattr(info, "pin_uv_auth_protocols", None) or [])
            checks["pinUvAuthProtocols"] = len(protocols) > 0
            details["pinUvAuthProtocols"] = (
                ", ".join(map(str, protocols)) if protocols else "nenhum protocolo anunciado"
            )
            try:
                retries = self._adapter.get_pin_retries()
                checks["pinRetries"] = isinstance(retries, int)
                details["pinRetries"] = (
                    f"{retries} tentativas restantes" if isinstance(retries, int) else str(retries)
                )
            except Exception as exc:
                checks["pinRetries"] = False
                details["pinRetries"] = f"falha: {exc}"
        else:
            details["pinUvAuthProtocols"] = "clientPin não anunciado (skip)"
            details["pinRetries"] = "clientPin não anunciado (skip)"

        self._run_firmware_checks(checks, details)

        report.checks = checks
        report.details = details
        report.passed = all(checks.values())
        return report

    def _run_firmware_checks(
        self, checks: Dict[str, bool], details: Dict[str, str]
    ) -> None:
        try:
            firmware = self._adapter.get_firmware_diagnostics()
        except Exception as exc:
            firmware = None
            for name in self.FIRMWARE_CHECKS:
                details[name] = f"falha ao consultar firmware: {exc}"
            return

        if firmware:
            for name, result in firmware.items():
                ok = bool(result.get("ok"))
                checks[name] = ok
                details[name] = str(result.get("message", ""))
        else:
            for name in self.FIRMWARE_CHECKS:
                details[name] = "não verificado (firmware sem suporte)"
