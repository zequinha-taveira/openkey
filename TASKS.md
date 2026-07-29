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
| PAR-04 | Security | 🔄 Em andamento | 30% |
| PAR-05 | Protocols | ⏳ Pendente | 0% |
| PAR-06 | Host Tools | ⏳ Pendente | 0% |
| PAR-07 | Validation | ⏳ Pendente | 0% |
| PAR-08 | Release | ⏳ Pendente | 0% |

---

## 🔧 PAR-04 — Security (Em Andamento)

### Prioridade Alta

- [ ] **SEC-001**: Implementar Secure Boot API no crate `firmware/boot`
  - Sub-tasks:
    - [ ] Definir trait `SecureBootProvider` em `firmware/boot/src/`
    - [ ] Implementar verificação de assinatura ECDSA P-256 (conforme ADR-0008)
    - [ ] Implementar bootloader dual-bank com rollback
    - [ ] Adicionar testes unitários no simulador

- [ ] **SEC-002**: Implementar Secure Storage
  - Sub-tasks:
    - [ ] Implementar wear-leveling circular no crate `firmware/storage`
    - [ ] Integrar AES-256-GCM para dados sensíveis (conforme ADR-0002)
    - [ ] Implementar power-loss recovery
    - [ ] Adicionar testes de integridade

- [ ] **SEC-003**: Implementar Key Management
  - Sub-tasks:
    - [ ] Definir trait `KeyProvider` para chaves de atestação
    - [ ] Implementar geração de pares de chaves P-256 / Ed25519
    - [ ] Implementar zeroização de chaves efêmeras
    - [ ] Adicionar testes de zeroização

- [ ] **SEC-004**: Implementar OTP Interface
  - Sub-tasks:
    - [ ] Definir trait `OtpProvider` no HAL
    - [ ] Implementar interface para OTP (One-Time Programmable) memory
    - [ ] Implementar leitura de chaves de atestação únicas

- [ ] **SEC-005**: Implementar Device Identity
  - Sub-tasks:
    - [ ] Definir estrutura `DeviceIdentity` em `firmware/platform/src/`
    - [ ] Implementar AAGUID (Authenticator Attestation GUID)
    - [ ] Implementar certificado de atestação
    - [ ] Adicionar validação de identidade no boot

### Prioridade Média

- [ ] **SEC-006**: Revisão de segurança do código `unsafe`
  - Sub-tasks:
    - [ ] Auditar todos os blocos `unsafe` existentes
    - [ ] Verificar comentários `// SAFETY:` em todos os blocos
    - [ ] Executar Miri para detecção de UB
    - [ ] Documentar resultados da auditoria

- [ ] **SEC-007**: Implementar TRNG health checks
  - Sub-tasks:
    - [ ] Implementar testes NIST SP 800-90B no `RngProvider`
    - [ ] Adicionar validação contínua de entropia
    - [ ] Implementar fallback para RNG software (apenas em simulador)

---

## ⏳ PAR-05 — Protocols (Pendente)

> Depende da conclusão de PAR-04 (Security)

### Prioridade Alta

- [ ] **PROTO-001**: Implementar CBOR parser/serializer canônico
  - Sub-tasks:
    - [ ] Implementar parser CBOR estático (sem alocação heap)
    - [ ] Implementar validação de canonicidade (RFC 8949)
    - [ ] Adicionar testes de fuzzing

- [ ] **PROTO-002**: Implementar COSE
  - Sub-tasks:
    - [ ] Implementar estrutura COSE Sign1
    - [ ] Integrar com crypto (ECDSA P-256 / Ed25519)
    - [ ] Adicionar testes de interoperabilidade

- [ ] **PROTO-003**: Implementar CTAP HID
  - Sub-tasks:
    - [ ] Implementar framing CTAPHID (conforme ADR-0003)
    - [ ] Implementar gerenciamento de canais
    - [ ] Implementar comandos: INIT, PING, MSG, CANCEL, ERROR
    - [ ] Adicionar timeout e reassembly

- [ ] **PROTO-004**: Implementar CTAP2
  - Sub-tasks:
    - [ ] Implementar `authenticatorGetInfo`
    - [ ] Implementar `authenticatorMakeCredential`
    - [ ] Implementar `authenticatorGetAssertion`
    - [ ] Implementar `authenticatorClientPIN`
    - [ ] Implementar `authenticatorCredentialManagement`
    - [ ] Implementar `authenticatorReset`

- [ ] **PROTO-005**: Implementar WebAuthn
  - Sub-tasks:
    - [ ] Implementar parsing de `PublicKeyCredential`
    - [ ] Implementar validação de `rpIdHash`
    - [ ] Implementar geração de `attestation statement`
    - [ ] Adicionar testes de interoperabilidade com navegadores

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
