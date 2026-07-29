# OpenKey — Histórico de Versões e Controle de Fases

> **Documento vivo.** Atualizado a cada **Gate de fase aprovado**.
> Formato: Keep a Changelog + SemVer + Rastreabilidade por Fase (PAR).
> Fonte de verdade para auditoria, rastreabilidade e comparação entre versões.

---

## Índice Rápido — Tabela Resumo

| Versão | Fase | Status | Data | Gate / Critério Principal |
|--------|------|--------|------|---------------------------|
| [v0.1.0](#v010--par-01-foundation) | PAR-01 Foundation | ✅ Aprovado | 2026-07-24 | Estrutura, CI, Docs iniciais |
| [v0.2.0](#v020--par-02-architecture) | PAR-02 Architecture | ✅ Aprovado | 2026-07-24 | ADRs 0001–0009, Arquitetura |
| [v0.3.0](#v030--par-03-platform) | PAR-03 Platform | ✅ Aprovado | 2026-07-27 | Compila, HAL estável, Testes |
| [v0.3.1](#v031--par-03-platform-config-manager-ab-slots) | PAR-03 Platform (micro) | ✅ Aprovado | 2026-07-27 | Config A/B + AES-256-GCM |
| [v0.4.0](#v040--par-04-security) | PAR-04 Security | 🔄 Em desenvolvimento | — | Security review, Testes |
| v0.5.0 | PAR-05 Protocols | ⏳ Pendente | — | Interop tests |
| v0.6.0 | PAR-06 Host Tools | ⏳ Pendente | — | Integração SDK/CLI |
| v0.7.0 | PAR-07 Validation | ⏳ Pendente | — | Todos testes, Cobertura |
| v1.0.0 | PAR-08 Release | ⏳ Pendente | — | RC aprovado, Tag criada |

> **Legenda:** ✅ Aprovado · 🔄 Em desenvolvimento · 🔄 Em revisão · ⏳ Pendente · ⏸️ Bloqueado

---

## Convenções de Versionamento

| Tipo | Formato | Quando |
|------|---------|--------|
| **Fase principal (PAR)** | `v0.X.0` | Cada gate de fase concluído (PAR-01 → v0.1.0, PAR-02 → v0.2.0, ...) |
| **Micro-release (patch)** | `v0.X.Y` | PRs/commits significativos dentro da fase (ex.: nova feature, refatoração maior) |
| **Release Candidate** | `v1.0.0-rc.N` | PAR-08, validação final |
| **Release Final** | `v1.0.0` | PAR-08 gate aprovado, tag git criada |

**Política:** [SemVer 2.0.0](https://semver.org/) com tags globais `vX.Y.Z` no monorepo ([ADR-0009](docs/reference/adr/ADR-0009-versioning.md)). Firmware e SDK mantêm Major/Minor alinhados.

---

## v0.1.0 — PAR-01 Foundation

| Campo | Valor |
|-------|-------|
| **Versão** | v0.1.0 |
| **Fase** | PAR-01 — Foundation |
| **Data da alteração** | 2026-07-24 |
| **Objetivo da fase** | Criar a fundação do projeto: estrutura do monorepo, documentação inicial, CI/CD, governança |
| **Status** | ✅ Aprovado |
| **Gate** | ✅ Estrutura criada · ✅ Documentação inicial aprovada · ✅ CI funcionando |

### Alterações Realizadas

- **Estrutura do Monorepo** criada com diretórios: `firmware/` (core, platform, protocols, storage, crypto, usb, config, boot), `host/` (cli, configurator, provisioner, sdk-python, updater, diagnostics), `tools/` (simulator, generators, manufacturing, migration, scripts), `boards/` (profiles, templates, examples), `docs/`, `examples/`, `fuzz/`, `packaging/`, `tests/`, `third_party/`, `scripts/`, `cmake/`
- **Arquivos de Governança e Processo:** `AGENTS.md`, `TASKS.md`, `PHASES.md`, `README.md`, `CODE_OF_CONDUCT.md`, `GOVERNANCE.md`, `SECURITY.md`, `CONTRIBUTING.md`, `SUPPORT.md`, `RELEASING.md`, `Development Plan.md`, `Product.md`, `Ecosystem.md`, `spec.md`
- **Workspace Cargo** configurado com 11 crates (`openkey-core`, `openkey-platform`, `openkey-protocols`, `openkey-storage`, `openkey-crypto`, `openkey-usb`, `openkey-config`, `openkey-boot`, `openkey-target-rp2350`, `openkey-simulator`) com `version.workspace = true` herdando `0.1.0`
- **CI/CD GitHub Actions** (`.github/workflows/ci.yml`): `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --workspace` em push/PR para `main` e `develop`
- **ADRs Iniciais (0001–0009)** criadas e aprovadas:
  - ADR-0001: Rust para firmware
  - ADR-0002: Flash storage com wear-leveling
  - ADR-0003: USB HID stack e channel multiplexing
  - ADR-0004: Política estrita de `unsafe`
  - ADR-0005: Arquitetura Host SDK (Rust core + Python bindings)
  - ADR-0006: Builds reproduzíveis via Docker
  - ADR-0007: Crypto suite e validação TRNG
  - ADR-0008: Flash layout e dual-bank bootloader
  - ADR-0009: Versionamento SemVer no monorepo
- **Documentação Diataxis** completa estruturada em 4 categorias:
  - **Tutorials** (5): getting-started, first-build, first-provisioning, first-security-key, add-new-board
  - **How-to** (8): build-rp23xx, flash-firmware, provision-device, update-firmware, recover-device, create-board-profile, create-device-profile, release
  - **Reference** (40+): architecture, protocols, crypto, host, boards, api, adrs
  - **Explanation** (9): product, development-plan, threat-model, security-principles, universal-firmware, provisioning, commissioning, architecture-decisions, roadmap
  - **Diagrams** (5 .drawio): architecture, provisioning, startup, storage, usb
- **Submódulos Git** configurados: CMSIS_5, FIDO conformance tools, nrf-hal

### Motivo das Alterações

Estabelecer base sólida, padronizada e auditável para desenvolvimento colaborativo de firmware de security key. A estrutura em camadas (Startup → HAL → Board/Device Profile → Config Manager → Platform Services → Core) garante separação de responsabilidades, testabilidade e portabilidade entre MCUs. ADRs capturam decisões arquiteturais irreversíveis desde o início. CI automatiza qualidade de código. Documentação Diataxis atende públicos distintos (aprendizes, operadores, referenciadores, tomadores de decisão).

### Prompts / Decisões Registradas

- **Arquitetura em camadas** definida no Prompt Mestre / Development Plan: separação estrita entre Security Core (`firmware/core`) e Platform Abstraction Layer (`firmware/platform`)
- **Linguagem:** Rust `no_std` para todo firmware; `std` apenas para simulador, testes e ferramentas host
- **Versionamento:** SemVer global com tags `vX.Y.Z` (ADR-0009) — firmware e SDK sincronizam Major/Minor
- **Documentação:** Modelo Diataxis adotado por clareza de propósito por tipo de documento
- **Segurança:** Política `unsafe` restritiva (ADR-0004) — blocos `unsafe` só com comentário `// SAFETY:` e auditoria Miri
- **Storage:** Wear-leveling circular + AES-256-GCM (ADR-0002) — dual-bank para bootloader (ADR-0008)
- **Board Profiles:** Dados YAML (não código Rust) resolvidos via `BoardProfileId` + catálogo compilado

### Resultados Obtidos

- Monorepo compila: `cargo build --workspace` ✅
- CI verde no GitHub Actions (fmt, clippy, test) ✅
- 9 ADRs aprovadas e versionadas ✅
- Documentação navegável, estruturada e versionada ✅
- Base para todas as fases subsequentes estabelecida ✅

### Problemas Encontrados

- Nenhum bloqueador crítico na fase
- Ajustes menores em `Cargo.toml` para herança de versão (`version.workspace = true`)
- Imports circulares iniciais entre crates resolvidos com re-exports em `lib.rs`

### Correções Aplicadas

- Padronização de `edition = "2021"` em todo workspace
- `overflow-checks = true` em `.cargo/config.toml` para debug
- `.gitignore` atualizado para novo layout do monorepo (commit `2105e37`)

### Dependências para Próxima Fase (PAR-02)

- ADRs 0001–0009 aprovadas e imutáveis
- Estrutura de crates estável e compilando
- CI funcional e green
- Documentação Diataxis publicada

### Observações Importantes

- Fase concluída em sprint único (fundação)
- **Commit base:** `44ea24b` (reestruturação monorepo per ADR-0010) → `5c27818` (docs Diataxis) → `00a4977` (remoção hardware/ + doc estratégia)
- Base para **todas** decisões arquiteturais subsequentes
- `CHANGELOG.md` iniciado com seção `[Unreleased]` documentando esta fase

---

## v0.2.0 — PAR-02 Architecture

| Campo | Valor |
|-------|-------|
| **Versão** | v0.2.0 |
| **Fase** | PAR-02 — Architecture |
| **Data da alteração** | 2026-07-24 |
| **Objetivo da fase** | Definir toda a arquitetura detalhada: specs, ADRs, data flow, threat model, security principles |
| **Status** | ✅ Aprovado |
| **Gate** | ✅ Arquitetura revisada · ✅ ADRs aprovadas · ✅ Nenhuma decisão pendente |

### Alterações Realizadas

- **Especificação Técnica (`spec.md`)** finalizada com:
  - **RF-001 a RF-015** (Requisitos Funcionais): CTAP2, WebAuthn, Credential Management, PIN/UV, Enterprise Attestation, Firmware Update, Device Config, Provisioning, Diagnostics, Interoperabilidade
  - **RNF-001 a RNF-010** (Requisitos Não-Funcionais): Segurança, Performance, Confiabilidade, Portabilidade, Manutenibilidade, Auditoria, Usabilidade, Compliance
- **ADRs 0001–0009** revisadas, validadas e marcadas como **Accepted** (todas em 2026-07-24)
- **Documentação de Arquitetura** expandida em `docs/reference/architecture/`:
  - `architecture.md` — Visão geral em camadas
  - `firmware.md` — Security Core internals
  - `platform.md` — Platform Abstraction Layer
  - `startup.md` — Sequência de boot
  - `hal.md` — HAL traits (10 traits: Flash, RNG, USB, GPIO, Timer, Watchdog, I2C, SPI, UART)
  - `board-profile.md` / `device-profile.md` — Estruturas de dados YAML/Rust
  - `config-manager.md` — A/B persistence design
  - `storage.md` — Wear-leveling circular
- **Data Flow** documentado (`docs/reference/data-flow.md`): Host ↔ USB HID ↔ CTAPHID ↔ CTAP2 Engine ↔ Crypto/Storage
- **Threat Model** (`docs/explanation/threat-model.md`): STRIDE aplicado a firmware, supply chain, side-channel, physical
- **Security Principles** (`docs/explanation/security-principles.md`): Fail-closed, least privilege, defense in depth, zeroize, auditability
- **Roadmap** (`docs/explanation/roadmap.md`): Fases PAR-01 a PAR-08 com marcos

### Motivo das Alterações

Consolidar todas as decisões arquiteturais antes de escrever código de produção. Evitar retrabalho caro em firmware embarcado onde mudanças de arquitetura impactam flash layout, boot sequence, e interfaces de hardware. Garantir que Security Core seja isolado, auditável e portável.

### Prompts / Decisões Registradas

- **Security Core** deve ser `no_std`, sem dependência de HAL concreta — só traits
- **HAL Traits** (10) definem contrato mínimo para qualquer MCU alvo
- **Board Profile** = dados estáticos (YAML) → compilado em catálogo Rust
- **Device Profile** = dados por dispositivo (serial, calibração, manufatura) → persistido em Flash A/B
- **Config Manager** = orquestra A/B slots com versionamento, checksum, AEAD (AES-256-GCM)
- **Storage Manager** = circular wear-leveling com page headers (CRC-16, sequence numbers)
- **Crypto Suite** = P-256 (ECDSA), Ed25519, AES-256-GCM, SHA-256, HKDF, TRNG com health checks NIST SP 800-90B
- **Bootloader** = dual-bank assimetricamente assinado (ECDSA P-256), rollback automático em falha

### Resultados Obtidos

- Especificação completa (`spec.md`) com 15 RFs + 10 RNFs ✅
- 9 ADRs aceitas e imutáveis ✅
- Arquitetura em camadas documentada e revisada ✅
- Threat model e security principles publicados ✅
- Zero decisões arquiteturais pendentes ✅

### Problemas Encontrados

- Necessidade de ADR-0010 (monorepo restructure) identificada durante revisão — criada posteriormente (2026-07-27)
- Definição de `DeviceText` (UTF-8 fixed 64 bytes) para `no_std` safety adicionada ao design

### Correções Aplicadas

- Ajuste no `spec.md` para alinhar RFs com CTAP2.1 / WebAuthn Level 2
- ADR-0008 refinada: flash layout inclui região OTP para chaves de atestação únicas

### Dependências para Próxima Fase (PAR-03)

- `spec.md` assinado e congelado
- Todas as ADRs 0001–0009 aceitas
- HAL traits definidas (10 traits em `firmware/platform/src/hal/`)
- Board/Device Profile structures definidas
- Config Manager A/B design aprovado

### Observações Importantes

- Fase de **design puro** — nenhum código de produção escrito ainda
- **Commits:** `7a2ea36` (platform persist device config safely) inicia implementação baseada neste design
- Base contratual para PAR-03 (Platform implementation)

---

## v0.3.0 — PAR-03 Platform

| Campo | Valor |
|-------|-------|
| **Versão** | v0.3.0 |
| **Fase** | PAR-03 — Platform |
| **Data da alteração** | 2026-07-27 |
| **Objetivo da fase** | Criar a infraestrutura do firmware: Platform API, HAL traits, Config Manager, Board/Device Profiles, Storage Manager, Crypto primitives |
| **Status** | ✅ Aprovado |
| **Gate** | ✅ Compila · ✅ Interfaces estáveis · ✅ Testes básicos passando |

### Alterações Realizadas

#### Crate `firmware/platform` (HAL + Profiles + Config + Services)
- **HAL Traits (10)** implementadas em `src/hal/`:
  - `FlashStorageProvider` (read, write, erase, total_size) + `FlashError`
  - `RngProvider` (fill_bytes, next_u32, is_healthy)
  - `UsbTransportProvider` (send_packet, receive_packet, is_connected) + `UsbDeviceProvider`
  - `GpioProvider` (set_direction, set_level, get_level) + `GpioDirection`, `GpioLevel`
  - `TimerProvider` (millis, micros, delay_ms, delay_us)
  - `WatchdogProvider` (init, feed, disable)
  - `I2cProvider` (configure, read, write, write_read)
  - `SpiProvider` (configure, transfer) + `SpiMode`, `SpiBitOrder`
  - `UartProvider` (init, write, read, available)
- **Board Profile** (`src/board.rs`): `BoardProfile`, `BoardProfileId`, `BoardProfileCatalog` trait, `GpioPin`, `LedConfig`, `ButtonConfig`, `UsbConfig`, `FlashConfig`, `OptionalFeatures`
- **Device Profile** (`src/device.rs`): `DeviceProfile`, `DeviceText` (64-byte UTF-8), `UsbIdentity`, `CalibrationData`, `ManufacturingData`
- **Config Manager** (`src/config.rs`): `ConfigurationManager` com A/B slot persistence, `ConfigKeyProvider` trait, `ConfigCryptoContext`, `ConfigStorageLayout`, `ProvisioningState` enum (Unprovisioned, Provisioned, Locked)
- **App Config** (`src/app_config.rs`): `AppConfig` com `Ctap2Config`, `CcidConfig`, `OpenPgpConfig`, `PivConfig`, `LoggingConfig`, `SecurityPolicies`
- **Platform Services** (`src/services.rs`): `HardwareProviders` struct + `PlatformServices` orchestrator
- **Re-exports** em `src/lib.rs` + `PLATFORM_VERSION` constant

#### Crate `firmware/crypto`
- AES-256-GCM encrypt/decrypt para config persistence: `encrypt_config()`, `decrypt_config()`
- `CONFIG_AEAD_KEY_SIZE = 32`, `CONFIG_AEAD_NONCE_SIZE = 12`, `CONFIG_AEAD_TAG_SIZE = 16`
- `AeadError` enum

#### Crate `firmware/storage`
- `StorageManager` com circular wear-leveling
- `PageHeader` (magic, version, sequence, crc16, state), `PageState` (Empty, Active, Obsolete, Corrupted)
- `StorageKeyProvider`, `StorageRngProvider` traits
- **5 unit tests** com `MockFlash` passando

#### Crate `firmware/core`
- Core engine entry point: `OPENKEY_CORE_VERSION`, `core_info()`
- `CoreError` enum: TransportError, RngFailure, StorageFailure, UserPresenceTimeout, ProtocolError
- Unit tests básicos

#### Crate `firmware/protocols`, `firmware/usb`, `firmware/config`, `firmware/boot`
- Stubs com version constants (implementação em PAR-04/05)

#### Target RP2350 (`firmware/platform/mcu/rp23xx`)
- `BOARD_PROFILE` e `DEVICE_PROFILE` constants
- HAL stubs retornando `Err(HalError::Unsupported)` / `Err(HalError::NotImplemented)`
- `main()` entry point `no_std` + `panic_handler`

#### Simulador (`tools/simulator`)
- Implementações `std` de todos HAL traits: `DummyRng`, `DummyFlash`, `DummyGpio`, `DummyUsb`, `DummyTimer`, `DummyWatchdog`
- `main()` testa todos providers

#### ADRs Novas (criadas durante implementação)
- **ADR-0010** (2026-07-27): Monorepo Restructure — layout final `firmware/`, `boards/`, `host/`, `tools/`
- **ADR-0011** (2026-07-27): A/B Configuration Persistence — dual-slot flash com versionamento, validação pré-Provisioned
- **ADR-0012** (2026-07-27): Crypto/Platform Boundary for Authenticated Config — AEAD key management via `ConfigKeyProvider`

### Motivo das Alterações

Materializar a arquitetura definida em PAR-02 em código Rust compilável, testável e `no_std`. Estabelecer traits HAL como contratos estáveis para permitir desenvolvimento paralelo de board ports. Implementar Config Manager e Storage Manager com segurança criptográfica (AES-256-GCM + wear-leveling) como base para persistência de chaves e credenciais.

### Prompts / Decisões Registradas

- **HAL Traits** devem ser `object-safe` onde possível (exceto associated types) para dyn dispatch
- **`DeviceText`** = wrapper `no_std` sobre `heapless::String<64>` para UTF-8 safe sem alocação
- **A/B Slots** = slot 0 (active) + slot 1 (staging), swap atômico após validação completa
- **Versionamento de persistência** = v1 (checksum-only) → v2 (AEAD) — migração requer reprovisioning (ADR-0011)
- **Wear-leveling** = circular buffer com sequence numbers monotônicos, CRC-16 por page
- **Zeroize** = chaves sensíveis zeradas em `Drop` via trait `zeroize::Zeroize`
- **Fail-closed** = qualquer erro de crypto/storage/config → estado `Locked` ou `Unprovisioned`

### Resultados Obtidos

- **Workspace compila** `cargo build --workspace` ✅ (target `thumbv7em-none-eabihf` para RP2350)
- **Testes unitários passando**:
  - `platform::config` — 12 tests (A/B read/write, version validation, crypto round-trip, provisioning state machine)
  - `storage` — 5 tests (wear-leveling, page state transitions, CRC detection, power-loss simulation)
  - `core` — basic tests ✅
- **HAL Traits** estáveis e documentadas ✅
- **Board/Device Profiles** serializáveis (YAML ↔ Rust) ✅
- **ADR-0010, 0011, 0012** aceitas ✅

### Problemas Encontrados

- `firmware/protocols`, `firmware/usb`, `firmware/config`, `firmware/boot` permanecem stubs — implementação real em PAR-04/05
- RP2350 HAL stubs não funcionais em hardware real — precisam implementação `rp-hal` / `embassy-rp`
- Simulador usa `std` — não valida `no_std` constraints (ex.: `heapless`, `no_std` alloc)

### Correções Aplicadas

- Fix em `ConfigStorageLayout` para alinhamento de página flash (4096 bytes)
- `PageHeader::crc16` usando `crc16::State::<crc16::XMODEM>` consistente
- `ProvisioningState` machine: transição `Unprovisioned → Provisioned` só após validação AEAD + size + UTF-8
- `MockFlash` no storage tests simula power-loss entre write e erase

### Dependências para Próxima Fase (PAR-04)

- Platform API estável (HAL traits, Config Manager, Storage Manager)
- Crypto primitives (AES-256-GCM) funcionais
- Board Profile RP2350 definido
- ADR-0008 (Flash layout) + ADR-0011 (A/B persistence) + ADR-0012 (AEAD boundary) aceitas

### Observações Importantes

- **Commits chave:** `7a2ea36` (persist device config safely) → `601fb7e` (authenticate persistent settings) → `6edba70` (authenticated config management + dual-slot flash storage + agent configs)
- **Micro-release v0.3.1** documentada abaixo para o merge do Config Manager A/B
- Base sólida para Security (PAR-04): Secure Boot, Secure Storage, Key Management, OTP, Device Identity

---

## v0.3.1 — PAR-03 Platform: Config Manager A/B Slots (Micro-release)

| Campo | Valor |
|-------|-------|
| **Versão** | v0.3.1 (patch) |
| **Fase** | PAR-03 — Platform (micro-release) |
| **Data** | 2026-07-27 |
| **PR / Commit** | `6edba70` / `feat: implement authenticated configuration management system with dual-slot flash storage and add agent configuration files` |
| **Status** | ✅ Aprovado (mergeado em main) |

### Alterações Realizadas

- **`ConfigurationManager`** completo em `firmware/platform/src/config.rs`:
  - A/B slot persistence com `ConfigSlot` struct (data + metadata)
  - Validação pré-escrita: tamanho, UTF-8, versão, checksum CRC-16
  - Criptografia AES-256-GCM via `ConfigCryptoContext` (key do `ConfigKeyProvider`, nonce do TRNG)
  - State machine `ProvisioningState`: `Unprovisioned → Provisioned → Locked`
  - Swap atômico de slots após validação completa do staging slot
  - Rollback automático se validação falhar
- **`ConfigKeyProvider` trait** — boundary Crypto/Platform (ADR-0012): `derive_config_key(&mut self) -> Result<[u8; 32], ConfigError>`
- **`ConfigStorageLayout`** — offsets fixos para slot A/B, metadados, versionamento
- **`BoardProfileId`, `BoardProfileCatalog`, `ConfigStorageLayout`** — separação perfis de placa vs dados de dispositivo
- **12 unit tests** cobrindo: round-trip encrypt/decrypt, slot swap, version rejection, corruption detection, provisioning flow, power-loss simulation
- **ADR-0011** (Device Configuration Persistence) e **ADR-0012** (Config AEAD Boundary) criadas e aceitas
- **Agent configs** adicionados em `.context/agents/` (13 especialistas) + `.context/skills/` (12 skills)

### Motivo das Alterações

Requisito crítico: persistência autenticada e confidencial de configuração de dispositivo (Device Profile + AppConfig) resistente a power-loss, rollback e tampering. Base para provisionamento de fábrica e updates OTA seguros.

### Prompts / Decisões Registradas

- **AEAD key derivation** isolada no `ConfigKeyProvider` (Platform não conhece key material)
- **Nonce** = 12 bytes do TRNG (`RngProvider::fill_bytes`) — nunca reutilizado
- **Version check** = rejeita v1 (checksum-only) → migração requer reprovisioning completo
- **Slot metadata** = version (u8), flags (u8), crc16 (u16), sequence (u32), length (u16)

### Resultados Obtidos

- Config Manager funcional com 12 tests ✅
- ADR-0011, ADR-0012 aceitas ✅
- Integração com `StorageManager` (wear-leveling) validada ✅
- Agent configs para desenvolvimento assistido por IA ✅

### Problemas Encontrados

- `ConfigKeyProvider` implementation real depende de OTP/HSM (PAR-04 SEC-005)
- Simulador usa `DummyRng` — nonce determinístico em testes

### Correções Aplicadas

- Ajuste em `ConfigSlot::validate()` para verificar `sequence` monotônico anti-replay
- `MockFlash` expandido para simular partial write + power loss

### Dependências para Próxima Fase (PAR-04)

- `ConfigKeyProvider` implementation concreta (OTP/HSM)
- Secure Boot para proteger integridade do Config Manager
- Device Identity (AAGUID, attestation cert) para binding de config

### Observações Importantes

- Este micro-release **não** incrementa a versão do workspace Cargo (mantém `0.1.0` até release formal)
- Documentado aqui para rastreabilidade granular solicitada
- **Commit:** `6edba70` (HEAD atual)

---

## v0.4.0 — PAR-04 Security

| Campo | Valor |
|-------|-------|
| **Versão** | v0.4.0-dev (em desenvolvimento) |
| **Fase** | PAR-04 — Security |
| **Data da alteração** | Em andamento (iniciada 2026-07-27) |
| **Objetivo da fase** | Implementar infraestrutura de segurança: Secure Boot, Secure Storage, Key Management, OTP Interface, Device Identity |
| **Status** | 🔄 Em desenvolvimento (≈30%) |
| **Gate** | ☐ Security review · ☐ Testes aprovados |

### Alterações Realizadas (até o momento)

- **Planejamento detalhado** em `TASKS.md` (SEC-001 a SEC-007)
- **ADRs base** já aceitas: ADR-0004 (Unsafe Policy), ADR-0007 (Crypto Suite), ADR-0008 (Flash Layout), ADR-0011 (Config Persistence), ADR-0012 (AEAD Boundary)
- **Crate `firmware/boot`** — stub com `BOOT_VERSION` (implementação Secure Boot pendente)
- **Crate `firmware/crypto`** — AES-256-GCM funcional (usado por Config Manager)
- **Crate `firmware/storage`** — wear-leveling + AES-GCM funcional (base para Secure Storage)

### Tarefas em Andamento (conforme `TASKS.md`)

#### Prioridade Alta

| Task | Descrição | Status | Sub-tasks |
|------|-----------|--------|-----------|
| **SEC-001** | Secure Boot API no `firmware/boot` | ⏳ Pendente | Trait `SecureBootProvider`, verificação ECDSA P-256, dual-bank rollback, testes simulador |
| **SEC-002** | Secure Storage | ⏳ Pendente | Wear-leveling circular (já em storage), AES-256-GCM integração, power-loss recovery, testes integridade |
| **SEC-003** | Key Management | ⏳ Pendente | Trait `KeyProvider`, geração P-256/Ed25519, zeroização chaves efêmeras, testes zeroização |
| **SEC-004** | OTP Interface | ⏳ Pendente | Trait `OtpProvider` no HAL, interface OTP memory, leitura chaves atestação únicas |
| **SEC-005** | Device Identity | ⏳ Pendente | Struct `DeviceIdentity`, AAGUID, certificado atestação, validação no boot |

#### Prioridade Média

| Task | Descrição | Status |
|------|-----------|--------|
| **SEC-006** | Revisão de segurança código `unsafe` | ⏳ Pendente (auditar blocos, `// SAFETY:`, Miri, documentar) |
| **SEC-007** | TRNG Health Checks | ⏳ Pendente (NIST SP 800-90B, validação contínua, fallback simulador) |

### Motivo das Alterações

Estabelecer根基 de confiança (Root of Trust) no firmware: boot seguro, armazenamento resistente a tampering, gerenciamento de chaves com zeroização, identidade de dispositivo atestável. Pré-requisito para protocolos FIDO2/CTAP2 (PAR-05) que dependem de chaves de atestação e credenciais protegidas.

### Prompts / Decisões Registradas

- **Secure Boot** = verificação assinatura ECDSA P-256 do firmware bank ativo antes de jump (ADR-0008)
- **Dual-bank** = bank A (ativo) + bank B (staging) — rollback automático se verificação falhar
- **Key Management** = chaves de atestação em OTP/efuses; chaves efêmeras zeroizadas após uso
- **OTP Interface** = trait no HAL para portabilidade (RP2350 OTP, ESP32 eFuse, STM32 OTP, nRF UICR)
- **Device Identity** = AAGUID único por modelo + certificado X.509 de atestação assinado por CA da fábrica
- **`unsafe` Policy** = ADR-0004: cada bloco `unsafe` requer `// SAFETY:` + auditoria Miri em CI

### Resultados Obtidos (parciais)

- Crypto primitives (AES-256-GCM) ✅
- Storage wear-leveling + encryption ✅
- Config Manager A/B com AEAD ✅
- ADRs de segurança base aceitas ✅

### Problemas Encontrados (até agora)

- `firmware/boot`, `firmware/protocols`, `firmware/usb`, `firmware/config` são stubs — precisam implementação real
- RP2350 HAL não implementado — `rp-hal` / `embassy-rp` integration pendente
- OTP/HSM interface depende de HAL traits ainda não implementadas no target
- Miri não configurado no CI ainda (necessário para SEC-006)

### Correções Aplicadas (até agora)

- Nenhuma — fase em início de implementação

### Dependências para Próxima Fase (PAR-05)

- **SEC-001 a SEC-005** concluídas e testadas
- **SEC-006** (auditoria `unsafe`) aprovada
- **SEC-007** (TRNG health checks) funcional
- Secure Boot protegendo integridade do firmware
- Chaves de atestação injetáveis via OTP/Provisioner
- Device Identity válida e verificável no boot

### Observações Importantes

- **Progresso:** ~30% (planejamento + crypto/storage base prontos)
- **Bloqueadores atuais:** HAL implementation no RP2350, OTP provider trait
- **Próximo marco:** SEC-001 (Secure Boot trait + dual-bank) — desbloqueia validação de integridade
- Atualizar este documento a cada task SEC-XXX concluída (micro-versões v0.4.1, v0.4.2...)

---

## Versões Futuras (Planejadas)

> Estas entradas são placeholders — serão preenchidas quando cada gate for aprovado.

### v0.5.0 — PAR-05 Protocols (⏳ Pendente)
- **Objetivo:** CBOR, COSE, CTAP HID, CTAP2, WebAuthn implementation
- **Gate:** Testes de interoperabilidade + compatibilidade validada
- **Depende de:** PAR-04 Security concluída

### v0.6.0 — PAR-06 Host Tools (⏳ Pendente)
- **Objetivo:** Python SDK, CLI, Configurator GUI, Provisioner, Updater
- **Gate:** Testes de integração + documentação atualizada
- **Depende de:** PAR-05 Protocols concluída

### v0.7.0 — PAR-07 Validation (⏳ Pendente)
- **Objetivo:** Testes unitários, integração, hardware, interoperabilidade, regressão
- **Gate:** Todos testes aprovados + cobertura mínima atingida
- **Depende de:** PAR-06 Host Tools concluída

### v1.0.0 — PAR-08 Release (⏳ Pendente)
- **Objetivo:** Primeira versão estável
- **Gate:** RC aprovado + documentação completa + tag `v1.0.0` criada
- **Depende de:** PAR-07 Validation concluída
- **Entregáveis:** CHANGELOG final, Release Notes, pacotes distribuição, binários assinados

---

## Processo de Atualização deste Documento

### Quando Atualizar

**Obrigatório** a cada:
1. **Gate de fase aprovado** → Nova versão principal `v0.X.0` (ex.: PAR-04 gate → v0.4.0)
2. **Micro-release significativo** dentro da fase → Patch `v0.X.Y` (ex.: feature completa, refatoração maior, bugfix crítico)
3. **Release Candidate / Final** → `v1.0.0-rc.N` / `v1.0.0`

**Opcional** (recomendado):
- Tarefa de prioridade Alta concluída (SEC-001, PROTO-001, etc.) → entrada resumida na seção da fase atual

### Como Preencher (Checklist)

- [ ] Copiar template da seção correspondente (fase principal ou micro-release)
- [ ] Preencher **todos os campos** da tabela de metadados (Versão, Fase, Data, Objetivo, Status, Gate)
- [ ] **Alterações Realizadas:** lista concreta (arquivos, structs, traits, testes, ADRs)
- [ ] **Motivo:** por que foi feito (requisito, dependência, qualidade, segurança)
- [ ] **Prompts/Decisões:** decisões arquiteturais, trade-offs, políticas adotadas
- [ ] **Resultados:** o que funciona, testes passando, métricas
- [ ] **Problemas:** bloqueadores, bugs conhecidos, dívida técnica
- [ ] **Correções:** fixes aplicados, workarounds
- [ ] **Dependências:** o que a próxima fase/task precisa deste trabalho
- [ ] **Observações:** contexto extra, commits chave, marcos
- [ ] Atualizar **Tabela Resumo** no topo (versão, status, data, gate)
- [ ] Commit com mensagem: `docs(version-history): v0.X.Y — PAR-XX <resumo>`

### Responsável

- **Fase principal:** Tech Lead / Arquiteto da fase
- **Micro-release:** Autor do PR / implementador
- **Revisão:** Code Review obrigatório (mesmo processo do código)

### Versionamento do Workspace Cargo

| Evento | `Cargo.toml` `workspace.package.version` |
|--------|------------------------------------------|
| Desenvolvimento contínuo | Mantém `0.1.0` (ou versão base da fase) |
| Gate de fase aprovado | Bump `0.X.0` (ex.: PAR-04 gate → `0.4.0`) |
| Release Candidate | `1.0.0-rc.N` |
| Release Final | `1.0.0` + tag git `v1.0.0` |

> **Nota:** `VERSION_HISTORY.md` tem granularidade maior que `Cargo.toml`. Micro-versões (v0.3.1) **não** alteram `Cargo.toml` — só documentam aqui.

### Sincronização com Outros Artefatos

| Artefato | Sincronização |
|----------|---------------|
| `CHANGELOG.md` | Entradas de release formal (v0.X.0, v1.0.0) replicadas no formato Keep a Changelog |
| `PHASES.md` | Gates marcados ✅ / 🔄 / ⏳ — espelha status deste documento |
| `TASKS.md` | Tasks concluídas → movidas para "Concluídas" com referência à versão aqui |
| ADRs | Novas ADRs criadas durante fase → listadas em "Alterações Realizadas" |
| Git Tags | Tags `vX.Y.Z` criadas no release → referenciadas na versão correspondente |

---

## Histórico de Alterações deste Documento

| Versão | Data | Autor | Alteração |
|--------|------|-------|-----------|
| 1.0 | 2026-07-29 | OpenKey Team | Criação inicial com PAR-01 a PAR-04 documentadas |
| — | — | — | — |

---

## Referências Cruzadas

- **Roadmap:** `PHASES.md` (PAR-01 a PAR-08)
- **Task Tracking:** `TASKS.md` (SEC-001 a REL-005)
- **Changelog Formal:** `CHANGELOG.md` (Keep a Changelog)
- **Versioning Policy:** `docs/reference/adr/ADR-0009-versioning.md`
- **Release Process:** `RELEASING.md`, `docs/how-to/release.md`
- **Architecture:** `docs/reference/architecture/`, `spec.md`
- **ADRs:** `docs/reference/adr/ADR-0001` a `ADR-0012`
- **Git Log:** `git log --oneline --graph` (commits `44ea24b` a `6edba70`)

---

*Fim do documento. Próxima atualização: conclusão de SEC-001 (Secure Boot) → v0.4.1*