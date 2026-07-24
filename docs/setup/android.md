# Host Setup — Android (`docs/setup/android.md`)

## 📌 Comunicação

No Android, o OpenKey comunica-se através dos seguintes canais:

```text
OpenKey (Chave de Segurança)
│
├── Conexão USB OTG (Cabo / Adaptador Direct USB-C)
│   ↓
│   FIDO HID Interface
│   ↓
│   CCID (Quando suportado pelo sistema/hardware Android)
```

---

## 📱 Compatibilidade de Autenticação

- **Google Chrome / Browsers Android**: Suporte nativo a FIDO2 / CTAP2 via USB OTG.
- **Android Credential Manager API**: Integração com FIDO2 WebAuthn para login sem senha em apps nativos e web.
- **Google Play Services (FIDO2 API)**: Gerenciamento do fluxo de autenticação física com prompt de toque de presença de usuário.

---

## 📲 Aplicativo OpenKey Android

O ecossistema OpenKey inclui o **OpenKey Companion App para Android** (localizado em `host/android`), que fornece as seguintes funcionalidades:

- **Diagnóstico do Dispositivo**: Leitura de estado, contadores e versão do firmware.
- **Gerenciamento de PIN**: Configuração e alteração do PIN do usuário.
- **Gerenciamento de Credenciais**: Enumeração e exclusão de credenciais residentes.
- **Atualização de Firmware**: Suporte a atualização segura de firmware via USB OTG.

---

## 🛠️ Ambiente de Desenvolvimento

O sistema operacional Android **não é a plataforma principal para desenvolvimento do firmware**.

O Android é utilizado primariamente para:
- **Testes de Interoperabilidade Mobile**.
- **Validação de fluxos WebAuthn em dispositivos portáteis**.
- **Demonstrações e testes em campo**.

### Requisitos para Desenvolver o App Android OpenKey
- **Android Studio (Ladybug / Jellyfish ou superior)**.
- **Android NDK** (para compilação dos bindings Rust do `openkey-sdk`).
- **Kotlin 1.9+ / Gradle 8+**.
