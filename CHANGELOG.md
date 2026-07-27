# Changelog

Todas as alterações notáveis neste projeto serão documentadas neste arquivo.

O formato é baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.0.0/),
e este projeto adere ao [Semantic Versioning](https://semver.org/lang/pt-BR/).

## [Unreleased]

### Adicionado
- Persistência A/B versionada para Device Profile e AppConfig, com validação de
  tamanho, UTF-8, versão e checksum antes do estado `Provisioned`.
- `BoardProfileId`, `BoardProfileCatalog` e `ConfigStorageLayout` para separar
  perfis de placa dos dados persistidos por dispositivo.
- ADR-0011 e política documentada para código `unsafe`.
- Estrutura inicial do monorepo OpenKey (`firmware/`, `host/`, `docs/`, `hardware/`, `fuzz/`, `examples/`).
- Hierarquia completa de documentação cobrindo Arquitetura, Segurança, Protocolos (FIDO2, CTAP2, WebAuthn, USB HID, CBOR), Desenvolvimento e APIs.
- Conjunto inicial de ADRs (Architecture Decision Records) de 0001 a 0009 em `docs/adr/`.
- Políticas de governança, conduta e reporte de vulnerabilidades.

### Alterado
- Reorganização do monorepo para seguir a arquitetura universal descrita no Prompt Mestre.
  - `firmware/core/` → `core/` (Security Core)
  - `firmware/pal/` → `platform/` (HAL traits, Board/Device Profiles, Configuration Manager, Platform Services)
  - `firmware/targets/rp2350/` → `boards/rp2350/` (Board implementations)
  - Criados novos crates: `protocols/`, `storage/`, `crypto/`
  - Atualizada a estrutura de diretórios para refletir a arquitetura em camadas.
