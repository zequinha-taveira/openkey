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
| PAR-05 | Protocols | ⏳ Pendente | 0% |
| PAR-06 | Host Tools | ⏳ Pendente | 0% |
| PAR-07 | Validation | ⏳ Pendente | 0% |
| PAR-08 | Release | ⏳ Pendente | 0% |

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
    - [x] Implementar parser CBOR estático (sem alocação heap) em `firmware/protocols/src/cbor/decoder.rs`
    - [x] Implementar validação de canonicidade (RFC 8949) em `firmware/protocols/src/cbor/`
    - [x] Adicionar testes unitários e de borda para validação canônica

- [x] **PROTO-002**: Implementar COSE
  - Sub-tasks:
    - [x] Implementar estrutura COSE Sign1 (RFC 9052) em `firmware/protocols/src/cose/mod.rs`
    - [x] Integrar com crypto (ECDSA P-256 / Ed25519) e Sig_structure
    - [x] Adicionar testes de encodagem e parsing roundtrip

- [x] **PROTO-003**: Implementar CTAP HID
  - Sub-tasks:
    - [x] Implementar framing CTAPHID (conforme ADR-0003) em `firmware/protocols/src/ctap_hid/mod.rs`
    - [x] Implementar gerenciamento de canais e sequenciamento de pacotes
    - [x] Implementar comandos: INIT, PING, MSG, CANCEL, ERROR
    - [x] Adicionar reassembly e testes de mensagens multi-pacote

- [x] **PROTO-004**: Implementar CTAP2
  - Sub-tasks:
    - [x] Implementar `authenticatorGetInfo` em `firmware/protocols/src/ctap2/get_info.rs`
    - [x] Implementar estruturas de resposta e códigos de status em `firmware/protocols/src/ctap2/status.rs`
    - [x] Implementar engine de comandos `Ctap2Engine` em `firmware/protocols/src/ctap2/mod.rs`

- [x] **PROTO-005**: Implementar WebAuthn
  - Sub-tasks:
    - [x] Implementar serialização de `AuthenticatorData` (`authData`)
    - [x] Implementar codificação de `PublicKeyCredential` (COSE Key P-256)
    - [x] Implementar suporte a `rpIdHash` e flags (`UP`, `UV`, `AT`)
    - [x] Adicionar testes unitários em `firmware/protocols/src/webauthn/mod.rs`

---

## ⏳ PAR-06 — Host Tools (Pendente)

> Depende da conclusão de PAR-05 (Protocols)

### Prioridade Alta

- [ ] **HOST-001**: Implementar Python SDK
  - Sub-tasks:
    - [ ] Implementar descoberta de dispositivos USB HID
    - [ ] Implementar comunicação CTAPHID
    - [ ] Implementar APIs de gerenciamento de credenciais
    - [ ] Adicionar testes de integração com simulador

- [ ] **HOST-002**: Implementar CLI
  - Sub-tasks:
    - [ ] Implementar subcomandos: info, pin, credentials, reset, update
    - [ ] Integrar com Python SDK
    - [ ] Adicionar testes de CLI

- [ ] **HOST-003**: Implementar Configurator
  - Sub-tasks:
    - [ ] Implementar interface gráfica (GUI)
    - [ ] Integrar com Python SDK
    - [ ] Adicionar funcionalidades de gerenciamento

- [ ] **HOST-004**: Implementar Provisioner
  - Sub-tasks:
    - [ ] Implementar provisionamento de fábrica
    - [ ] Implementar injeção de chaves de atestação
    - [ ] Adicionar testes de provisionamento

- [ ] **HOST-005**: Implementar Updater
  - Sub-tasks:
    - [ ] Implementar atualização de firmware via USB
    - [ ] Implementar verificação de assinatura
    - [ ] Adicionar testes de atualização

---

## ⏳ PAR-07 — Validation (Pendente)

> Depende da conclusão de PAR-06 (Host Tools)

- [ ] **VAL-001**: Testes unitários
- [ ] **VAL-002**: Testes de integração
- [ ] **VAL-003**: Testes de hardware
- [ ] **VAL-004**: Testes de interoperabilidade
- [ ] **VAL-005**: Testes de regressão

---

## ⏳ PAR-08 — Release (Pendente)

> Depende da conclusão de PAR-07 (Validation)

- [ ] **REL-001**: Preparar CHANGELOG
- [ ] **REL-002**: Preparar Release Notes
- [ ] **REL-003**: Criar pacotes de distribuição
- [ ] **REL-004**: Finalizar documentação
- [ ] **REL-005**: Criar tag de versão

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
