"""Janela principal do OpenKey Manager com navegação por páginas.

A navegação usa uma lista lateral (``QListWidget``) empilhando as páginas num
``QStackedWidget``. As páginas reais (dispositivo, credenciais, PIN, etc.) são
registradas via ``register_page``; nesta fase (G10-T06) as páginas são
placeholders exibindo o título e o estado de conexão.
"""

from typing import Dict, List, Optional, Tuple

from PySide6.QtCore import Qt
from PySide6.QtWidgets import (
    QHBoxLayout,
    QLabel,
    QListWidget,
    QListWidgetItem,
    QMainWindow,
    QStackedWidget,
    QVBoxLayout,
    QWidget,
)

from openkey_manager.core.device import DeviceController
from openkey_manager.core.discovery import DiscoveryService
from openkey_manager.core.models import ConnectionState
from openkey_manager.ui.credentials_page import CredentialsPage
from openkey_manager.ui.device_page import DevicePage
from openkey_manager.ui.diagnostics_page import DiagnosticsPage
from openkey_manager.ui.pin_page import PinPage

_PAGE_IDS = [
    ("device", "Dispositivo"),
    ("credentials", "Credenciais"),
    ("pin", "PIN"),
    ("diagnostics", "Diagnóstico"),
    ("update", "Atualização"),
    ("logs", "Logs"),
    ("interop", "Interoperabilidade"),
]


class PlaceholderPage(QWidget):
    """Página temporária até a implementação da funcionalidade (G10-T07+)."""

    def __init__(self, title: str, parent: Optional[QWidget] = None):
        super().__init__(parent)
        layout = QVBoxLayout(self)
        heading = QLabel(title)
        heading.setObjectName("pageTitle")
        heading.setAlignment(Qt.AlignLeft | Qt.AlignTop)
        heading.setStyleSheet("font-size: 20pt; font-weight: bold; padding: 12px;")
        hint = QLabel("Página em desenvolvimento (próximas tarefas da Fase 10).")
        hint.setAlignment(Qt.AlignCenter)
        layout.addWidget(heading)
        layout.addStretch(1)
        layout.addWidget(hint)
        layout.addStretch(1)


class MainWindow(QMainWindow):
    """Janela principal com navegação lateral e barra de status."""

    def __init__(
        self,
        controller: Optional[DeviceController] = None,
        discovery: Optional[DiscoveryService] = None,
        parent=None,
    ):
        super().__init__(parent)
        self._controller = controller if controller is not None else DeviceController()
        self._discovery = (
            discovery if discovery is not None else DiscoveryService(self._controller.backend)
        )
        self._pages: Dict[str, QWidget] = {}
        self._items: Dict[str, QListWidgetItem] = {}
        self.setWindowTitle("OpenKey Manager")
        self.resize(900, 600)

        central = QWidget(self)
        root = QHBoxLayout(central)
        root.setContentsMargins(0, 0, 0, 0)

        self._nav = QListWidget()
        self._nav.setFixedWidth(180)
        self._nav.currentRowChanged.connect(self._on_nav_changed)
        root.addWidget(self._nav)

        self._stack = QStackedWidget()
        root.addWidget(self._stack, 1)
        self.setCentralWidget(central)

        for page_id, title in _PAGE_IDS:
            self.register_page(page_id, title, PlaceholderPage(title))

        self.register_page(
            "device", "Dispositivo", DevicePage(self._controller, self._discovery)
        )
        self.register_page(
            "credentials",
            "Credenciais",
            CredentialsPage(self._controller),
        )
        self.register_page(
            "pin", "PIN", PinPage(self._controller)
        )
        self.register_page(
            "diagnostics", "Diagnóstico", DiagnosticsPage(self._controller)
        )

        self._nav.setCurrentRow(0)
        self._setup_status_bar()
        self._controller.add_listener(self._on_state_changed)

    # ------------------------------------------------------------------
    # Navegação
    # ------------------------------------------------------------------

    def register_page(self, page_id: str, title: str, widget: QWidget) -> None:
        """Registra (ou substitui) uma página no stack de navegação."""
        if page_id in self._pages:
            old = self._pages[page_id]
            index = self._stack.indexOf(old)
            self._stack.removeWidget(old)
            self._stack.insertWidget(index, widget)
            self._pages[page_id] = widget
            return

        item = QListWidgetItem(title)
        self._nav.addItem(item)
        self._stack.addWidget(widget)
        self._pages[page_id] = widget
        self._items[page_id] = item

    def page(self, page_id: str) -> Optional[QWidget]:
        return self._pages.get(page_id)

    def navigate_to(self, page_id: str) -> None:
        item = self._items.get(page_id)
        if item is not None:
            self._nav.setCurrentItem(item)

    def _on_nav_changed(self, row: int) -> None:
        self._stack.setCurrentIndex(row)

    # ------------------------------------------------------------------
    # Status / estado
    # ------------------------------------------------------------------

    def _setup_status_bar(self) -> None:
        self._status_label = QLabel("Desconectado")
        self.statusBar().addWidget(self._status_label)

    def _on_state_changed(self, state: ConnectionState, message: str) -> None:
        self._status_label.setText(message or state.value)
        color = {
            ConnectionState.CONNECTED: "#2e7d32",
            ConnectionState.CONNECTING: "#f9a825",
            ConnectionState.ERROR: "#c62828",
            ConnectionState.DISCONNECTED: "#616161",
        }.get(state, "#616161")
        self._status_label.setStyleSheet(f"color: {color}; padding: 0 8px;")

    @property
    def controller(self) -> DeviceController:
        return self._controller

    def closeEvent(self, event) -> None:
        device_page = self._pages.get("device")
        if isinstance(device_page, DevicePage):
            device_page.stop()
        super().closeEvent(event)
