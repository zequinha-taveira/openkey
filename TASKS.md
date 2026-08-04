# OpenKey — Task Tracking

> Este documento rastreia as tarefas ativas, pendentes e concluídas pelo projeto OpenKey.
>
> As tarefas são organizadas por fase (PAR) e prioridade. Cada tarefa deve ter um responsável,
> uma data de início prevista e um status.

---

## 📊 Status Geral

| PAR | Fase | Status | Progresso |
|-----|------|--------|-----------|
| PAR-01 | Foundation | ✅ Concluída | 100% |
| PAR-02 | Architecture | ✅ Concluída | 100% |
| PAR-03 | Platform | ✅ Concluída | 100% |
| PAR-04 | Security | ✅ Concluída | 100% |
| PAR-05 | Protocols | ✅ Concluída | 100% |
| PAR-06 | Host Tools | ✅ Concluída | 100% |
| PAR-07 | Validation | ✅ Concluída | 100% |
| PAR-08 | Release | ✅ Concluída | 100% |
| Fase 10 | Desktop GUI (OpenKey Manager) | 📋 Planejada | 0% |

---

## 🖥️ Fase 10 — Desktop GUI (OpenKey Manager)

Fase do `Development Plan.md` que implementa o **OpenKey Manager**, aplicação
desktop gráfica multiplataforma (Windows, macOS, Linux). Framework: **PySide6**
(ADR-0013). Estrutura: `host/gui/` com camada `core/` (sem Qt, testável
headless) + `ui/` (PySide6). Depende de gaps do `host/sdk-python`.

### Prioridade Alta

- [ ] **G10-T01**: Criar ADR-0013 (framework GUI + resolução de layout) e atualizar documentação
- [x] **G10-T02**: SDK — backend USB HID (hidapi) + descoberta de dispositivos + testes
- [x] **G10-T03**: SDK — protocolo ClientPIN (set_pin, change_pin, get_pin_token) + testes
- [x] **G10-T04**: SDK — CredentialManagement (enumerate_rps, enumerate_credentials, delete_credential) + testes
- [x] **G10-T05**: SDK — make_credential / get_assertion + hook de logging de pacotes CTAP + testes
- [x] **G10-T06**: Scaffold da GUI — pyproject, bootstrap, MainWindow/navegação, core/models.py + DeviceController + testes
- [x] **G10-T07**: Página de dispositivo + descoberta + auto-refresh (attach/detach)
- [x] **G10-T08**: Página de credenciais residentes (listar/ver/remover) + testes
- [x] **G10-T09**: Diálogo de PIN (set/change) + padrão de confirmação + testes
- [x] **G10-T10**: Serviço de diagnósticos (em `host/diagnostics/`) + página + testes
- [ ] **G10-T11**: Assistente visual de atualização de firmware (reuso FirmwareUpdater) + UpdateService + testes

### Prioridade Média

- [ ] **G10-T12**: Visualizador de logs de eventos e pacotes CTAP (captura via hook)
- [ ] **G10-T13**: Ferramenta visual de interoperabilidade (smoke make_credential/get_assertion)
- [ ] **G10-T14**: Fluxo de reset de fábrica com confirmação explícita
- [ ] **G10-T15**: Job Python na CI (matrix Ubuntu/Windows/macOS, headless)

### Prioridade Baixa

- [ ] **G10-T16**: Instaladores nativos em `packaging/` + README + docs + gate da Fase 10

---

## 🔧 PAR-04 — Security (Em Andamento)

### Prioridade Alta

- [x] **SEC-001**: Implementar Secure Boot API no crate `firmware/boot`
  - Sub-tasks:
    - [x] Definir trait `SecureBootProvider` em `firmware/boot/src/`
    - [x] Implementar verificação de assinatura ECDSA P-256 (conforme ADR-0008)
    - [x] Implementar bootloader dual-bank com rollback
    - [x] Adicionar testes unitários no simulador

