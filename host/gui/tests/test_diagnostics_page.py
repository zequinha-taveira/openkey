"""Testes da página de diagnóstico (execução + exibição do relatório)."""

import pytest

from openkey_manager.core.device import DeviceController
from openkey_manager.core.diagnostics import DiagnosticsService
from openkey_manager.core.models import ConnectionState, DiagnosticsReport
from openkey_manager.ui.diagnostics_page import DiagnosticsPage

from test_device import FakeBackend


@pytest.fixture
def page(qtbot):
    backend = FakeBackend()
    controller = DeviceController(backend=backend)
    widget = DiagnosticsPage(controller)
    qtbot.addWidget(widget)
    return backend, controller, widget


def test_run_button_disabled_when_disconnected(page):
    backend, controller, widget = page
    assert not widget._run_btn.isEnabled()


def test_run_button_enabled_when_connected(page):
    backend, controller, widget = page
    controller.connect()
    assert widget._run_btn.isEnabled()


def test_run_populates_table(page):
    backend, controller, widget = page
    controller.connect()

    widget._on_run()

    assert widget._table.rowCount() >= 4
    assert "OK" in widget._summary_label.text()
    assert widget._meta_label.text() != ""


def test_run_not_connected_shows_error(page):
    backend, controller, widget = page
    widget._on_run()
    assert "Falha" in widget._summary_label.text()
    assert widget._table.rowCount() == 0


def test_failed_check_shows_failure_row(page, monkeypatch):
    backend, controller, widget = page
    controller.connect()
    backend.pin_fail_retries = RuntimeError("boom")

    widget._on_run()

    assert "falharam" in widget._summary_label.text()
    assert "#c62828" in widget._summary_label.styleSheet()

    rows = widget._table.rowCount()
    results = [widget._table.item(r, 1).text() for r in range(rows)]
    assert "FALHA" in results


def test_skipped_firmware_checks_shown(page):
    backend, controller, widget = page
    controller.connect()

    widget._on_run()

    rows = widget._table.rowCount()
    names = [widget._table.item(r, 0).text() for r in range(rows)]
    assert "flash" in names
    assert "rng" in names
    assert "secrets" in names


def test_disconnect_resets_page(page):
    backend, controller, widget = page
    controller.connect()
    widget._on_run()
    assert widget._table.rowCount() > 0

    controller.disconnect()
    assert not widget._run_btn.isEnabled()
    assert "Nenhum dispositivo" in widget._summary_label.text()


def test_run_with_injected_service_uses_it(qtbot, monkeypatch):
    from openkey_manager.ui import diagnostics_page as page_module

    backend = FakeBackend()
    controller = DeviceController(backend=backend)
    controller.connect()

    captured = {}

    class FakeService:
        def run(self):
            captured["called"] = True
            return DiagnosticsReport(
                device_connected=True,
                checks={"custom": True},
                details={"custom": "ok"},
                passed=True,
                generated_at="2026-01-01T00:00:00+00:00",
            )

    widget = DiagnosticsPage(controller, service=FakeService())
    qtbot.addWidget(widget)
    widget._on_run()

    assert captured.get("called") is True
    assert widget._table.rowCount() == 1
    assert widget._table.item(0, 0).text() == "custom"
    assert widget._table.item(0, 1).text() == "OK"
