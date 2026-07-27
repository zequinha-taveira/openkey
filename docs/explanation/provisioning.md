# Provisionamento

## Visão Geral

O firmware nunca deve assumir características da placa. O OpenKey Configurator grava:

- Board Profile
- Device Profile
- Application Configuration

na Flash persistente.

## Fluxo

```text
Configurator
      │
      ▼
Configuration Manager
      │
      ▼
Flash Storage (persistente)
```

## Estados

- **Unprovisioned** - Nenhuma configuração
- **Partial** - Alguma configuração
- **Provisioned** - Configuração completa

## Comando

```bash
openkey-configurator provision --board board.json --device device.json
```