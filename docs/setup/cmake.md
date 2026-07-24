# CMake & Toolchain C Setup (`docs/setup/cmake.md`)

## 📌 Objetivo

Configurar o ambiente de compilação baseado em **CMake** e **Ninja** para compilar drivers C, a HAL de baixo nível e integração com o **Raspberry Pi Pico SDK** e a pilha **TinyUSB** na implementação de referência RP2350.

---

## 🛠️ Requisitos de Software

- **CMake 3.20** ou superior.
- **Ninja Build System** (recomendado para compilações rápidas).
- **GNU Arm Embedded Toolchain** (`arm-none-eabi-gcc` / `arm-none-eabi-g++`).

---

## 📥 Instalação dos Pacotes

### Linux (Debian / Ubuntu / Raspberry Pi OS)

```bash
sudo apt update && sudo apt install -y \
    cmake \
    ninja-build \
    gcc-arm-none-eabi \
    libnewlib-arm-none-eabi \
    libstdc++-arm-none-eabi-newlib \
    build-essential \
    git
```

### macOS (via Homebrew)

```bash
brew install cmake ninja arm-none-eabi-gcc
```

### Windows (via winget ou Choco)

```powershell
winget install Kitware.CMake
winget install Ninja-build.Ninja
winget install Arm.GNUToolchain.13.3.Rel1
```

---

## 🔌 Configuração do Raspberry Pi Pico SDK

Para compilar os drivers C do RP2350, o repositório utiliza submódulos Git no diretório `third_party/pico-sdk`:

```bash
# Inicializar e atualizar submódulos
git submodule update --init --recursive

# Definir a variável de ambiente PICO_SDK_PATH
export PICO_SDK_PATH=$(pwd)/third_party/pico-sdk
```

---

## 🏗️ Compilando com CMake

Para gerar os artefatos C e integrações de hardware:

```bash
# Criar diretório de build
mkdir -p build && cd build

# Gerar arquivos de compilação com Ninja
cmake -G Ninja -DPICO_BOARD=pico2 ..

# Compilar a imagem final do firmware (.elf / .uf2)
ninja
```

A imagem gerada `.uf2` estará disponível em `build/targets/rp2350/openkey-rp2350.uf2` para gravação via bootloader USB drag-and-drop no RP2350.
