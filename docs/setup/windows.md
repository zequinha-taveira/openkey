# Host Setup — Windows (`docs/setup/windows.md`)

## 📌 USB Enumeration

No Windows, o OpenKey enumera como um dispositivo composto USB (*USB Composite Device*), disponibilizando as seguintes interfaces:

- **USB Composite Device**
- **FIDO HID** (Interface de Autenticação FIDO2 / CTAP2)
- **CCID** (Interface Smart Card - Opcional)

```text
Gerenciador de Dispositivos (Windows)
↓
OpenKey (USB Composite Device)
↓
├── FIDO HID (Dispositivo de Interface Humana)
↓
└── Smart Card Reader (Leitor de Cartão Inteligente - CCID)
```

---

## 🔌 Drivers

### FIDO HID
- **Nenhum driver adicional é necessário.** 
- O Windows reconhece nativamente a interface FIDO HID utilizando o driver padrão `HIDClass` do sistema operacional.
- O acesso a dispositivos FIDO2 pelo Windows 10/11 é gerenciado nativamente pela **Windows WebAuthn API**.

### CCID (Smart Card)
- Utiliza o driver de **Smart Card Reader** nativo fornecido pelo Windows (`Microsoft Usbccid Driver`).
- Funciona de forma transparente com o subsistema WinSCard.

---

## 🛠️ Ambiente de Desenvolvimento

### 1. Pré-requisitos
Para compilar o OpenKey Framework, SDK e ferramentas de host no Windows, instale:

- **Visual Studio Build Tools** (com a carga de trabalho *C++ Build Tools* e SDK do Windows 10/11).
- **Rust** (instalado via `rustup-init.exe` escolhendo o MSVC toolchain `x86_64-pc-windows-msvc`).
- **Python 3.10+** (adicionado ao PATH).
- **CMake** (adicionado ao PATH).
- **Git for Windows**.

### 2. Configuração do Rust Toolchain

```powershell
# Adicionar o alvo cross-compile para a plataforma de referência RP2350
rustup target add thumbv7em-none-eabihf
```

### 3. Compilando o Projeto

No PowerShell ou Prompt de Comando do Desenvolvedor:

```powershell
# Compilar o workspace completo
cargo build --workspace

# Executar a suíte de testes
cargo test --workspace
```

---

## 💻 Compatibilidade de Ferramentas e APIs

O OpenKey no Windows é totalmente compatível com:
- **Windows WebAuthn API** (Windows Hello e Autenticação de Navegador).
- **libfido2** / `fido2-token`.
- **python-fido2**.
- **WinSCard API / PC-SC**.

---

## 🔍 Diagnóstico e Verificação

### Gerenciador de Dispositivos (`devmgmt.msc`)
Abra o Gerenciador de Dispositivos e verifique a seguinte hierarquia:

```text
Gerenciador de Dispositivos
├── Dispositivos de Interface Humana (HID)
│   └── Dispositivo de Entrada USB (OpenKey FIDO HID)
├── Leitores de cartão inteligente
│   └── Microsoft Usbccid Smartcard Reader (OpenKey CCID)
└── Dispositivos do sistema
    └── USB Composite Device (OpenKey)
```

### Diagnóstico em PowerShell
Para verificar a presença do dispositivo USB conectado via PowerShell:

```powershell
Get-PnpDevice | Where-Object { $_.FriendlyName -like "*OpenKey*" -or $_.Description -like "*FIDO*" }
```
