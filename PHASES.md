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

* [ ] Testes de integração
* [ ] Documentação atualizada

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

* [ ] Todos os testes aprovados
* [ ] Cobertura mínima atingida

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

* [ ] Release Candidate aprovado
* [ ] Documentação completa
* [ ] Tag de versão criada

---

# Regra Geral

Uma fase somente pode iniciar quando:

* a fase anterior estiver concluída;
* todos os critérios do Gate estiverem atendidos;
* a documentação estiver atualizada;
* não existirem bloqueadores críticos.

Caso contrário, o trabalho deve permanecer na fase atual.
