"""Página de Diagnóstico do OpenKey Manager (G10-T10).

Executa o ``DiagnosticsService`` (host/diagnostics) contra o dispositivo
conectado e exibe o relatório: checks de integridade/conformidade, detalhes,
versão de firmware e marca temporal.
"""

from typing import Optional

from PySide6.QtCore import Qt
from PySide6.QtWidgets import (
    QHBoxLayout,
    QHeaderView,
    QLabel,
    QPushButton,
    QTableWidget,
    QTableWidgetItem,
    QVBoxLayout,
    QWidget,
)

from openkey_manager.core.device import DeviceController, DeviceError
from openkey_manager.core.diagnostics import DiagnosticsService
from openkey_manager.core.models import ConnectionState, DiagnosticsReport


class DiagnosticsPage(QWidget):
    """Página "Diagnóstico" do OpenKey Manager."""

    def __init__(
        self,
        controller: DeviceController,
        service: Optional[DiagnosticsService] = None,
        parent: Optional[QWidget] = None,
    ):
        super().__init__(parent)
        self._controller = controller
        self._service = service if service is not None else DiagnosticsService(controller)
        self._build_ui()
        self._controller.add_listener(self._on_state_changed)

    # ------------------------------------------------------------------
    # Construção da UI
    # ------------------------------------------------------------------

    def _build_ui(self) -> None:
        root = QVBoxLayout(self)

        heading = QLabel("Diagnóstico do dispositivo")
        heading.setStyleSheet("font-size: 16pt; font-weight: bold;")
        root.addWidget(heading)

        hint = QLabel(
            "Executa verificações de integridade e conformidade FIDO2/CTAP2 "
            "contra o dispositivo conectado."
        )
        hint.setWordWrap(True)
        root.addWidget(hint)

        actions = QHBoxLayout()
        self._run_btn = QPushButton("Executar diagnóstico")
        self._run_btn.clicked.connect(self._on_run)
        actions.addWidget(self._run_btn)
        actions.addStretch(1)
        root.addLayout(actions)

        self._summary_label = QLabel("Nenhum dispositivo conectado.")
        self._summary_label.setWordWrap(True)
        root.addWidget(self._summary_label)

        self._table = QTableWidget(0, 3)
        self._table.setHorizontalHeaderLabels(["Verificação", "Resultado", "Detalhe"])
        header = self._table.horizontalHeader()
        header.setSectionResizeMode(0, QHeaderView.ResizeToContents)
        header.setSectionResizeMode(1, QHeaderView.ResizeToContents)
        header.setSectionResizeMode(2, QHeaderView.Stretch)
        self._table.setEditTriggers(QTableWidget.NoEditTriggers)
        self._table.setSelectionMode(QTableWidget.NoSelection)
        self._table.verticalHeader().setVisible(False)
        root.addWidget(self._table, 1)

        self._meta_label = QLabel("")
        root.addWidget(self._meta_label)

        self._on_state_changed(self._controller.state, "")

    # ------------------------------------------------------------------
    # Execução
    # ------------------------------------------------------------------

    def _on_run(self) -> None:
        try:
            report = self._service.run()
        except DeviceError as exc:
            self._set_summary(f"Falha: {exc}", error=True)
            return
        except Exception as exc:
            self._set_summary(f"Falha ao executar diagnóstico: {exc}", error=True)
            return
        self._populate(report)

    def _populate(self, report: DiagnosticsReport) -> None:
        if not report.device_connected:
            self._set_summary("Dispositivo não conectado.", error=True)
            self._table.setRowCount(0)
            self._meta_label.setText("")
            return

        if report.passed:
            self._set_summary(
                f"Diagnóstico OK: {report.passed_checks} verificação(ões) "
                f"passaram, {report.failed_checks} falharam."
            )
        else:
            self._set_summary(
                f"Problemas encontrados: {report.failed_checks} verificação(ões) "
                f"falharam de {report.passed_checks + report.failed_checks}.",
                error=True,
            )

        rows = []
        for name, ok in report.checks.items():
            rows.append((name, "OK" if ok else "FALHA", report.details.get(name, "")))
        for name in report.details:
            if name not in report.checks:
                rows.append((name, "—", report.details[name]))

        self._table.setRowCount(len(rows))
        for row, (name, result, detail) in enumerate(rows):
            name_item = QTableWidgetItem(name)
            result_item = QTableWidgetItem(result)
            detail_item = QTableWidgetItem(detail)
            if result == "FALHA":
                result_item.setForeground(Qt.red)
            elif result == "OK":
                result_item.setForeground(Qt.darkGreen)
            self._table.setItem(row, 0, name_item)
            self._table.setItem(row, 1, result_item)
            self._table.setItem(row, 2, detail_item)

        meta = []
        if report.firmware_version:
            meta.append(f"Firmware: {report.firmware_version}")
        if report.generated_at:
            meta.append(f"Gerado em: {report.generated_at}")
        self._meta_label.setText("  |  ".join(meta))

    # ------------------------------------------------------------------
    # Estado / feedback
    # ------------------------------------------------------------------

    def _set_summary(self, message: str, error: bool = False) -> None:
        self._summary_label.setText(message)
        self._summary_label.setStyleSheet(
            f"color: {'#c62828' if error else '#2e7d32'}; font-weight: bold;"
        )

    def _on_state_changed(self, state: ConnectionState, message: str) -> None:
        connected = state is ConnectionState.CONNECTED
        self._run_btn.setEnabled(connected)
        if connected:
            self._summary_label.setText("Dispositivo conectado. Execute o diagnóstico.")
            self._summary_label.setStyleSheet("color: #2e7d32;")
        else:
            self._set_summary("Nenhum dispositivo conectado.", error=False)
