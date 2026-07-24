# Arquitetura do Host SDK (`docs/architecture/host-sdk.md`)

## 💻 Arquitetura da Camada Host

A camada host (`host/sdk`) atua como a biblioteca de abstração entre as aplicações clientes (browsers, utilitários CLI, aplicativo GUI) e os dispositivos OpenKey (seja hardware real ou simulador via soquete local).

## 🏢 Módulos do SDK

- **`openkey-transport`**: Abstrai comunicação via USB HID nativo (winapi no Windows, hidapi no Linux/macOS) e comunicação NFC.
- **`openkey-ctap`**: Empacotador de requisições CTAP2.0 / CTAP2.1 e gerenciador de criptografia de canal `ClientPin` / `pinUvAuthToken`.
- **`openkey-bindings`**: Interface FFI C e bindings Python (`pyopenkey`).
