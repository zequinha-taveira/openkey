# Architecture Decision Records (ADRs) 📜

Este diretório contém todos os Registros de Decisão de Arquitetura (ADRs) do projeto OpenKey. As ADRs documentam decisões arquiteturais significativas, seu contexto e suas consequências.

## 📋 Índice de ADRs

| ID | Título | Status | Data |
| --- | ------ | ------ | ---- |
| [ADR-0001](ADR-0001-rust.md) | Seleção da Linguagem Rust para Firmware | Aceito | 2026-07-24 |
| [ADR-0002](ADR-0002-storage.md) | Armazenamento Seguro e Wear-Leveling na Flash | Aceito | 2026-07-24 |
| [ADR-0003](ADR-0003-usb.md) | Implementação de Pilha USB HID e Multiplexação de Canais | Aceito | 2026-07-24 |
| [ADR-0004](ADR-0004-unsafe.md) | Política Estrita de Isolamento e Auditoria de Código `unsafe` | Aceito | 2026-07-24 |
| [ADR-0005](ADR-0005-sdk.md) | Arquitetura do Host SDK e Suporte a Bindings Multi-linguagem | Aceito | 2026-07-24 |
| [ADR-0006](ADR-0006-build.md) | Pipeline de Compilação Reproduzível via Containers Docker | Aceito | 2026-07-24 |
| [ADR-0007](ADR-0007-crypto.md) | Seleção da Suíte Criptográfica e Validação de Hardware TRNG | Aceito | 2026-07-24 |
| [ADR-0008](ADR-0008-flash-layout.md) | Layout de Memória Flash e Bootloader Dual-Bank com Assinatura Assimétrica | Aceito | 2026-07-24 |
| [ADR-0009](ADR-0009-versioning.md) | Política de Versionamento Semântico e Release em Monorepo | Aceito | 2026-07-24 |

---

## 📝 Formato das ADRs

Cada ADR deve seguir o modelo padronizado:
- **Título e Status**: (Proposto, Aceito, Obsoleto, Substituído)
- **Contexto**: O problema e as restrições técnicas.
- **Decisão**: A solução escolhida e a justificativa técnica.
- **Consequências**: Os impactos positivos e os compromissos (trade-offs) aceitos.
