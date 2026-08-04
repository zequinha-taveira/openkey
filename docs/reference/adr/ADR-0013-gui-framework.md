# ADR-0013: Framework GUI e Estrutura do OpenKey Manager Desktop

| Campo     | Valor                                      |
|-----------|--------------------------------------------|
| **ID**    | ADR-0013                                   |
| **Data**  | 2026-07-31                                 |
| **Status**| Proposto                                   |
| **Autores**| OpenKey Team                              |

---

## Contexto

A **Fase 10** do `Development Plan.md` exige uma aplicação desktop gráfica
multiplataforma (Windows, macOS, Linux) — o **OpenKey Manager** — para:

* Visualização gráfica de credenciais residentes (visualizar, listar, remover).
* Alteração/configuração de PIN.
* Diagnóstico de integridade e estatísticas do dispositivo.
* Atualização de firmware com assistente visual.
* Visualização de logs de eventos e pacotes CTAP.
* Ferramenta de testes visuais de interoperabilidade.

O ADR-0010 (item 4) renomeou `host/gui/` → `host/configurator/`, descrevendo
responsabilidade em vez de tecnologia. Na prática, o `host/configurator/`
existente é uma ferramenta CLI simples (sem UI gráfica), e os documentos de
produto (`Ecosystem.md` §4, `Product.md`) continuam a descrever o OpenKey
Manager como `host/gui/`. Não existe nenhum framework GUI declarado no
repositório (única dependência Python é `cbor2` no SDK).

Restrições relevantes:

* Licença do repositório: **Apache-2.0 / MIT dual license**.
* O SDK `host/sdk-python/openkey/` é a camada de comunicação com o dispositivo
  e **não deve ser duplicado** pela GUI.
* Testes devem rodar **headless em CI** (sem display), separando lógica de UI.
* Alvos: Windows, macOS e Linux.

---

## Decisão

### 1. Framework GUI: PySide6 (Qt for Python, LGPLv3)

Adotar **PySide6** como framework oficial da GUI do OpenKey Manager.

| Critério | PySide6 | Tkinter/ttk+customtkinter | PyQt6 | Flet |
|---|---|---|---|---|
| Licença (compat. Apache-2.0/MIT) | LGPL ✔ | BSD (stdlib) ✔ | GPL/comercial ✘ | Apache-2.0 ✔ |
| Widgets nativos (tabela, wizard, chart) | ✔ (QTableWidget, QWizard, QtCharts) | limitado | ✔ | ✔ (via Flutter) |
| Teste headless | `offscreen` + pytest-qt ✔ | ✔ | ✔ | parcial |
| Look "produto pronto para usuário final" | excelente | datado | excelente | moderno |
| Maturidade | alta | alta | alta | média |

Justificativa:

* **PyQt6 descartado** — licença GPL conflita com o dual license do projeto.
* **Tkinter descartado** — custo zero, porém UI datada e sem suporte robusto a
  wizard e tabela avançada; fraco para o objetivo de "GUI pronta para usuário
  final".
* **Flet descartado** — build via Flutter, ecossistema jovem e maior risco de
  dependência para os 3 SO.
* **PySide6** combina licença compatível, widgets nativos de linha (tabela de
  credenciais, `QWizard` para o assistente de update, gráficos de estatísticas)
  e teste headless maduro (`QT_QPA_PLATFORM=offscreen` + `pytest-qt`).

Custo de dependência: `PySide6` (~150 MB instalado) + `openkey-sdk` (novas
dependências `hidapi` e `cryptography` para transporte e PIN) + dev-only
`pytest-qt`.

### 2. Estrutura: reintroduzir `host/gui/`

Reintroduzir o diretório **`host/gui/`** para o OpenKey Manager, amendando o
item 4 do ADR-0010:

* `host/configurator/` permanece como ferramenta CLI legada (não é alterada).
* `host/gui/` é o lar do OpenKey Manager, alinhado a `Ecosystem.md` §4 e
  `Product.md`.

