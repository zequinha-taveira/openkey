# OpenKey Manager — Arquitetura da GUI (`host/gui/`)

Referência da aplicação desktop do OpenKey (Fase 10). Framework: **PySide6**
(ver [ADR-0013](../adr/ADR-0013-gui-framework.md)).

## Estrutura

```
host/gui/
├── pyproject.toml            # pacote "openkey-manager" (deps: openkey-sdk, openkey-diagnostics, PySide6)
├── requirements-dev.txt      # pytest, pytest-qt
├── openkey_manager/
│   ├── __init__.py
│   ├── __main__.py           # python -m openkey_manager
│   ├── app.py                # bootstrap QApplication + MainWindow
│   ├── core/                 # lógica SEM imports de Qt (testável headless)
│   │   ├── models.py         # DeviceInfo, Credential, DiagnosticsReport, UpdateSession
│   │   ├── device.py         # DeviceController (descoberta, connect, info, PIN, reset)
│   │   ├── discovery.py      # DiscoveryService (snapshot + auto-refresh attach/detach)
│   │   ├── credentials.py    # CredentialService (listar/remover credenciais residentes)
│   │   └── diagnostics.py    # DiagnosticsService (integra host/diagnostics)
│   └── ui/
│       ├── main_window.py    # QMainWindow, navegação por páginas
│       ├── device_page.py    # página Dispositivo (G10-T07)
│       ├── credentials_page.py # página Credenciais (G10-T08)
│       ├── pin_dialog.py     # diálogo de PIN (G10-T09)
│       ├── pin_page.py       # página PIN (G10-T09)
│       └── diagnostics_page.py # página Diagnóstico (G10-T10)
└── tests/                    # test_models, test_device, test_widgets (pytest-qt)
```

## Camada core (sem Qt)

* **`core/models.py`** — dataclasses de domínio: `DeviceCandidate`,
  `DeviceInfo`, `Credential`, `DiagnosticsReport`, `UpdateSession`,
  `ConnectionState`, `UpdateStage`.
* **`core/device.py`** — `DeviceController`: gerencia o ciclo de vida do
  dispositivo (discover → connect → info → reset → disconnect), notifica
  ouvintes (`listener(state, message)`) em cada transição e expõe PIN
  (`get_pin_retries`, `setup_pin`, `change_pin`). A comunicação com o hardware
  passa pelo backend injetável `DeviceBackend` (default: `openkey-sdk`), o que
  permite testar com `FakeBackend`/`FakeDevice`.
* **`core/discovery.py`** — `DiscoveryService`: mantém um snapshot dos
  dispositivos (via `backend.discover()`) e notifica ouvintes
  `(attached, detached)` a cada mudança — base do *auto-refresh* por polling
  (G10-T07).
* **`core/credentials.py`** — `CredentialService`: enumera credenciais
  residentes de todas as RPs (`list_credentials`), remove credenciais
  (`delete_credential`) e traduz os objetos do SDK para o modelo `Credential`.
  O PIN é obtido via `pin_provider` (chamado na primeira operação); o
  `pinUvAuthToken` é efêmero e descartado em `reset_session`.
* **`core/diagnostics.py`** — `DiagnosticsService`: executa o diagnóstico do
  dispositivo conectado delegando ao `host/diagnostics` (pacote
  `openkey-diagnostics`, independente de Qt/SDK). Um adapter converte o
  `DeviceController` na interface duck-typed do serviço (get_info,
  get_pin_retries, get_firmware_diagnostics) e o resultado é mapeado para o
  modelo `DiagnosticsReport`.

O protocolo PIN usa preferencialmente `pinUvAuthProtocol` v2 quando anunciado
em `pinUvAuthProtocols` (fallback v1).

## Serviço de diagnóstico (`host/diagnostics/`)

Pacote independente `openkey-diagnostics` com o `DiagnosticsService` (headless,
sem Qt/SDK). As verificações são executadas sobre um *adapter* duck-typed:

* **`getInfo`** — o dispositivo responde ao `authenticatorGetInfo`
  (conectividade); em caso de falha o dispositivo é marcado como não conectado.
* **`versions`** — anuncia ao menos uma versão `FIDO_2_x`.
* **`aaguid`** — AAGUID válido de 16 bytes.
* **`options`** / **`maxMsgSize`** / **`pinUvAuthProtocols`** / **`pinRetries`**
  — integridade do GetInfo e do subsistema de PIN (quando `clientPin` anunciado).
* **`flash` / `rng` / `secrets`** — integridade de firmware, quando o adapter
  fornecer `get_firmware_diagnostics()`. Sem suporte, são registrados como
  "não verificado" (apenas em `details`) e não contam para o `passed`.

## Camada ui (PySide6)

* **`ui/main_window.py`** — `MainWindow` com navegação lateral
  (`QListWidget`) + `QStackedWidget`. Páginas registradas via
  `register_page(page_id, title, widget)`; nesta fase são placeholders.
  A barra de status reflete `ConnectionState` do `DeviceController`.
* **`ui/device_page.py`** — `DevicePage`: lista de dispositivos com refresh
  manual e *auto-refresh* (QTimer a cada 2s, detectando attach/detach via
  `DiscoveryService`), botões Conectar/Desconectar e painel de capacidades
  (AAGUID, versões, VID:PID, opções rk/clientPin/credentialMgmt/uv/up/plat,
  maxMsgSize, pinUvAuthProtocols).
* **`ui/credentials_page.py`** — `CredentialsPage`: tabela de credenciais
  residentes (RP, usuário, credential ID), botões Atualizar/Ver detalhes/
  Remover (com confirmação), diálogo de detalhes com o hex completo e PIN via
  `QInputDialog` (fluxo completo chega em G10-T09).
* **`ui/pin_dialog.py`** — `PinDialog`: coleta o PIN atual (modo CHANGE), o
  novo PIN e a confirmação (padrão de confirmação). Validação local (tamanho
  4–63, confirmação) antes de fechar.
* **`ui/pin_page.py`** — `PinPage`: botões Definir/Alterar PIN que abrem o
  `PinDialog` e executam via `DeviceController`; exibe erros mapeados
  (PIN incorreto, bloqueado, não definido) com tentativas restantes.
* **`ui/diagnostics_page.py`** — `DiagnosticsPage`: botão "Executar
  diagnóstico" que chama o `DiagnosticsService` (core) e exibe o relatório em
  uma tabela (verificação, resultado OK/FALHA, detalhe), com resumo
  passou/falhou, versão de firmware e marca temporal. Ativada apenas com
  dispositivo conectado.

## Testes (headless)

```powershell
$env:PYTHONPATH = "C:\openkey\host\sdk-python;C:\openkey\host\gui;C:\openkey\host\diagnostics"
python -m pytest host/sdk-python host/gui host/diagnostics -q
```

O `tests/conftest.py` força `QT_QPA_PLATFORM=offscreen`, permitindo execução na
CI sem display. Os testes de core usam `FakeBackend`; os de widget usam
pytest-qt.
