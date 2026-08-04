"""Testes da integração do DiagnosticsService com o DeviceController."""

import pytest

from openkey_manager.core.device import DeviceController, DeviceError
from openkey_manager.core.diagnostics import DiagnosticsService
from openkey_manager.core.models import ConnectionState, DiagnosticsReport

from test_device import FakeBackend


def test_run_requires_connection():
    controller = DeviceController(backend=FakeBackend())
    service = DiagnosticsService(controller)
    with pytest.raises(DeviceError):
        service.run()


def test_run_returns_report_from_connected_device():
    backend = FakeBackend()
    controller = DeviceController(backend=backend)
    controller.connect()

    report = DiagnosticsService(controller).run()

    assert isinstance(report, DiagnosticsReport)
    assert report.device_connected is True
    assert report.passed is True
    assert report.checks["getInfo"] is True
    assert report.checks["versions"] is True
    assert report.checks["aaguid"] is True
    assert report.checks["pinRetries"] is True
    assert report.generated_at is not None


def test_run_reports_firmware_unsupported_not_verified():
    backend = FakeBackend()
    controller = DeviceController(backend=backend)
    controller.connect()

    report = DiagnosticsService(controller).run()

    for name in ("flash", "rng", "secrets"):
        assert name not in report.checks
        assert "não verificado" in report.details[name]


def test_run_reports_pin_retries_failure_as_fail():
    backend = FakeBackend()
    backend.pin_fail_retries = RuntimeError("timeout")
    controller = DeviceController(backend=backend)
    controller.connect()

    report = DiagnosticsService(controller).run()

    assert report.checks["pinRetries"] is False
    assert report.passed is False
    assert "timeout" in report.details["pinRetries"]


def test_controller_info_shared():
    backend = FakeBackend()
    controller = DeviceController(backend=backend)
    controller.connect()

    service = DiagnosticsService(controller)
    report = service.run()

    assert service.controller is controller
    assert report.checks["aaguid"] is True
