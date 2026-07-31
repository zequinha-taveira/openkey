# Changelog

Todas as alterações notáveis neste projeto serão documentadas neste arquivo.

O formato é baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.0.0/),
e este projeto adere ao [Semantic Versioning](https://semver.org/lang/pt-BR/).

## [Unreleased]

## [1.0.0] - 2026-07-31

### Adicionado
- **PAR-01 Foundation (`v0.1.0`)**: Estrutura completa do monorepo, governança (`AGENTS.md`, `TASKS.md`, `PHASES.md`), CI/CD automatizado no GitHub Actions.
- **PAR-02 Architecture (`v0.2.0`)**: Especificação técnica `spec.md` (15 Requisitos Funcionais + 10 Requisitos Não-Funcionais), 12 ADRs aceitas e imutáveis, Threat Model e Security Principles.
- **PAR-03 Platform (`v0.3.0` / `v0.3.1`)**: Platform Abstraction Layer (PAL) com 10 HAL traits, Board & Device Profiles, e Config Manager A/B dual-slot com criptografia AES-256-GCM (ADR-0011, ADR-0012).
- **PAR-04 Security (`v0.4.0`)**: Secure Boot API com verificação ECDSA P-256 e bootloader dual-bank com rollback, Secure Storage com wear-leveling circular e detecção de corte de energia (`Writing` state recovery), Key Management com zeroização automática via `Drop`, interface OTP/efuses, Device Identity (AAGUID) e validação de TRNG contínua NIST SP 800-90B (Monobit, Poker, Runs, CRNGT). Codebase 100% Safe Rust auditado por Miri.
- **PAR-05 Protocols (`v0.5.0`)**: Pilha de protocolos no crate `openkey-protocols`:
  - Engine CBOR Canônico estático (`no_std`, RFC 8949) sem alocação heap.
  - Estrutura COSE Sign1 (RFC 9052) com algoritmos `ES256` (`-7`) e `EdDSA` (`-8`).
  - Framing USB HID CTAPHID de 64 bytes com suporte a pacotes `Init` e `Cont`, alocação de canais e montagem multi-pacotes.
  - CTAP2 Command Engine com `authenticatorGetInfo` (0x04) e enums de status/erro CTAP2.1.
  - Integração WebAuthn (`AuthenticatorData` `authData` e COSE Key P-256).
- **PAR-06 Host Tools (`v0.6.0`)**: Ferramentas de computador em `host/`:
  - Python SDK (`openkey-sdk` v0.6.0) para comunicação CTAPHID e CTAP2.
  - CLI Tool (`openkey-cli`) com subcomandos `info`, `pin`, `credentials`, `reset`, `update`.
  - Configurator Tool para diagnósticos e gerenciamento de preferências.
  - Provisioner Tool de fábrica para injeção de AAGUID e chaves de atestação.
  - Firmware Updater Tool para atualização via dual-bank bootloader.
- **PAR-07 Validation (`v0.7.0`)**: Suíte de testes com 55 testes unitários e de integração aprovados, validação de interoperabilidade, resiliência de falha de energia e compilação do target RP2350.

### Alterado
- Bump da versão do Cargo Workspace para `1.0.0`.
- Documentação sincronizada em 100% das fases do roadmap.
