"""OpenKey Diagnostics: serviço de diagnóstico de dispositivos OpenKey (G10-T10).

Pacote independente do OpenKey Manager (não depende de Qt nem do openkey-sdk).
O serviço consulta o dispositivo através de um *adapter* duck-typed, o que o
torna testável headless e reutilizável por outras ferramentas (CLI, GUI).
"""

from openkey_diagnostics.diagnostics import (
    DiagnosticsError,
    DiagnosticsReport,
    DiagnosticsService,
)

__all__ = [
    "DiagnosticsError",
    "DiagnosticsReport",
    "DiagnosticsService",
]
