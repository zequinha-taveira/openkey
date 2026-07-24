# Pipeline de Integração Contínua (`docs/development/ci.md`)

## 🤖 Automação CI no GitHub Actions

O workflow `.github/workflows/ci.yml` executa automaticamente a cada push ou Pull Request:
1. `cargo fmt --check`
2. `cargo clippy --workspace`
3. `cargo test --workspace`
4. Auditoria de vulnerabilidades de dependências via `cargo-audit`.
