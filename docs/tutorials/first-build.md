# Primeiro Build

## Pré-requisitos

- Rust toolchain instalada (`rustup`, `cargo` e `rustc`)
- Rust estável com `rustfmt` e `clippy`

## Passos

1. **Clonar o repositório**
   ```bash
   git clone https://github.com/zequinha-taveira/openkey.git
   cd openkey
   ```

2. **Verificar a formatação**
   ```bash
   cargo fmt --check
   ```

3. **Compilar o workspace ativo**
   ```bash
   cargo build --workspace
   ```

4. **Executar a CLI de exemplo**
   ```bash
   cargo run -p openkey-cli
   ```

5. **Executar testes e lint**
   ```bash
   cargo test --workspace
   cargo clippy --workspace --all-targets -- -D warnings
   ```

O firmware específico de placas ainda será conectado por adaptadores em `boards/`. A
implementação anterior de firmware, protocolos e simulador foi preservada em `legacy/` e
não participa do workspace ativo.

## Erros comuns

- **Toolchain não instalada**: `rustup toolchain install stable`
- **Componentes ausentes**: `rustup component add rustfmt clippy`
- **Alvo embarcado ausente**: adicionar o target somente quando o adaptador da placa for definido
