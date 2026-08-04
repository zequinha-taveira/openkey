"""Bootstrap da aplicação OpenKey Manager (QApplication + MainWindow)."""

import sys
from typing import List, Optional

from PySide6.QtWidgets import QApplication

from openkey_manager.core.device import DeviceController
from openkey_manager.ui.main_window import MainWindow

APP_NAME = "OpenKey Manager"


def create_app(argv: Optional[List[str]] = None) -> QApplication:
    """Cria a ``QApplication`` global (sem exibir a janela)."""
    app = QApplication.instance() or QApplication(argv if argv is not None else [])
    app.setApplicationName(APP_NAME)
    app.setApplicationDisplayName(APP_NAME)
    return app


def create_window(
    app: Optional[QApplication] = None,
    controller: Optional[DeviceController] = None,
) -> MainWindow:
    """Cria a janela principal, garantindo a existência da QApplication."""
    if app is None:
        app = create_app()
    window = MainWindow(controller=controller)
    return window


def main(argv: Optional[List[str]] = None) -> int:
    """Ponto de entrada: cria a aplicação, mostra a janela e roda o loop."""
    app = create_app(argv)
    window = create_window(app=app)
    window.show()
    return app.exec()
