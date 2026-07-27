# WebAuthn

## Visão Geral

Web Authentication (Level 2/3) - W3C.

## Fluxo

1. **Chamada** - `navigator.credentials.create()` ou `get()`
2. **Transferência** - Dados via CTAP2
3. **Processamento** - No dispositivo
4. **Resposta** - Ativação ou assertion

## Tipos de Credencial

- **Resident Key (RK)** - Credencial descobrível
- **Non-Resident Key** - Credencial não descobrível

## Opções

- `rk` - Suporte a Resident Keys
- `up` - User Presence
- `uv` - User Verification