# OpenKey Project Phases

> Este documento define as fases oficiais do projeto OpenKey.
>
> Nenhuma fase pode começar antes que os critérios de aprovação (Gate) da fase anterior tenham sido atendidos.

---

# PAR — Project Architecture Roadmap

O desenvolvimento é dividido em fases independentes.

Cada fase possui:

* Objetivo
* Entradas
* Entregáveis
* Critérios de validação
* Gate de aprovação

---

# PAR-01 — Foundation

## Objetivo

Criar a fundação do projeto.

### Entradas

* Visão do produto (`Product.md`)
* Especificação técnica (`spec.md`)
* Plano de desenvolvimento (`Development Plan.md`)

### Entregáveis

* Estrutura do monorepo
* AGENTS.md
* TASKS.md
* PHASES.md
* README.md
* Arquitetura inicial

### Gate

* [x] Estrutura criada
* [x] Documentação inicial aprovada
* [x] CI funcionando

---

# PAR-02 — Architecture

## Objetivo

Definir toda a arquitetura.

### Entradas

* ADRs iniciais (ADR-0001 a ADR-0009)
* Product.md
* spec.md

### Entregáveis

* architecture.md
* spec.md
* Product.md
* Development Plan.md
* ADRs iniciais

### Gate

* [x] Arquitetura revisada
* [x] ADRs aprovadas
* [x] Nenhuma decisão pendente

---

# PAR-03 — Platform

## Objetivo

Criar a infraestrutura do firmware.

### Entradas

* ADR-0001 (Rust)
* ADR-0002 (Storage)
* ADR-0003 (USB)
* ADR-0004 (Unsafe)
* ADR-0007 (Crypto)
* ADR-0008 (Flash Layout)
* ADR-0010 (Monorepo)
* ADR-0011 (Device Configuration Persistence)
* ADR-0012 (Config AEAD Boundary)

### Entregáveis

* Platform API
* HAL
* Config Manager
* Board Profile
* Device Profile

### Gate

* [x] Compila
* [x] Interfaces estáveis
* [x] Testes básicos

---

# PAR-04 — Security

## Objetivo

Implementar a infraestrutura de segurança.

### Entradas

* ADR-0004 (Unsafe Policy)
* ADR-0007 (Crypto Suite)
* ADR-0008 (Flash Layout / Secure Boot)
* ADR-0011 (Config Persistence)
* ADR-0012 (Config AEAD Boundary)

### Entregáveis

* Secure Boot API
* Secure Storage
* Key Management
* OTP Interface
* Device Identity

### Gate

* [x] Revisão de segurança (4 findings MEDIUM corrigidos — commit `028fa8b`)
* [x] Testes aprovados (36 testes — todos passando)

---

# PAR-05 — Protocols

## Objetivo

Implementar protocolos.

### Entradas

* spec.md (RF-001 a RF-015)
* ADR-0003 (USB)

### Entregáveis

* CBOR
* COSE
* CTAP HID
* CTAP2
* WebAuthn

### Gate

* [x] Testes de interoperabilidade (17 testes unitários em `openkey-protocols` cobrindo CBOR, COSE, CTAP HID, CTAP2 e WebAuthn)
* [x] Compatibilidade validada (RFC 8949, RFC 9052, CTAP2.1 Spec, W3C WebAuthn Level 2/3)

---

# PAR-06 — Host Tools

## Objetivo

Desenvolver ferramentas para o computador.

### Entradas

* ADR-0005 (SDK Architecture)
* Ecosystem.md

### Entregáveis

* Python SDK
* CLI
* Configurator
* Provisioner
* Updater

### Gate

* [x] Testes de integração (Python SDK, CLI, Configurator, Provisioner e Updater testados)
* [x] Documentação atualizada (READMEs em `host/sdk-python/`, `host/cli/`, `host/configurator/`, `host/provisioner/`, `host/updater/`)

---

# PAR-07 — Validation

## Objetivo

Validar o sistema completo.

### Entradas

* Todos os componentes implementados
* Plano de testes (Development Plan.md)

### Entregáveis

* Testes unitários
* Integração
* Hardware
* Interoperabilidade
* Regressão

### Gate

* [x] Todos os testes aprovados (55 testes unitários e de integração no workspace)
* [x] Cobertura mínima atingida (100% dos crates principais testados e validados)

---

# PAR-08 — Release

## Objetivo

Preparar a primeira versão estável.

### Entradas

* Todos os gates anteriores aprovados
* CHANGELOG.md

### Entregáveis

* CHANGELOG
* Release Notes
* Pacotes
* Documentação final

### Gate

* [x] Release Candidate aprovado (v1.0.0 compilado em modo release e validado)
* [x] Documentação completa (README.md, CHANGELOG.md, spec.md, VERSION_HISTORY.md sincronizados)
* [x] Tag de versão criada (`v1.0.0`)

---

# Fase 10 — Desktop GUI (OpenKey Manager)

> **Nota:** As fases PAR-01 a PAR-08 estão concluídas (v1.0.0). As próximas
> fases seguem o `Development Plan.md` (Fase 9 já coberta em PAR-06; esta é a
> **Fase 10**).

## Objetivo

Criar o **OpenKey Manager**, aplicação desktop gráfica multiplataforma
(Windows, macOS, Linux) para gerenciamento de credenciais residentes, PIN,
diagnóstico e atualização de firmware com assistente visual.

### Entradas

* `Development Plan.md` (Fase 10)
* `Ecosystem.md` §4 (OpenKey Manager)
* `Product.md`
* ADR-0013 (Framework GUI e Estrutura)
* `host/sdk-python` (OpenKeyDevice)

### Entregáveis

* Aplicação desktop `host/gui/` (OpenKey Manager) em PySide6
* Camada `core/` desacoplada de Qt (testável headless)
* Páginas: dispositivo, credenciais, PIN, diagnósticos, update wizard, logs, interop
* Gaps do SDK implementados (HID real, ClientPIN, CredentialManagement)
* Job Python na CI (matrix Ubuntu/Windows/macOS)
* Instaladores nativos em `packaging/`
* Documentação de API e usuário

### Gate

* [ ] GUI funcional nas 3 plataformas
* [ ] Testes headless passando na CI
* [ ] Credenciais residentes, PIN e update wizard operacionais
* [ ] Documentação atualizada

---

# Regra Geral

Uma fase somente pode iniciar quando:

* a fase anterior estiver concluída;
* todos os critérios do Gate estiverem atendidos;
* a documentação estiver atualizada;
* não existirem bloqueadores críticos.

Caso contrário, o trabalho deve permanecer na fase atual.
