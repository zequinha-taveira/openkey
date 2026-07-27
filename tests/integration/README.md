# Testes de Integração (`tests/integration/`)

Testes end-to-end entre o SDK, CLI e o Simulador/Hardware OpenKey.

Migrado de `host/tests/`. Cobre os fluxos completos:
- `MakeCredential` → `GetAssertion`
- `ClientPin` (PIN creation, change, lockout)
- `CredentialManagement` (enumeration, deletion)
- `Reset` do autenticador

Consulte [`docs/development/testing.md`](../../docs/development/testing.md) para instruções de execução.
