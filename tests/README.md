# Suíte de Testes (`tests/`)

Testes separados por objetivo e escopo.

## Estrutura

```text
tests/
├── unit/              # Testes unitários isolados por crate
├── integration/       # Testes de integração SDK ↔ Simulador ↔ Firmware
├── interoperability/  # Testes de interoperabilidade com clients FIDO2 reais
├── hardware/          # Testes que requerem hardware físico conectado
└── regression/        # Suíte de regressão para bugs conhecidos
```

## Execução

```bash
# Todos os testes (exceto hardware)
cargo test --workspace

# Testes de integração
cargo test -p openkey-integration

# Testes de interoperabilidade (requer cliente FIDO2)
cargo test -p openkey-interop
```

Consulte [`docs/development/testing.md`](../docs/development/testing.md) para detalhes completos.
