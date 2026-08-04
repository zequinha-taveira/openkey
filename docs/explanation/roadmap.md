# Plano de Desenvolvimento

## Fases

### Fase 1: Infraestrutura
- Estrutura do monorepo
- HAL traits
- Board Profile, Device Profile
- Configuration Manager

### Fase 2: Protocolo
- CTAP2
- CBOR
- WebAuthn

### Fase 3: Segurança
- PIN management
- Credential storage
- Attestation

### Fase 4: Host
- SDK Python/Rust
- CLI
- Configurator

## Roadmap

| Milestone | Status | Data Estimada |
|-----------|--------|---------------|
| MVP | Em andamento | 2024-Q4 |
| Beta | Planejado | 2025-Q2 |
| 1.0 GA | Planejado | 2025-Q4 |

## Fase 10 — Desktop GUI (OpenKey Manager)

| Milestone | Status |
|-----------|--------|
| ADR-0013 (framework PySide6 + estrutura) | Planejado |
| Gaps do SDK (HID real, ClientPIN, CredentialManagement) | Planejado |
| OpenKey Manager `host/gui/` (core + ui) | Planejado |
| Job Python na CI (matrix 3 SO, headless) | Planejado |
| Instaladores nativos em `packaging/` | Planejado |

Próximas fases: **Fase 11** (Multi-target STM32/nRF/ESP32 + interoperabilidade),
**Fase 12** (Hardening, Fuzzing, Auditoria, RC 1.0).