- [x] **SEC-002**: Implementar Secure Storage
  - Sub-tasks:
    - [x] Implementar wear-leveling circular no crate `firmware/storage`
    - [x] Integrar AES-256-GCM para dados sensíveis (conforme ADR-0002)
    - [x] Implementar power-loss recovery (estado `Writing` + `recover_power_loss`)
    - [x] Adicionar testes de integridade (corruption detection, CRNGT)

- [x] **SEC-003**: Implementar Key Management
  - Sub-tasks:
    - [x] Definir trait `AttestationKeyProvider` para chaves de atestação
    - [x] Implementar geração de pares de chaves P-256 / Ed25519
    - [x] Implementar zeroização de chaves efêmeras (via `Drop`)
    - [x] Adicionar testes de zeroização e sign/verify

- [x] **SEC-004**: Implementar OTP Interface
  - Sub-tasks:
    - [x] Definir trait `OtpProvider` no HAL (`hal/otp.rs`)
    - [x] Implementar interface para OTP (One-Time Programmable) memory
    - [x] Implementar leitura de chaves de atestação únicas

- [x] **SEC-005**: Implementar Device Identity
  - Sub-tasks:
    - [x] Definir estrutura `DeviceIdentity` em `firmware/platform/src/identity.rs`
    - [x] Implementar AAGUID (Authenticator Attestation GUID)
    - [x] Implementar validação de identidade no boot
    - [x] Adicionar testes de validação

### Prioridade Média

- [x] **SEC-006**: Revisão de segurança do código `unsafe`
  - Sub-tasks:
    - [x] Auditar todos os blocos `unsafe` existentes (nenhum encontrado — 100% safe Rust)
    - [x] Verificar comentários `// SAFETY:` em todos os blocos
    - [x] Configurar Miri no CI (`.github/workflows/ci.yml` — job `miri-check`)
    - [x] Documentar resultados da auditoria

- [x] **SEC-007**: Implementar TRNG health checks
  - Sub-tasks:
    - [x] Implementar testes NIST SP 800-90B no `RngProvider` (`hal/rng.rs`)
    - [x] Adicionar validação contínua de entropia (Monobit, Poker, Runs, CRNGT)
    - [x] Implementar fallback para RNG software (apenas em simulador)

---

## 🔧 PAR-05 — Protocols (Concluída)

### Prioridade Alta

- [x] **PROTO-001**: Implementar CBOR parser/serializer canônico
  - Sub-tasks:
    - [x] Implementar parser CBOR estático (sem alocação heap) em `protocol/protocols/src/cbor/decoder.rs`
    - [x] Implementar validação de canonicidade (RFC 8949) em `protocol/protocols/src/cbor/`
    - [x] Adicionar testes unitários e de borda para validação canônica

- [x] **PROTO-002**: Implementar COSE
  - Sub-tasks:
    - [x] Implementar estrutura COSE Sign1 (RFC 9052) em `protocol/protocols/src/cose/mod.rs`
    - [x] Integrar com crypto (ECDSA P-256 / Ed25519) e Sig_structure
    - [x] Adicionar testes de encodagem e parsing roundtrip

- [x] **PROTO-003**: Implementar CTAP HID
  - Sub-tasks:
    - [x] Implementar framing CTAPHID (conforme ADR-0003) em `protocol/protocols/src/ctap_hid/mod.rs`
    - [x] Implementar gerenciamento de canais e sequenciamento de pacotes
    - [x] Implementar comandos: INIT, PING, MSG, CANCEL, ERROR
    - [x] Adicionar reassembly e testes de mensagens multi-pacote

- [x] **PROTO-004**: Implementar CTAP2
  - Sub-tasks:
    - [x] Implementar `authenticatorGetInfo` em `protocol/protocols/src/ctap2/get_info.rs`
    - [x] Implementar estruturas de resposta e códigos de status em `protocol/protocols/src/ctap2/status.rs`
    - [x] Implementar engine de comandos `Ctap2Engine` em `protocol/protocols/src/ctap2/mod.rs`

- [x] **PROTO-005**: Implementar WebAuthn
  - Sub-tasks:
    - [x] Implementar serialização de `AuthenticatorData` (`authData`)
    - [x] Implementar codificação de `PublicKeyCredential` (COSE Key P-256)
    - [x] Implementar suporte a `rpIdHash` e flags (`UP`, `UV`, `AT`)
    - [x] Adicionar testes unitários em `protocol/protocols/src/webauthn/mod.rs`

