"""Testes de widgets do OpenKey Manager (PySide6, plataforma offscreen).

A ``QApplication`` é criada automaticamente pelo pytest-qt (fixture ``qtbot``).
"""

import pytest

from openkey_manager.core.device import DeviceController
from openkey_manager.core.discovery import DiscoveryService
from openkey_manager.core.models import ConnectionState, DeviceCandidate
from openkey_manager.ui.device_page import DevicePage
from openkey_manager.ui.main_window import MainWindow, PlaceholderPage

from test_device import FakeBackend


@pytest.fixture
def window(qtbot):
    backend = FakeBackend()
    controller = DeviceController(backend=backend)
    win = MainWindow(controller=controller)
    qtbot.addWidget(win)
    return win


def test_window_title(window):
    assert window.windowTitle() == "OpenKey Manager"


def test_navigation_pages_registered(window):
    # Dispositivo, Credenciais, PIN, Diagnóstico, Atualização, Logs, Interop
    assert window._nav.count() == 7
    assert window._stack.count() == 7
    assert set(window._pages.keys()) == {
        "device",
        "credentials",
        "pin",
        "diagnostics",
        "update",
        "logs",
        "interop",
    }


def test_device_page_replaces_placeholder(window):
    from openkey_manager.ui.credentials_page import CredentialsPage

    assert isinstance(window.page("device"), DevicePage)
    assert isinstance(window.page("credentials"), CredentialsPage)
    assert isinstance(window.page("logs"), PlaceholderPage)


def test_navigate_to_changes_page(window, qtbot):
    window.navigate_to("credentials")
    assert window._stack.currentIndex() == window._nav.currentRow()
    assert window._stack.currentWidget() is window.page("credentials")


def test_register_page_replaces_placeholder(window, qtbot):
    from PySide6.QtWidgets import QLabel

    new_page = QLabel("cred test")
    window.register_page("credentials", "Credenciais", new_page)
    assert window.page("credentials") is new_page
    # mantém a ordem/original sem duplicar
    assert window._stack.count() == 7
    assert window._nav.count() == 7


def test_status_bar_updates_on_state_change(window, qtbot):
    controller = window.controller
    backend = controller._backend
    controller.connect()
    assert "Conectado" in window._status_label.text()

    controller.disconnect()
    assert "Desconectado" in window._status_label.text()


def test_controller_state_change_reflects_info(window, qtbot):
    controller = window.controller
    controller.connect()
    assert controller.info is not None
    assert controller.info.supports_resident_keys
