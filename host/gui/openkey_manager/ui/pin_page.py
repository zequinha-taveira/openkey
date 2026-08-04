"""Página de PIN do OpenKey Manager: definir e alterar o PIN (G10-T09).

A página abre o ``PinDialog`` (padrão de confirmação), executa a operação via
``DeviceController`` e exibe o resultado/erros (incluindo tentativas restantes
em caso de PIN incorreto).
"""

from typing import Optional

from PySide6.QtWidgets import (
    QHBoxLayout,
    QLabel,
    QPushButton,
    QVBoxLayout,
    QWidget,
)

from openkey.exceptions import CtapError

from openkey_manager.core.device import DeviceController, DeviceError
from openkey_manager.core.models import ConnectionState
from openkey_manager.ui.pin_dialog import PinDialog, PinMode


class PinPage(QWidget):
    """Página "PIN" do OpenKey Manager."""

    def __init__(self, controller: DeviceController, parent: Optional[QWidget] = None):
        super().__init__(parent)
        self._controller = controller
        self._build_ui()
        self._controller.add_listener(self._on_state_changed)

    # ------------------------------------------------------------------
    # Construção da UI
    # ------------------------------------------------------------------

    def _build_ui(self) -> None:
        root = QVBoxLayout(self)

        heading = QLabel("Gerenciamento de PIN")
        heading.setStyleSheet("font-size: 16pt; font-weight: bold;")
        root.addWidget(heading)

        hint = QLabel(
            "Defina o PIN de fábrica ou altere o PIN atual. O novo PIN deve "
            "ser digitado duas vezes (padrão de confirmação)."
        )
        hint.setWordWrap(True)
        root.addWidget(hint)

        actions = QHBoxLayout()
        self._set_btn = QPushButton("Definir PIN")
        self._set_btn.clicked.connect(self._on_set_pin)
        self._change_btn = QPushButton("Alterar PIN")
        self._change_btn.clicked.connect(self._on_change_pin)
        actions.addWidget(self._set_btn)
        actions.addWidget(self._change_btn)
        actions.addStretch(1)
        root.addLayout(actions)

        self._status_label = QLabel("")
        self._status_label.setWordWrap(True)
        root.addWidget(self._status_label)
        root.addStretch(1)

        self._on_state_changed(self._controller.state, "")

    # ------------------------------------------------------------------
    # Operações
    # ------------------------------------------------------------------

    def _on_set_pin(self) -> None:
        dialog = PinDialog(PinMode.SET, self)
        if dialog.exec() != PinDialog.Accepted:
            return
        try:
            self._controller.setup_pin(dialog.new_pin)
        except DeviceError as exc:
            self._set_status(f"Falha: {exc}", error=True)
            return
        except CtapError as exc:
            self._on_ctap_error(exc)
            return
        self._set_status("PIN definido com sucesso.")

    def _on_change_pin(self) -> None:
        dialog = PinDialog(PinMode.CHANGE, self)
        if dialog.exec() != PinDialog.Accepted:
            return
        try:
            self._controller.change_pin(dialog.current_pin, dialog.new_pin)
        except DeviceError as exc:
            self._set_status(f"Falha: {exc}", error=True)
            return
        except CtapError as exc:
            self._on_ctap_error(exc)
            return
        self._set_status("PIN alterado com sucesso.")

    # ------------------------------------------------------------------
    # Estado / feedback
    # ------------------------------------------------------------------

    def _on_ctap_error(self, exc: CtapError) -> None:
        message = self._describe_error(exc)
        retries = self._safe_retries()
        if retries is not None:
            message += f" ({retries} tentativa{'s' if retries != 1 else ''} restante{'s' if retries != 1 else ''})"
        self._set_status(message, error=True)

    @staticmethod
    def _describe_error(exc: CtapError) -> str:
        code = getattr(exc, "status_code", None)
        messages = {
            0x31: "PIN incorreto.",
            0x30: "Autenticação de PIN inválida.",
            0x32: "PIN ainda não definido.",
            0x37: "Dispositivo bloqueado por excesso de tentativas.",
        }
        if code in messages:
            return messages[code]
        return f"Falha ao operar o PIN: {exc}"

    def _safe_retries(self) -> Optional[int]:
        try:
            return self._controller.get_pin_retries()
        except Exception:
            return None

    def _on_state_changed(self, state: ConnectionState, message: str) -> None:
        connected = state is ConnectionState.CONNECTED
        self._set_btn.setEnabled(connected)
        self._change_btn.setEnabled(connected)

    def _set_status(self, message: str, error: bool = False) -> None:
        self._status_label.setText(message)
        self._status_label.setStyleSheet(
            f"color: {'#c62828' if error else '#2e7d32'}; font-weight: bold;"
        )
