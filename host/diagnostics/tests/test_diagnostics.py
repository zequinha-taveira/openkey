"""Testes do OpenKey Diagnostics (serviço headless, sem Qt/SDK)."""

import pytest

from openkey_diagnostics.diagnostics import (
    DiagnosticsReport,
    DiagnosticsService,
)


class FakeInfo:
    aaguid = b"\x01" * 16
    versions = ["FIDO_2_0", "FIDO_2_1"]
    options = {"rk": True, "clientPin": True, "credentialMgmt": True}
    max_msg_size = 1200
    pin_uv_auth_protocols = [1, 2]


class FakeAdapter:
    """Adapter configurável para exercitar os cenários do serviço."""

    def __init__(
        self,
        *,
        info=None,
        fail_info=None,
        retries=5,
        fail_retries=None,
        firmware=None,
        fail_firmware=None,
    ):
        self.info = info if info is not None else FakeInfo()
        self.fail_info = fail_info
        self.retries = retries
        self.fail_retries = fail_retries
        self.firmware = firmware
        self.fail_firmware = fail_firmware

    def get_info(self):
        if self.fail_info:
            raise self.fail_info
        return self.info

    def get_pin_retries(self):
        if self.fail_retries:
            raise self.fail_retries
        return self.retries

    def get_firmware_diagnostics(self):
        if self.fail_firmware:
            raise self.fail_firmware
        return self.firmware


def test_successful_report():
    report = DiagnosticsService(FakeAdapter()).run()

    assert report.device_connected is True
    assert report.passed is True
    assert report.checks["getInfo"] is True
    assert report.checks["versions"] is True
    assert report.checks["aaguid"] is True
    assert report.checks["options"] is True
    assert report.checks["maxMsgSize"] is True
    assert report.checks["pinUvAuthProtocols"] is True
    assert report.checks["pinRetries"] is True
    assert report.failed_checks == 0
    assert report.generated_at is not None


def test_get_info_failure_marks_disconnected():
    report = DiagnosticsService(FakeAdapter(fail_info=RuntimeError("no device"))).run()

    assert report.device_connected is False
    assert report.passed is False
    assert report.checks == {"getInfo": False}
    assert "no device" in report.details["getInfo"]


def test_invalid_aaguid_fails_check():
    info = FakeInfo()
    info.aaguid = b"\x01" * 8
    report = DiagnosticsService(FakeAdapter(info=info)).run()

    assert report.checks["aaguid"] is False
    assert report.passed is False


def test_missing_fido_versions_fails_check():
    info = FakeInfo()
    info.versions = []
    report = DiagnosticsService(FakeAdapter(info=info)).run()

    assert report.checks["versions"] is False
    assert report.passed is False


def test_pin_retries_failure_fails_check():
    report = DiagnosticsService(
        FakeAdapter(fail_retries=RuntimeError("timeout"))
    ).run()

    assert report.checks["pinRetries"] is False
    assert "timeout" in report.details["pinRetries"]
    assert report.passed is False


def test_no_pin_skips_pin_checks():
    info = FakeInfo()
    info.options = {"rk": True}
    report = DiagnosticsService(FakeAdapter(info=info)).run()

    assert "pinUvAuthProtocols" not in report.checks
    assert "pinRetries" not in report.checks
    assert "skip" in report.details["pinUvAuthProtocols"]
    assert report.passed is True


def test_firmware_checks_included_when_supported():
    firmware = {
        "flash": {"ok": True, "message": "flash OK"},
        "rng": {"ok": True, "message": "RNG OK"},
        "secrets": {"ok": False, "message": "secrets falhou"},
    }
    report = DiagnosticsService(FakeAdapter(firmware=firmware)).run()

    assert report.checks["flash"] is True
    assert report.checks["rng"] is True
    assert report.checks["secrets"] is False
    assert report.details["secrets"] == "secrets falhou"
    assert report.passed is False


def test_firmware_unsupported_reports_not_verified():
    report = DiagnosticsService(FakeAdapter(firmware=None)).run()

    assert "flash" not in report.checks
    assert "rng" not in report.checks
    assert "secrets" not in report.checks
    for name in ("flash", "rng", "secrets"):
        assert "não verificado" in report.details[name]
    assert report.passed is True


def test_firmware_query_failure_marks_not_verified():
    report = DiagnosticsService(
        FakeAdapter(fail_firmware=RuntimeError("chan closed"))
    ).run()

    assert "flash" not in report.checks
    assert "falha" in report.details["flash"]
    assert report.passed is True


def test_firmware_version_carried_over():
    info = FakeInfo()
    info.firmware_version = "2.1.0"
    report = DiagnosticsService(FakeAdapter(info=info)).run()

    assert report.firmware_version == "2.1.0"


def test_as_dict_roundtrip():
    report = DiagnosticsService(FakeAdapter()).run()
    data = report.as_dict()

    assert data["device_connected"] is True
    assert data["passed"] is True
    assert data["checks"] == report.checks
    assert data["details"] == report.details
    assert data["generated_at"] == report.generated_at


def test_report_defaults():
    report = DiagnosticsReport()
    assert report.device_connected is False
    assert report.passed is False
    assert report.checks == {}
    assert report.passed_checks == 0
    assert report.failed_checks == 0
