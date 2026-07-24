# Rust Setup & Toolchain (`docs/setup/rust.md`)

## 📌 Objetivo

Configurar a toolchain de compilação em **Rust** para compilar o `openkey-core` (`no_std`), a *Platform Abstraction Layer* (PAL), os alvos de firmware (`targets/rp2350` e `targets/simulator`), ferramentas CLI e harness de fuzzing.

---

## ⚙️ Instalação da Toolchain Rust

O OpenKey utiliza o gerenciador oficial **`rustup`**.

### Instalação

```bash
# Linux / macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
# Faça o download e execute rustup-init.exe de https://rustup.rs
```

---

## 🎯 Alvos de Compilação (Targets)

O monorepo utiliza múltiplos alvos de compilação:

1. **Host Native (`x86_64` / `aarch64`)**: Para compilar o Simulador de Software, SDK Rust, CLI e ferramentas Desktop.
2. **RP2350 Hardware Target (`thumbv7em-none-eabihf`)**: Para compilar o firmware embarcado `no_std` para a plataforma de referência RP2350 (ARM Cortex-M33).

### Adicionando o Alvo RP2350:

```bash
rustup target add thumbv7em-none-eabihf
```

---

## 🛠️ Componentes e Ferramentas Auxiliares

Instale as ferramentas normativas de qualidade de código:

```bash
rustup component add clippy rustfmt
```

Para análise de código inseguro ou execução de fuzzing:

```bash
# Instalar cargo-fuzz para sessões de fuzzing no simulador
cargo install cargo-fuzz

# Instalar llvm-tools para análise de cobertura de código
rustup component add llvm-tools-preview
```

---

## 🚀 Comandos de Compilação e Verificação

### Compilar todo o Workspace
```bash
cargo build --workspace
```

### Compilar Especificamente para o Hardware RP2350
```bash
cargo build --target thumbv7em-none-eabihf --package openkey-rp2350
```

### Checagens Obrigatórias de Código
Em conformidade com as regras do [`AGENTS.md`](../../AGENTS.md):

```bash
# 1. Checagem de Formatação
cargo fmt --check

# 2. Análise Estática de Linter
cargo clippy --all-targets -- -D warnings

# 3. Execução da Suíte de Testes
cargo test --workspace
```
