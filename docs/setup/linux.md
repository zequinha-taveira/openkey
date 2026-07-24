# Host Setup — Linux (`docs/setup/linux.md`)

## 📌 Objetivo

Configurar um ambiente Linux completo para desenvolver, testar, depurar e utilizar o OpenKey Framework e a implementação de referência RP2350 ou o Simulador de Software.

---

## 🔌 USB Enumeration

O OpenKey enumera como um dispositivo USB composto (*USB Composite Device*):

- **Interface 0**: FIDO HID (Autenticação FIDO2 / CTAP2)
- **Interface 1**: CCID (Smart Card / PC-SC - Opcional)

As duas interfaces compartilham o mesmo dispositivo USB físico, porém funcionam de forma completamente independente.

```text
OpenKey (USB Composite Device)
│
├── Interface 0: FIDO HID (Usage Page: 0xF1D0, Usage: 0x0001)
│
└── Interface 1: CCID (Smart Card / PC-SC - Opcional)
```

---

## 🆔 Identidade USB

- **Build Oficial**: A build oficial utiliza o Vendor ID (VID) e Product ID (PID) do próprio projeto OpenKey.
- O nome do leitor PC/SC registrado no sistema contém a string: `OpenKey`.

---

## 🧪 Builds de Interoperabilidade

O sistema de build do OpenKey fornece perfis opcionais configuráveis para testes de interoperabilidade. 

- Esses perfis destinam-se **exclusivamente** ao desenvolvimento local e validação de compatibilidade com subsistemas host específicos.
- Perfis de interoperabilidade **nunca** fazem parte das imagens de release oficiais e não personificam a identidade comercial de terceiros.

---

## ⚙️ Requisitos para FIDO HID

Para interagir com a interface FIDO HID no Linux como usuário não-root, é necessário configurar as permissões adequadas de acesso aos nós `/dev/hidraw*`.

### Dependências
- `hidraw` (módulo de kernel Linux)
- `hidapi` (biblioteca de acesso a dispositivos HID)
- Regras `udev` de acesso sem privilégios de superusuário

### Regra udev do OpenKey
Crie o arquivo `/etc/udev/rules.d/70-openkey.rules` com o seguinte conteúdo:

```udev
# OpenKey FIDO2 / CTAP2 Security Key
KERNEL=="hidraw*", SUBSYSTEM=="hidraw", ATTRS{idVendor}=="1209", ATTRS{idProduct}=="0001", MODE="0660", GROUP="plugdev", TAG+="uaccess"
```

Após criar a regra, recarregue o subsistema udev:

```bash
sudo udevadm control --reload-rules && sudo udevadm trigger
```

### Ferramentas Host Compatíveis
- `libfido2` / `fido2-token`
- `python-fido2`
- Navegadores com suporte a WebAuthn (Chrome, Firefox, Edge, Brave)

---

## 💳 Requisitos para CCID (Smart Card)

Quando a interface CCID estiver habilitada no firmware:

### Dependências
- `pcsc-lite`
- Daemon `pcscd`

### Instalação em distribuições Debian/Ubuntu:

```bash
sudo apt update && sudo apt install -y pcscd libpcsclite-dev pcsc-tools opensc
sudo systemctl enable --now pcscd
```

### Ferramentas Compatíveis
- `pcsc_scan`
- `opensc-tool` / `pkcs11-tool`
- Outras aplicações baseadas em PC/SC.

---

## 🛠️ Ambiente de Desenvolvimento

### Instalação de Ferramentas Básicas

```bash
sudo apt install -y build-essential cmake ninja-build git pkg-config libusb-1.0-0-dev libftdi1-dev
```

### Toolchain Rust
Instale o Rust via `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add thumbv7em-none-eabihf
```

### Compilação do Projeto

```bash
# Compilar o simulador local e ferramentas host
cargo build --workspace

# Executar a suíte de testes unitários e de integração
cargo test --workspace
```

---

## 🔍 Diagnóstico e Verificação

### 1. Verificar Enumeração USB
```bash
lsusb | grep -i openkey
```

### 2. Verificar Dispositivos HID raw
```bash
ls -l /dev/hidraw*
```

### 3. Verificar Leitor PC/SC (CCID)
```bash
pcsc_scan
```
