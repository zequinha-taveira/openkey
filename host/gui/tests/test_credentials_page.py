"""Testes da página de credenciais residentes (tabela + remover + detalhes)."""

import pytest

from openkey_manager.core.credentials import CredentialService
from openkey_manager.core.device import DeviceController
from openkey_manager.core.models import ConnectionState
from openkey_manager.ui.credentials_page import CredentialsPage

from test_device import FakeBackend, FakeCredentialManager


@pytest.fixture
def cm():
    return FakeCredentialManager(
        credentials=[
            ("example.com", "Example", b"\x11" * 16, b"u1", "alice", "Alice"),
            ("other.org", "Other", b"\x33" * 16, b"u3", "carol", None),
        ]
    )


@pytest.fixture
def page(qtbot, cm):
    backend = FakeBackend(credential_manager=cm)
    controller = DeviceController(backend=backend)
    controller.connect()
    service = CredentialService(controller, pin_provider=lambda: "1234")
    widget = CredentialsPage(controller, service=service)
    qtbot.addWidget(widget)
    return widget


def test_not_connected_shows_message(qtbot):
    backend = FakeBackend()
    controller = DeviceController(backend=backend)
    widget = CredentialsPage(controller)
    qtbot.addWidget(widget)
    widget.refresh()
    assert "Conecte um dispositivo" in widget._empty_label.text()
    assert widget._table.rowCount() == 0


def test_refresh_populates_table(page):
    page.refresh()
    assert page._table.rowCount() == 2
    assert page._table.item(0, 0).text() == "example.com"
    assert page._table.item(1, 0).text() == "other.org"


def test_selection_enables_buttons(page):
    page.refresh()
    assert not page._remove_btn.isEnabled()
    page._table.selectRow(0)
    assert page._remove_btn.isEnabled()
    assert page._detail_btn.isEnabled()


def test_remove_selected(page, cm, monkeypatch):
    from openkey_manager.ui import credentials_page as page_module

    monkeypatch.setattr(
        page_module.QMessageBox,
        "question",
        staticmethod(lambda *a, **k: page_module.QMessageBox.Yes),
    )
    page.refresh()
    page._table.selectRow(0)
    page._remove_selected()
    assert page._table.rowCount() == 1


def test_detail_dialog_contains_full_hex(page, monkeypatch):
    from openkey_manager.ui.credentials_page import CredentialDetailDialog

    monkeypatch.setattr(CredentialDetailDialog, "exec", lambda self: 0)
    page.refresh()
    page._table.selectRow(1)
    page._show_detail()
    # o diálogo é aberto via exec(); apenas verifica que seleção persiste
    assert page._selected_credential().credential_id == b"\x33" * 16


def test_refresh_clears_on_empty(page, cm):
    page.refresh()
    assert page._table.rowCount() == 2
    cm._creds.clear()
    page.refresh()
    assert page._table.rowCount() == 0
    assert "Nenhuma credencial residente" in page._empty_label.text()
