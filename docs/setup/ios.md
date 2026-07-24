# Host Setup — iOS (`docs/setup/ios.md`)

## 📌 Comunicação

No iOS (iPhone / iPad), o OpenKey comunica-se por meio dos seguintes canais físicos e lógicos:

```text
OpenKey (Chave de Segurança)
│
├── Conexão USB-C Direta (Dispositivos iPad / iPhone 15 e superiores)
│   ↓
│   FIDO HID Interface
│   ↓
│   CCID (Quando disponível e suportado por aplicações iOS)
```

---

## 🍏 Compatibilidade de Autenticação

- **Safari iOS**: Suporte nativo a WebAuthn / FIDO2 via chave USB-C.
- **ASWebAuthenticationSession**: Integração para autenticação em aplicativos de terceiros.
- **Passkeys / WebAuthn Framework**: Suporte completo a assinaturas e autenticação sem senha.

---

## 📲 Aplicativo OpenKey iOS

O aplicativo utilitário **OpenKey Manager para iOS** (localizado em `host/ios`) permite:
- Consultar informações e status do dispositivo OpenKey.
- Diagnóstico de saúde do hardware e contadores.
- Atualização de firmware (quando suportada pela plataforma iOS).
- Execução de testes de validação de resposta CTAP2.

---

## 🛠️ Ambiente de Desenvolvimento

O desenvolvimento do firmware e do core do OpenKey ocorre em Linux, Windows ou macOS.

No iOS, o objetivo foca em:
- **Utilização e validação por usuários finais**.
- **Testes de interoperabilidade com o ecossistema Apple Mobile**.

### Requisitos para Compilar o App iOS OpenKey
- **macOS com Xcode 15+**.
- **Swift 5.9+**.
- **Rust Toolchain (`aarch64-apple-ios` target)** para compilar a biblioteca nativa `openkey-sdk`.
