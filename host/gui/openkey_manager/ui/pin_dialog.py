"""Diálogo de PIN do OpenKey Manager (set/change) com padrão de confirmação.

O ``PinDialog`` coleta o PIN atual (apenas no modo ``CHANGE``), o novo PIN e a
confirmação do novo PIN (padrão de confirmação: digitar duas vezes). A
validação é feita localmente antes de fechar o diálogo; a operação em si é
executada pelo ``DeviceController`` (chamador).
"""

from enum import Enum
from typing import Optional

from PySide6.QtWidgets import (
    QDialog,
    QDialogButtonBox,
    QFormLayout,
    QLabel,
    QLineEdit,
    QVBoxLayout,
    QWidget,
)

_PIN_MIN = 4
_PIN_MAX = 63


class PinMode(Enum):
    SET = "set"
    CHANGE = "change"


class PinDialog(QDialog):
    """Solicita o PIN com confirmação.

    Após ``exec()`` retornar ``Accepted``, os PINs estão em
    ``dialog.current_pin`` (só no modo CHANGE) e ``dialog.new_pin``.
    """

    def __init__(self, mode: PinMode = PinMode.SET, parent: Optional[QWidget] = None):
        super().__init__(parent)
        self.mode = mode
        self.current_pin: Optional[str] = None
        self.new_pin: Optional[str] = None
        self.setWindowTitle(
            "Alterar PIN" if mode is PinMode.CHANGE else "Definir PIN"
        )
        self.setMinimumWidth(360)
        self._build_ui()

    # ------------------------------------------------------------------
    # Construção da UI
    # ------------------------------------------------------------------

    def _build_ui(self) -> None:
        layout = QVBoxLayout(self)

        form = QFormLayout()
        self._current_edit: Optional[QLineEdit] = None
        if self.mode is PinMode.CHANGE:
            self._current_edit = QLineEdit()
            self._current_edit.setEchoMode(QLineEdit.Password)
            form.addRow("PIN atual:", self._current_edit)

        self._new_edit = QLineEdit()
        self._new_edit.setEchoMode(QLineEdit.Password)
        form.addRow("Novo PIN:", self._new_edit)

        self._confirm_edit = QLineEdit()
        self._confirm_edit.setEchoMode(QLineEdit.Password)
        form.addRow("Confirme o novo PIN:", self._confirm_edit)
        layout.addLayout(form)

        self._error_label = QLabel("")
        self._error_label.setStyleSheet("color: #c62828;")
        self._error_label.setVisible(False)
        layout.addWidget(self._error_label)

        buttons = QDialogButtonBox(QDialogButtonBox.Ok | QDialogButtonBox.Cancel)
        buttons.accepted.connect(self.accept)
        buttons.rejected.connect(self.reject)
        layout.addWidget(buttons)

    # ------------------------------------------------------------------
    # Validação
    # ------------------------------------------------------------------

    def accept(self) -> None:
        error = self._validate()
        if error:
            self._error_label.setText(error)
            self._error_label.setVisible(True)
            return  # não fecha o diálogo

        self.current_pin = (
            self._current_edit.text() if self._current_edit is not None else None
        )
        self.new_pin = self._new_edit.text()
        super().accept()

    def _validate(self) -> Optional[str]:
        if self.mode is PinMode.CHANGE:
            if self._current_edit is None or not self._current_edit.text():
                return "Digite o PIN atual."

        new_pin = self._new_edit.text()
        if not new_pin:
            return "O novo PIN não pode ser vazio."
        if len(new_pin) < _PIN_MIN:
            return f"O PIN deve ter pelo menos {_PIN_MIN} caracteres."
        if len(new_pin) > _PIN_MAX:
            return f"O PIN deve ter no máximo {_PIN_MAX} caracteres."

        if new_pin != self._confirm_edit.text():
            return "A confirmação não confere com o novo PIN."
        return None
