# Host Setup — macOS (`docs/setup/macos.md`)

## 📌 USB Enumeration

No macOS, o OpenKey apresenta as seguintes interfaces quando conectado via USB:

- **FIDO HID** (Interface Human Interface Device para FIDO2 / CTAP2)
- **CCID** (Interface Smart Card / PC-SC - Opcional)

O macOS reconhece nativamente a interface FIDO HID por meio do IOKit framework e gerencia requisições WebAuthn diretamente via Safari, Chrome, Firefox e Edge.

---

## 🛠️ Ambiente de Desenvolvimento

### 1. Xcode Command Line Tools
Instale os utilitários de compilação essenciais do macOS executando no Terminal:

```bash
xcode-select --install
```

### 2. Homebrew
Instale os pacotes de dependências via Homebrew:

```bash
brew install rustup cmake ninja pkg-config libfido2 pcsc-lite opensc
```

### 3. Configuração da Toolchain Rust

```bash
rustup-init
rustup target add thumbv7em-none-eabihf
```

---

## 🌐 Compatibilidade

O OpenKey é testado e suportado nativamente no macOS para:
- **Navegadores Web**: Safari, Google Chrome, Mozilla Firefox, Microsoft Edge, Brave.
- **APIs de Autenticação**: WebAuthn W3C API, LocalAuthentication framework.
- **Subsistema Smart Card**: Apple PCSC framework (`PCSC.framework`).

---

## 🧰 Ferramentas Host

Ferramentas CLI e bibliotecas suportadas no macOS:
- `libfido2` (`fido2-token`, `fido2-assert`, `fido2-cred`)
- `python-fido2`
- `OpenSC` (`opensc-tool`, `pkcs11-tool`)

---

## 🛠️ Compilando no macOS

```bash
# Compilar o Simulador e Ferramentas Host
cargo build --workspace

# Executar suíte de testes
cargo test --workspace
```

---

## 🔍 Diagnóstico e Verificação

Para listar os detalhes de enumeração USB do OpenKey no macOS via Terminal:

```bash
system_profiler SPUSBDataType | grep -A 10 -i "OpenKey"
```

Para listar leitores PC/SC ativos no macOS:

```bash
pcsctest
```
