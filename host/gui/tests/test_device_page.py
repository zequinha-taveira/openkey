"""Testes da página de dispositivo (descoberta + conexão + auto-refresh)."""

import pytest

from openkey_manager.core.device import DeviceController
from openkey_manager.core.discovery import DiscoveryService
from openkey_manager.core.models import ConnectionState, DeviceCandidate
from openkey_manager.ui.device_page import DevicePage

from test_device import FakeBackend


def _candidate(serial: str):
    return DeviceCandidate(
        vid=0x16C0, pid=0x27DB, serial_number=serial, product_string="OpenKey"
    )


@pytest.fixture
def backend():
    return FakeBackend(devices=[_candidate("123456")])


@pytest.fixture
def page(qtbot, backend):
    controller = DeviceController(backend=backend)
    discovery = DiscoveryService(backend=backend)
    widget = DevicePage(controller, discovery)
    qtbot.addWidget(widget)
    return widget


def test_initial_refresh_populates_list(page):
    page.refresh()
    assert page._list.count() == 1
    assert page._list.item(0).text() == "OpenKey 123456"


def test_attach_updates_list(page, backend):
    page.refresh()
    assert page._list.count() == 1
    backend.devices.append(_candidate("999999"))
    page.refresh()
    assert page._list.count() == 2


def test_connect_populates_info(page):
    page.refresh()
    page._list.setCurrentRow(0)
    page._on_connect()

    controller = page._controller
    assert controller.state == ConnectionState.CONNECTED
    assert controller.info is not None
    assert page._name_label.text() == "OpenKey 123456"
    assert page._aaguid_label.text() == "01" * 16
    assert page._serial_label.text() == "123456"
    assert page._cap_labels["rk"].text() == "sim"
    assert page._disconnect_btn.isEnabled()


def test_disconnect_clears_info(page):
    page.refresh()
    page._list.setCurrentRow(0)
    page._on_connect()
    page._on_disconnect()

    assert page._controller.state == ConnectionState.DISCONNECTED
    assert page._name_label.text() == "Nenhum dispositivo conectado"
    assert page._aaguid_label.text() == "-"
    assert not page._disconnect_btn.isEnabled()


def test_auto_refresh_toggle_starts_timer(page):
    assert page._timer.isActive()
    page._auto_check.setChecked(False)
    assert not page._timer.isActive()
    page._auto_check.setChecked(True)
    assert page._timer.isActive()


def test_connect_button_enabled_on_selection(page):
    assert not page._connect_btn.isEnabled()
    page.refresh()
    page._list.setCurrentRow(0)
    assert page._connect_btn.isEnabled()


def test_selected_device_removed_after_refresh(page, backend):
    page.refresh()
    page._list.setCurrentRow(0)
    backend.devices.clear()
    page.refresh()
    assert page._list.count() == 0
