# Primeiro Build

## Pré-requisitos

- Rust toolchain instalada
- Cargo

## Passos

1. **Clonar o repositório**
   ```bash
   git clone https://github.com/openkey/openkey.git
   cd openkey
   ```

2. **Build do workspace**
   ```bash
   cargo build --workspace
   ```

3. **Build do simulador**
   ```bash
   cargo build --package openkey-simulator
   ```

4. **Build do firmware RP2350**
   ```bash
   cargo build --package openkey-target-rp2350
   ```

## Erros Comuns

- **Toolchain não instalada**: `rustup install stable`
- **Alvo não configurado**: `rustup target add thumbv6m-none-eabi`