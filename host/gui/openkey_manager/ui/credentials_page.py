"""Página de credenciais residentes: listar, ver detalhes e remover (G10-T08).

A página usa o ``CredentialService`` (camada core). O PIN é solicitado via
``QInputDialog`` (fluxo dedicado completo chega em G10-T09).
"""

from typing import Optional

from PySide6.QtCore import Qt
from PySide6.QtWidgets import (
    QAbstractItemView,
    QDialog,
    QFormLayout,
    QHBoxLayout,
    QHeaderView,
    QInputDialog,
    QLabel,
    QLineEdit,
    QMessageBox,
    QPushButton,
    QTableWidget,
    QTableWidgetItem,
    QVBoxLayout,
    QWidget,
)

from openkey_manager.core.device import DeviceController
from openkey_manager.core.credentials import CredentialError, CredentialService
from openkey_manager.core.models import Credential

_CREDENTIAL_ROLE = Qt.UserRole
_ID_TRUNC = 20


class CredentialDetailDialog(QDialog):
    """Exibe os detalhes completos de uma credencial residente."""

    def __init__(self, credential: Credential, parent: Optional[QWidget] = None):
        super().__init__(parent)
        self.setWindowTitle("Detalhes da credencial")
        self.setMinimumWidth(420)
        form = QFormLayout(self)
        form.addRow("RP ID:", QLabel(credential.rp_id))
        form.addRow("RP nome:", QLabel(credential.rp_name or "-"))
        form.addRow("Credencial ID:", QLabel(credential.credential_id_hex))
        form.addRow("User ID:", QLabel(_hex(credential.user_id)))
        form.addRow("User name:", QLabel(credential.user_name or "-"))
        form.addRow("Display name:", QLabel(credential.user_display_name or "-"))


def _hex(data) -> str:
    if data is None:
        return "-"
    return data.hex()


def _trunc_hex(data: bytes) -> str:
    hexed = data.hex()
    if len(hexed) <= _ID_TRUNC:
        return hexed
    return f"{hexed[:_ID_TRUNC]}…"


class CredentialsPage(QWidget):
    """Página "Credenciais" do OpenKey Manager."""

    def __init__(
        self,
        controller: DeviceController,
        service: Optional[CredentialService] = None,
        parent: Optional[QWidget] = None,
    ):
        super().__init__(parent)
        self._controller = controller
        self._service = service if service is not None else CredentialService(
            controller, pin_provider=self._ask_pin
        )
        self._credentials: list = []
        self._build_ui()

    # ------------------------------------------------------------------
    # Construção da UI
    # ------------------------------------------------------------------

    def _build_ui(self) -> None:
        root = QVBoxLayout(self)

        toolbar = QHBoxLayout()
        self._refresh_btn = QPushButton("Atualizar")
        self._refresh_btn.clicked.connect(self.refresh)
        self._detail_btn = QPushButton("Ver detalhes")
        self._detail_btn.clicked.connect(self._show_detail)
        self._remove_btn = QPushButton("Remover")
        self._remove_btn.clicked.connect(self._remove_selected)
        toolbar.addWidget(self._refresh_btn)
        toolbar.addWidget(self._detail_btn)
        toolbar.addWidget(self._remove_btn)
        toolbar.addStretch(1)
        root.addLayout(toolbar)

        self._table = QTableWidget(0, 3)
        self._table.setHorizontalHeaderLabels(["RP", "Usuário", "Credencial ID"])
        self._table.setSelectionBehavior(QAbstractItemView.SelectRows)
        self._table.setSelectionMode(QAbstractItemView.SingleSelection)
        self._table.setEditTriggers(QAbstractItemView.NoEditTriggers)
        self._table.horizontalHeader().setSectionResizeMode(
            QHeaderView.Stretch
        )
        self._table.doubleClicked.connect(self._show_detail)
        self._table.itemSelectionChanged.connect(self._update_buttons)
        root.addWidget(self._table, 1)

        self._empty_label = QLabel("Nenhum dispositivo conectado.")
        self._empty_label.setAlignment(Qt.AlignCenter)
        root.addWidget(self._empty_label)
        self._update_buttons()

    # ------------------------------------------------------------------
    # Operações
    # ------------------------------------------------------------------

    def refresh(self) -> None:
        if not self._controller.is_connected:
            self._set_empty("Conecte um dispositivo para listar credenciais.")
            return
        try:
            credentials = self._service.list_credentials()
        except CredentialError as exc:
            self._set_empty(str(exc))
            return
        except Exception as exc:
            self._set_empty(f"Erro ao listar credenciais: {exc}")
            return
        self._populate(credentials)

    def _populate(self, credentials: list) -> None:
        self._credentials = list(credentials)
        self._table.setRowCount(0)
        for credential in self._credentials:
            row = self._table.rowCount()
            self._table.insertRow(row)

            cells = [
                QTableWidgetItem(credential.rp_id),
                QTableWidgetItem(credential.display_name),
                QTableWidgetItem(_trunc_hex(credential.credential_id)),
            ]
            cells[2].setToolTip(credential.credential_id_hex)
            for column, item in enumerate(cells):
                item.setData(_CREDENTIAL_ROLE, credential)
                self._table.setItem(row, column, item)

        self._empty_label.setText(
            "Nenhuma credencial residente." if not credentials else ""
        )
        self._update_buttons()

    def _selected_credential(self) -> Optional[Credential]:
        row = self._table.currentRow()
        if row < 0:
            return None
        item = self._table.item(row, 0)
        if item is None:
            return None
        return item.data(_CREDENTIAL_ROLE)

    def _update_buttons(self) -> None:
        has_selection = self._selected_credential() is not None
        self._detail_btn.setEnabled(has_selection)
        self._remove_btn.setEnabled(has_selection)

    def _show_detail(self) -> None:
        credential = self._selected_credential()
        if credential is None:
            return
        CredentialDetailDialog(credential, self).exec()

    def _remove_selected(self) -> None:
        credential = self._selected_credential()
        if credential is None:
            return
        answer = QMessageBox.question(
            self,
            "Remover credencial",
            f"Remover a credencial de {credential.display_name} "
            f"({credential.rp_id})?",
        )
        if answer != QMessageBox.Yes:
            return
        try:
            self._service.delete_credential(
                credential.credential_id, credential.rp_id
            )
        except Exception as exc:
            QMessageBox.critical(self, "OpenKey Manager", f"Falha ao remover: {exc}")
            return
        self.refresh()

    # ------------------------------------------------------------------
    # PIN / estado
    # ------------------------------------------------------------------

    def _ask_pin(self) -> Optional[str]:
        text, ok = QInputDialog.getText(
            self,
            "PIN",
            "Digite o PIN do dispositivo:",
            QLineEdit.Password,
        )
        return text if ok and text else None

    def _set_empty(self, message: str) -> None:
        self._table.setRowCount(0)
        self._credentials = []
        self._empty_label.setText(message)
        self._update_buttons()
