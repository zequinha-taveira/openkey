# Estratégia de Testes (`docs/development/testing.md`)

## 🧪 Níveis de Teste no Monorepo

1. **Testes Unitários**:
   ```bash
   cargo test --workspace --lib
   ```
2. **Testes de Integração com Simulador**:
   ```bash
   cargo test --package openkey-tests
   ```
3. **Fuzzing (CBOR e CTAP)**:
   ```bash
   cargo fuzz run cbor_decode_target
   ```
4. **Hardware-in-the-Loop (HIL)**:
   Testes automatizados rodando em placas físicas conectadas a testbeds HIL via `probe-rs`.
