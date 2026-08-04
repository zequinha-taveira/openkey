"""Página de dispositivo: descoberta, conexão e informações (G10-T07).

Permite listar dispositivos (com *auto-refresh* periódico que detecta
attach/detach), conectar/desconectar e exibir as informações do autenticador
(GetInfo) em um painel de capacidades.
"""

from typing import Optional

from PySide6.QtCore import Qt, QTimer
from PySide6.QtWidgets import (
    QCheckBox,
    QFormLayout,
    QGridLayout,
    QGroupBox,
    QHBoxLayout,
    QLabel,
    QListWidget,
    QListWidgetItem,
    QMessageBox,
    QPushButton,
    QSplitter,
    QVBoxLayout,
    QWidget,
)

from openkey_manager.core.device import DeviceController, DeviceError
from openkey_manager.core.discovery import DiscoveryService
from openkey_manager.core.models import (
    ConnectionState,
    DeviceCandidate,
    DeviceInfo,
)

_AUTO_REFRESH_MS = 2000


class DevicePage(QWidget):
    """Página "Dispositivo" do OpenKey Manager."""

    def __init__(
        self,
        controller: DeviceController,
        discovery: DiscoveryService,
        parent: Optional[QWidget] = None,
    ):
        super().__init__(parent)
        self._controller = controller
        self._discovery = discovery
        self._candidates: dict = {}
        self._build_ui()
        self._discovery.add_listener(self._on_devices_changed)
        self._controller.add_listener(self._on_state_changed)

    # ------------------------------------------------------------------
    # Construção da UI
    # ------------------------------------------------------------------

    def _build_ui(self) -> None:
        root = QVBoxLayout(self)

        toolbar = QHBoxLayout()
        self._refresh_btn = QPushButton("Atualizar")
        self._refresh_btn.clicked.connect(self.refresh)
        self._auto_check = QCheckBox("Auto-atualizar (attach/detach)")
        self._auto_check.setChecked(True)
        toolbar.addWidget(self._refresh_btn)
        toolbar.addWidget(self._auto_check)
        toolbar.addStretch(1)
        root.addLayout(toolbar)

        splitter = QSplitter(Qt.Horizontal)
        splitter.addWidget(self._build_device_list())
        splitter.addWidget(self._build_info_panel())
        splitter.setStretchFactor(0, 1)
        splitter.setStretchFactor(1, 2)
        root.addWidget(splitter, 1)

        actions = QHBoxLayout()
        self._connect_btn = QPushButton("Conectar")
        self._connect_btn.clicked.connect(self._on_connect)
        self._connect_btn.setEnabled(False)
        self._disconnect_btn = QPushButton("Desconectar")
        self._disconnect_btn.clicked.connect(self._on_disconnect)
        self._disconnect_btn.setEnabled(False)
        actions.addWidget(self._connect_btn)
        actions.addWidget(self._disconnect_btn)
        actions.addStretch(1)
        root.addLayout(actions)

        # Auto-refresh periódico para detectar attach/detach
        self._timer = QTimer(self)
        self._timer.setInterval(_AUTO_REFRESH_MS)
        self._timer.timeout.connect(self.refresh)
        self._auto_check.toggled.connect(self._on_auto_refresh_toggled)
        self._timer.start()

    def _build_device_list(self) -> QWidget:
        group = QGroupBox("Dispositivos")
        layout = QVBoxLayout(group)
        self._list = QListWidget()
        self._list.itemSelectionChanged.connect(self._on_selection_changed)
        layout.addWidget(self._list)
        return group

    def _build_info_panel(self) -> QWidget:
        group = QGroupBox("Informações")
        layout = QVBoxLayout(group)

        self._name_label = QLabel("Nenhum dispositivo conectado")
        self._name_label.setStyleSheet("font-weight: bold; font-size: 12pt;")
        layout.addWidget(self._name_label)

        form = QFormLayout()
        self._aaguid_label = QLabel("-")
        self._versions_label = QLabel("-")
        self._serial_label = QLabel("-")
        self._vidpid_label = QLabel("-")
        self._max_msg_label = QLabel("-")
        self._protocols_label = QLabel("-")
        form.addRow("AAGUID:", self._aaguid_label)
        form.addRow("Versões:", self._versions_label)
        form.addRow("Série:", self._serial_label)
        form.addRow("VID:PID:", self._vidpid_label)
        form.addRow("maxMsgSize:", self._max_msg_label)
        form.addRow("pinUvAuthProtocols:", self._protocols_label)
        layout.addLayout(form)

        caps = QGridLayout()
        self._cap_labels = {
            "rk": QLabel("-"),
            "clientPin": QLabel("-"),
            "credentialMgmt": QLabel("-"),
            "uv": QLabel("-"),
            "up": QLabel("-"),
            "plat": QLabel("-"),
        }
        for i, (key, label) in enumerate(self._cap_labels.items()):
            caps.addWidget(QLabel(f"{key}:"), i // 2, (i % 2) * 2)
            caps.addWidget(label, i // 2, (i % 2) * 2 + 1)
        layout.addLayout(caps)
        layout.addStretch(1)
        return group

    # ------------------------------------------------------------------
    # Descoberta
    # ------------------------------------------------------------------

    def refresh(self) -> None:
        try:
            self._discovery.refresh()
        except Exception as exc:
            self._show_error(f"Falha na descoberta: {exc}")

    def _on_auto_refresh_toggled(self, checked: bool) -> None:
        if checked:
            self._timer.start()
        else:
            self._timer.stop()

    def _on_devices_changed(self, attached, detached) -> None:
        self._update_list()

    def _update_list(self) -> None:
        snapshot = self._discovery.snapshot()
        selected_key = self._selected_key()
        self._candidates = {}
        self._list.clear()
        for candidate in snapshot:
            self._candidates[candidate.label] = candidate
            item = QListWidgetItem(candidate.label)
            if selected_key == self._candidate_key(candidate):
                self._list.setCurrentItem(item)
            self._list.addItem(item)

    @staticmethod
    def _candidate_key(candidate: DeviceCandidate) -> str:
        path = candidate.path
        return f"path:{path.hex()}" if path else f"{candidate.serial_number}"

    def _selected_key(self) -> Optional[str]:
        current = self._list.currentItem()
        if current is None:
            return None
        candidate = self._candidates.get(current.text())
        if candidate is None:
            return None
        return self._candidate_key(candidate)

    # ------------------------------------------------------------------
    # Conexão
    # ------------------------------------------------------------------

    def _on_selection_changed(self) -> None:
        self._connect_btn.setEnabled(self._list.currentItem() is not None)

    def _on_connect(self) -> None:
        current = self._list.currentItem()
        if current is None:
            return
        candidate = self._candidates.get(current.text())
        if candidate is None:
            return
        try:
            self._controller.connect(candidate)
        except DeviceError as exc:
            self._show_error(str(exc))

    def _on_disconnect(self) -> None:
        self._controller.disconnect()

    def _on_state_changed(self, state: ConnectionState, message: str) -> None:
        connected = state is ConnectionState.CONNECTED
        self._disconnect_btn.setEnabled(connected)
        if connected:
            self._populate_info(self._controller.info)
        elif state is ConnectionState.DISCONNECTED:
            self._clear_info()

    def _populate_info(self, info: Optional[DeviceInfo]) -> None:
        if info is None:
            self._clear_info()
            return
        self._name_label.setText(info.label)
        self._aaguid_label.setText(info.aaguid_hex)
        self._versions_label.setText(", ".join(info.versions) or "-")
        self._serial_label.setText(info.serial_number or "-")
        self._vidpid_label.setText(
            f"{info.vid:04X}:{info.pid:04X}" if info.vid else "-"
        )
        self._max_msg_label.setText(str(info.max_msg_size))
        self._protocols_label.setText(", ".join(map(str, info.pin_uv_auth_protocols)))
        for key, label in self._cap_labels.items():
            value = info.options.get(key)
            label.setText("sim" if value else ("não" if value is False else "-"))

    def _clear_info(self) -> None:
        self._name_label.setText("Nenhum dispositivo conectado")
        for widget in (
            self._aaguid_label,
            self._versions_label,
            self._serial_label,
            self._vidpid_label,
            self._max_msg_label,
            self._protocols_label,
        ):
            widget.setText("-")
        for label in self._cap_labels.values():
            label.setText("-")

    @staticmethod
    def _show_error(message: str) -> None:
        QMessageBox.critical(None, "OpenKey Manager", message)

    # ------------------------------------------------------------------
    # Cleanup
    # ------------------------------------------------------------------

    def stop(self) -> None:
        """Interrompe o timer de auto-refresh (usado em testes/fechamento)."""
        self._timer.stop()
