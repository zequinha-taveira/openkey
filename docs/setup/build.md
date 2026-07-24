# Guia de Compilação Unificado (`docs/setup/build.md`)

## 📌 Objetivo

Fornecer as instruções consolidadas de compilação para todos os artefatos do monorepo **OpenKey**:
1. **OpenKey Security Core (`no_std`)**
2. **Implementação de Referência RP2350 (Firmware Hardware)**
3. **Simulador de Software Desktop (Target Software)**
4. **OpenKey Host SDK (Python / Rust)**
5. **Interface de Linha de Comando (`openkey-cli`)**
6. **Aplicação Desktop Graphical Manager (`openkey-gui`)**

---

## 🛠️ Pré-requisitos de Compilação

Consulte os guias específicos por linguagem e ferramenta antes de iniciar:
- [`rust.md`](rust.md): Instalação do Rust e alvos de compilação.
- [`cmake.md`](cmake.md): Instalação do CMake, Ninja e toolchain ARM GCC.
- [`python.md`](python.md): Ambiente virtual Python.

---

## 🚀 1. Compilando o Firmware de Referência RP2350

### Usando Cargo / Rust

```bash
# Adicionar o alvo ARM Cortex-M33
rustup target add thumbv7em-none-eabihf

# Compilar o firmware para o RP2350
cargo build --target thumbv7em-none-eabihf --package openkey-rp2350 --release
```

### Usando CMake / Pico SDK

```bash
# Atualizar submódulos
git submodule update --init --recursive

# Configurar e compilar a imagem UF2
mkdir -p build && cd build
cmake -G Ninja -DPICO_BOARD=pico2 ..
ninja
```

O arquivo `openkey-rp2350.uf2` gerado estará pronto para ser gravado na placa RP2350 via drag-and-drop.

---

## 💻 2. Compilando o Simulador de Software

O simulador compila nativamente na plataforma host (Linux, macOS, Windows):

```bash
# Compilar e executar o simulador de software
cargo run --package openkey-simulator
```

---

## 📦 3. Compilando o SDK e Ferramentas Host

### OpenKey Host CLI (`openkey-cli`)
```bash
cargo build --package openkey-cli --release
```
O binário resultante estará em `target/release/openkey-cli`.

### OpenKey Manager GUI (`openkey-gui`)
```bash
cargo build --package openkey-gui --release
```

### OpenKey Python SDK
```bash
cd host/sdk
pip install -e .[dev]
```

---

## 🧪 4. Executando a Suíte Completa de Testes

Para garantir a integridade de todo o monorepo antes de submeter um PR:

```bash
# 1. Testes do Workspace Rust
cargo test --workspace

# 2. Linter Clippy
cargo clippy --all-targets -- -D warnings

# 3. Formatação
cargo fmt --check

# 4. Testes do SDK Python
cd host/sdk && pytest
```
