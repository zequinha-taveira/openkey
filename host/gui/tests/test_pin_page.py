"""Testes da página de PIN (fluxo set/change via controller + erros)."""

import pytest

from openkey.exceptions import CtapError

from openkey_manager.core.device import DeviceController
from openkey_manager.ui.pin_dialog import PinDialog, PinMode
from openkey_manager.ui.pin_page import PinPage

from test_device import FakeBackend


@pytest.fixture
def page(qtbot):
    backend = FakeBackend()
    controller = DeviceController(backend=backend)
    widget = PinPage(controller)
    qtbot.addWidget(widget)
    return backend, controller, widget


def _accept_with(pin_attrs):
    def fake_exec(self):
        for key, value in pin_attrs.items():
            setattr(self, key, value)
        return PinDialog.Accepted

    return fake_exec


def test_buttons_disabled_when_disconnected(page):
    backend, controller, widget = page
    assert not widget._set_btn.isEnabled()
    assert not widget._change_btn.isEnabled()


def test_buttons_enabled_when_connected(page):
    backend, controller, widget = page
    controller.connect()
    assert widget._set_btn.isEnabled()
    assert widget._change_btn.isEnabled()


def test_set_pin_flow(page, monkeypatch):
    backend, controller, widget = page
    controller.connect()
    monkeypatch.setattr(PinDialog, "exec", _accept_with({"new_pin": "1234"}))

    widget._on_set_pin()
    assert backend.pin_clients[0].set_pins == ["1234"]
    assert "sucesso" in widget._status_label.text().lower()


def test_change_pin_flow(page, monkeypatch):
    backend, controller, widget = page
    controller.connect()
    monkeypatch.setattr(
        PinDialog,
        "exec",
        _accept_with({"current_pin": "1234", "new_pin": "5678"}),
    )

    widget._on_change_pin()
    assert backend.pin_clients[0].changes == [("1234", "5678")]
    assert "sucesso" in widget._status_label.text().lower()


def test_cancel_does_nothing(page, monkeypatch):
    backend, controller, widget = page
    controller.connect()
    monkeypatch.setattr(PinDialog, "exec", lambda self: PinDialog.Rejected)

    widget._on_set_pin()
    widget._on_change_pin()
    assert backend.pin_clients == []
    assert backend.cm is not None
    assert widget._status_label.text() == ""


def test_wrong_current_pin_shows_retries(page, monkeypatch):
    backend, controller, widget = page
    controller.connect()
    backend.pin_fail_change = CtapError(0x31, "PIN_INVALID")
    monkeypatch.setattr(
        PinDialog,
        "exec",
        _accept_with({"current_pin": "9999", "new_pin": "5678"}),
    )

    widget._on_change_pin()
    assert "PIN incorreto" in widget._status_label.text()
    assert "5 tentativas restantes" in widget._status_label.text()
    assert "#c62828" in widget._status_label.styleSheet()


def test_not_connected_operation_handled(page, monkeypatch):
    backend, controller, widget = page
    monkeypatch.setattr(PinDialog, "exec", _accept_with({"new_pin": "1234"}))
    widget._on_set_pin()
    assert "Falha" in widget._status_label.text()
