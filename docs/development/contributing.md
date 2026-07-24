# Guia Detalhado de Contribuição (`docs/development/contributing.md`)

## 🛠️ Ambiente Local de Desenvolvimento

1. Instalar Toolchain Rust (stable & nightly para fuzzing):
   ```bash
   rustup toolchain install stable
   rustup target add thumbv7em-none-eabihf
   ```
2. Clonar e rodar verificação geral:
   ```bash
   git clone https://github.com/openkey/openkey.git
   cd openkey
   cargo check --workspace
   ```
3. Consulte [`CONTRIBUTING.md`](../../CONTRIBUTING.md) na raiz do repositório para regras de Pull Request.