Estrutura proposta:

```
host/gui/
├── pyproject.toml                  # pacote "openkey-manager"; deps: openkey-sdk, PySide6
├── README.md
├── requirements-dev.txt            # pytest, pytest-qt
├── openkey_manager/
│   ├── __init__.py
│   ├── __main__.py                 # python -m openkey_manager
│   ├── app.py                      # bootstrap QApplication + MainWindow
│   ├── core/                       # lógica SEM imports de Qt (testável headless)
│   │   ├── models.py               # DeviceInfo, Credential, DiagnosticsReport, UpdateSession
│   │   ├── device.py               # DeviceController: descoberta, connect, info, PIN, reset
│   │   ├── credentials.py          # CredentialService: listar/ver/remover RK
│   │   ├── diagnostics.py          # DiagnosticsService (integra host/diagnostics)
│   │   ├── update.py               # UpdateService: estado do wizard + FirmwareUpdater
│   │   └── logging.py              # CaptureService (log de eventos/pacotes CTAP)
│   └── ui/
│       ├── main_window.py          # QMainWindow, navegação por páginas
│       ├── device_page.py          # info/capacidades do dispositivo
│       ├── credentials_page.py     # tabela de credenciais residentes
│       ├── pin_dialog.py           # fluxo set/change PIN
│       ├── diagnostics_page.py     # integridade + estatísticas
│       ├── update_wizard.py        # QWizard de atualização de firmware
│       ├── log_viewer.py           # visualizador de logs CTAP
│       └── interop_page.py         # testes visuais de interoperabilidade
└── tests/
    ├── test_device.py              # core, com FakeDevice/mock do SDK
    ├── test_credentials.py
    ├── test_update.py
    └── test_widgets.py             # pytest-qt com QT_QPA_PLATFORM=offscreen
```

### 3. Camada de lógica desacoplada da UI

A camada `core/` não importa PySide. Isso permite:

* Testes unitários headless com `pytest` puro usando `FakeTransport`/`FakeDevice`.
* Testes de widgets com `pytest-qt` + `QT_QPA_PLATFORM=offscreen`.
* Reuso de serviços (`UpdateService`, `DiagnosticsService`) por outras
  ferramentas host.

---

## Consequências

### Positivas

* **GUI de produto**: widgets nativos e wizard para atualização de firmware.
* **Licença compatível**: PySide6 (LGPLv3) não conflita com Apache-2.0/MIT.
* **Testabilidade**: lógica testada headless em CI (3 SO).
* **Reuso**: SDK e `FirmwareUpdater` são integrados, não duplicados.

### Negativas / Trade-offs

* **Custo de dependência**: ~150 MB do PySide6 + novas deps do SDK
  (`hidapi`, `cryptography`).
* **Aumento do SDK**: a GUI expõe gaps que exigirão implementação no
  `host/sdk-python` (transporte USB HID real, ClientPIN, CredentialManagement,
  `make_credential`/`get_assertion`, logging de transporte).
* **CI**: novo job Python será necessário (hoje a CI só roda jobs Rust).

## Alternativas Consideradas

1. **PyQt6** — rejeitado por licença GPL incompatível com o projeto.
2. **Tkinter/ttk** — rejeitado por UI datada e widgets insuficientes.
3. **Flet** — rejeitado por build via Flutter e ecossistema jovem.
4. **Manter apenas CLI** — rejeitado: a Fase 10 exige GUI para usuário final.

## Referências

* [ADR-0001](ADR-0001-rust.md) — Escolha de Rust como linguagem primária
* [ADR-0005](ADR-0005-sdk.md) — Arquitetura do Host SDK
* [ADR-0010](ADR-0010-monorepo-restructure.md) — Reestruturação do Monorepo (amendado no item 4)
* `Development Plan.md` — Fase 10: Aplicação Desktop Graphic Manager
* `Ecosystem.md` §4 — OpenKey Manager (`host/gui`)
* `Product.md` — Ecossistema e roadmap
