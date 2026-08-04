# OpenKey Manager (`host/gui/`)

Aplicação desktop gráfica multiplataforma (Windows, macOS, Linux) do OpenKey —
Fase 10 do `Development Plan.md`. Framework: **PySide6** (ver
[ADR-0013](../../docs/reference/adr/ADR-0013-gui-framework.md)).

## Funcionalidades planejadas

- Visualização e remoção de credenciais residentes.
- Configuração/alteração de PIN.
- Diagnóstico de integridade e estatísticas do dispositivo.
- Atualização de firmware com assistente visual.
- Visualização de logs de eventos e pacotes CTAP.
- Ferramenta visual de interoperabilidade (smoke make_credential/get_assertion).

## Estrutura

```
openkey_manager/
├── app.py            # bootstrap (QApplication + MainWindow)
├── __main__.py       # python -m openkey_manager
├── core/             # lógica SEM Qt (testável headless)
│   ├── models.py     # DeviceInfo, Credential, DiagnosticsReport, UpdateSession
│   └── device.py     # DeviceController (descoberta, connect, info, PIN, reset)
└── ui/               # widgets PySide6
    ├── main_window.py  # navegação por páginas (QListWidget + QStackedWidget)
    └── (demais páginas nas próximas tarefas)
```

## Instalação (dev)

Na raiz do monorepo, instale os pacotes locais na ordem de dependência:

```bash
pip install -r requirements-dev.txt
```

O pacote depende do `openkey-sdk` (caminho `host/sdk-python`) e do
`openkey-diagnostics` (caminho `host/diagnostics`) — ambos instalados como
pacotes locais (editable) pelo `requirements-dev.txt`, pois ainda não são
publicados no PyPI. Instalar apenas `pip install -e ".[dev]"` em `host/gui`
falharia enquanto essas dependências não forem publicadas.

## Execução

```bash
python -m openkey_manager
```

## Testes (headless)

```bash
python -m pytest host/gui -q
```

Os testes de widgets usam `QT_QPA_PLATFORM=offscreen` (sem display), garantindo
execução na CI em qualquer SO.
