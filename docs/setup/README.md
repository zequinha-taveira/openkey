# Host Setup & Guia de Plataformas (`docs/setup/README.md`)

Bem-vindo ao guia de configuração de ambiente host, desenvolvimento e suporte a plataformas do **OpenKey Framework**.

Este diretório contém instruções detalhadas para preparar o ambiente de desenvolvimento, configurar drivers USB, utilizar ferramentas de diagnósticos e executar o OpenKey em diferentes sistemas operacionais e plataformas móveis.

---

## 🧭 Mapa da Documentação de Setup

```text
docs/setup/
├── README.md           # Este índice geral
├── linux.md            # Guia de configuração e desenvolvimento em Linux
├── windows.md          # Guia de configuração e desenvolvimento em Windows
├── macos.md            # Guia de configuração e desenvolvimento em macOS
├── android.md          # Suporte e integração em Android (USB OTG / WebAuthn)
├── ios.md              # Suporte e integração em iOS (USB-C / WebAuthn)
├── python.md           # Setup da toolchain Python e OpenKey Host SDK
├── rust.md             # Setup da toolchain Rust (core, targets e fuzzing)
├── cmake.md            # Setup do CMake/Ninja para HAL C e Pico SDK
├── usb-drivers.md      # Identidade USB, política VID/PID, FIDO HID e CCID
├── troubleshooting.md # Solução de problemas comuns, permissões e diagnósticos
└── build.md            # Guia unificado de compilação (Firmware, Simulador, SDK, CLI, GUI)
```

---

## 💻 Sistemas Operacionais Suportados

| Plataforma | Suporte FIDO HID | Suporte CCID | Papel Principal | Guia Detalhado |
| :--- | :--- | :--- | :--- | :--- |
| **Linux** | Sim (via udev/hidraw) | Sim (via pcscd) | Desenvolvimento, HIL, Fuzzing | [`linux.md`](linux.md) |
| **Windows** | Sim (Nativo) | Sim (WinSCard) | Desenvolvimento, Aplicações Host | [`windows.md`](windows.md) |
| **macOS** | Sim (Nativo) | Sim (PCSC framework) | Desenvolvimento, Testes WebAuthn | [`macos.md`](macos.md) |
| **Android** | Sim (via USB OTG) | Opcional | Uso Final, Diagnóstico, App OpenKey | [`android.md`](android.md) |
| **iOS** | Sim (via USB-C) | Opcional | Uso Final, Validação WebAuthn | [`ios.md`](ios.md) |

---

## 🔑 Identidade USB e Perfis de Build

O OpenKey estabelece uma política clara para identificação de dispositivos USB:

1. **Build Oficial**: Utiliza o VID/PID registrado oficial do projeto OpenKey (obtido legalmente, ex: via *pid.codes* ou Vendor ID próprio).
2. **Builds de Interoperabilidade**: Perfis de compilação fornecidos exclusivamente para testes de desenvolvimento e homologação que utilizam um VID/PID alternativo de testes sem personificar a identidade de outros fabricantes de hardware.

Consulte o guia [`usb-drivers.md`](usb-drivers.md) para mais detalhes sobre a pilha USB e regras de enumeração.
