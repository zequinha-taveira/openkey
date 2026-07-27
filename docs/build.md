# Guia de Build do OpenKey

## 🔧 Requisitos

- Rust 1.70+ (stable)
- Cargo
- Para RP2350: ARM Cortex-M toolchain, Pico SDK

## 🛠️ Comandos de Build

```bash
# Build do workspace
cargo build --workspace

# Build de release (sem unwind)
cargo build --workspace --release

# Build para alvo específico
cargo build --package openkey-core

# Build do simulador
cargo build --package openkey-simulator

# Build do firmware RP2350 (não-std)
cargo build --package openkey-target-rp2350
```

## ⚙️ Configurações

### Profile de Release
```toml
[profile.release]
panic = "abort"
lto = true
opt-level = "z"
```

### Profile de Desenvolvimento
```toml
[profile.dev]
panic = "abort"
```

## 📦 Cross-compilation

Para compilar para o RP2350:
```bash
# Instalar toolchain ARM
rustup target add thumbv6m-none-eabi

# Build
cargo build --package openkey-target-rp2350 --target thumbv6m-none-eabi
```

## 📖 Documentação Relacionada

- [Setup Rust](setup/rust.md)
- [Setup CMake & Pico SDK](setup/cmake.md)
- [Build Reproduzível](architecture/build.md)