---

## 🔧 PAR-06 — Host Tools (Concluída)

### Prioridade Alta

- [x] **HOST-001**: Implementar Python SDK
  - Sub-tasks:
    - [x] Implementar descoberta de dispositivos USB HID e mock em `host/sdk-python/openkey/client.py`
    - [x] Implementar comunicação CTAPHID em `host/sdk-python/openkey/transport.py`
    - [x] Implementar APIs de gerenciamento de credenciais e CTAP2 em `host/sdk-python/openkey/ctap2.py`
    - [x] Adicionar testes unitários do SDK em `host/sdk-python/tests/test_sdk.py`

- [x] **HOST-002**: Implementar CLI
  - Sub-tasks:
    - [x] Implementar subcomandos: info, pin, credentials, reset, update em `host/cli/openkey_cli.py`
    - [x] Integrar com Python SDK (`OpenKeyDevice`)
    - [x] Adicionar argumentos e tratamento de erros

- [x] **HOST-003**: Implementar Configurator
  - Sub-tasks:
    - [x] Implementar interface interativa/CLI em `host/configurator/configurator.py`
    - [x] Integrar com Python SDK
    - [x] Adicionar leitura de diagnósticos e opções de aplicativo

- [x] **HOST-004**: Implementar Provisioner
  - Sub-tasks:
    - [x] Implementar provisionamento de fábrica em `host/provisioner/provisioner.py`
    - [x] Implementar injeção de AAGUID e chaves de atestação
    - [x] Adicionar validação de transição de estado de provisionamento

- [x] **HOST-005**: Implementar Updater
  - Sub-tasks:
    - [x] Implementar atualização de firmware via USB em `host/updater/updater.py`
    - [x] Implementar verificação de assinatura ECDSA P-256
    - [x] Adicionar validação de imagem dual-bank

---

## 🔧 PAR-07 — Validation (Concluída)

- [x] **VAL-001**: Testes unitários (55 testes unitários em 11 crates do workspace Rust passando)
- [x] **VAL-002**: Testes de integração (`tests/integration_test.rs` validando fluxo CTAPHID -> CBOR -> CTAP2 Engine)
- [x] **VAL-003**: Testes de hardware (`simulator` e compilação `no_std` para o target RP2350)
- [x] **VAL-004**: Testes de interoperabilidade (Conformidade WebAuthn `authData`, COSE Key P-256 e CBOR canônico)
- [x] **VAL-005**: Testes de regressão (Resiliência de storage em falhas de escrita e saúde contínua do TRNG SP 800-90B)

---

## 🔧 PAR-08 — Release (Concluída)

- [x] **REL-001**: Preparar CHANGELOG (`CHANGELOG.md` atualizado com versão `1.0.0`)
- [x] **REL-002**: Preparar Release Notes (`VERSION_HISTORY.md` e notas de lançamento v1.0.0)
- [x] **REL-003**: Criar pacotes de distribuição (Cargo release, firmware binário RP2350, Python SDK)
- [x] **REL-004**: Finalizar documentação (Sincronização 100% dos documentos `spec.md`, `README.md`, `PHASES.md`, `TASKS.md`)
- [x] **REL-005**: Criar tag de versão (Tag Git `v1.0.0` gerada)

---

## 📝 Convenções

### Formato de ID de Tarefa

```
<PAR-04>-<TIPO>-<NUMERO>
```

Exemplo: `PAR-04-SEC-001`

### Status

| Status | Descrição |
|--------|-----------|
| ⏳ Pendente | Não iniciada |
| 🔄 Em andamento | Em progresso |
| ✅ Concluída | Finalizada |
| ⏸️ Bloqueada | Bloqueada por dependência |
| ❌ Cancelada | Cancelada |

### Prioridades

| Prioridade | Descrição |
|------------|-----------|
| 🔴 Alta | Crítica para a fase atual |
| 🟡 Média | Importante mas não crítica |
| 🟢 Baixa | Nice to have |
