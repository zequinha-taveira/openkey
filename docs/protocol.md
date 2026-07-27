# Protocolos do OpenKey

## 📌 Visão Geral

O OpenKey implementa os protocolos FIDO2/CTAP2.1 e WebAuthn conforme as especificações da FIDO Alliance e W3C.

## 🔌 Protocolos Suportados

### CTAP2 (Client-to-Authenticator Protocol)
- `authenticatorGetInfo` - Informações do dispositivo
- `authenticatorMakeCredential` - Criação de credencial
- `authenticatorGetAssertion` - Autenticação
- `authenticatorClientPIN` - Gestão de PIN
- `authenticatorReset` - Reset de fábrica
- `authenticatorGetDisposition` - Status do dispositivo

### CBOR (Concise Binary Object Representation)
- Codificação/decodificação canônica estática
- Sem alocação dinâmica na heap
- Validação rigorosa de estruturas

### USB HID
- Framing CTAPHID
- Pacotes de 64 bytes
- Suporte a canais múltiplos

### WebAuthn
- Nível 2 e 3 da especificação W3C
- Autenticação com presença de usuário (UP)
- Verificação de usuário (UV)

## 📖 Documentação Detalhada

- [CBOR](protocols/cbor.md)
- [CTAP2](protocols/ctap2.md)
- [HID](protocols/hid.md)
- [WebAuthn](protocols/webauthn.md)
- [Machine de Estados do Protocolo](protocols/protocol-state-machine.md)