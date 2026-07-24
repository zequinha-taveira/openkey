# OpenKey Firmware (`firmware/`)

Este diretório contém o firmware embarcado `no_std` desenvolvido em Rust para a chave de segurança de hardware OpenKey.

## 🧱 Arquitetura

O firmware gerencia:
- Pilha USB HID e NFC (`docs/architecture/transport.md`)
- Parser e serializador CBOR canonical (`docs/protocols/cbor.md`)
- Máquina de estados CTAP2.0 / CTAP2.1 (`docs/protocols/ctap2.md`)
- Camada de abstração de armazenamento seguro na Flash (`docs/architecture/storage.md`)
- Primitivas criptográficas aceleradas via hardware (`docs/architecture/crypto.md`)

Para documentação de arquitetura detalhada, veja [`docs/architecture/firmware.md`](../docs/architecture/firmware.md).
