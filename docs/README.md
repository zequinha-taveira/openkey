# Documentação do OpenKey 📖

Bem-vindo ao centro de documentação oficial do monorepo **OpenKey**.

---

## 🧭 Mapa da Documentação

```text
docs/
├── README.md                 # Índice da documentação
├── tutorials/                # Aprendizado (getting started, first builds)
├── how-to/                   # Tarefas (build, flash, provision)
├── reference/                # Referência técnica
│   ├── architecture/         # Architecture, firmware, HAL, profiles
│   ├── protocols/            # CTAP2, WebAuthn, HID, CBOR, COSE
│   ├── crypto/               # Crypto, keys, attestation, RNG
│   ├── host/                 # Python SDK, CLI, Configurator
│   ├── boards/               # RP23xx, ESP32-S3, STM32, nRF
│   └── api/                  # Firmware API, Host SDK API
├── explanation/              # Conceitos (product, threat model, decisions)
├── adr/                      # Architecture Decision Records
├── architecture.md           # Visão geral da arquitetura
├── firmware.md               # Arquitetura do firmware
├── protocol.md               # Protocolos
├── storage.md                # Armazenamento
├── security-principles.md    # Princípios de segurança
├── threat-model.md           # Modelo de ameaças
├── testing.md                # Estratégia de testes
└── build.md                  # Guia de build
```

---

## 🎓 Tutorials (Aprendizado)

- [Getting Started](tutorials/getting-started.md)
- [Primeiro Build](tutorials/first-build.md)
- [Primeiro Provisionamento](tutorials/first-provisioning.md)
- [Primeira Chave de Segurança](tutorials/first-security-key.md)
- [Adicionar Novo Board](tutorials/add-new-board.md)

---

## 🔧 How-to (Tarefas)

- [Build Firmware RP23xx](how-to/build-rp23xx.md)
- [Flash Firmware](how-to/flash-firmware.md)
- [Provisionar Dispositivo](how-to/provision-device.md)
- [Atualizar Firmware](how-to/update-firmware.md)
- [Recuperar Dispositivo](how-to/recover-device.md)
- [Criar Board Profile](how-to/create-board-profile.md)
- [Criar Device Profile](how-to/create-device-profile.md)
- [Release](how-to/release.md)

---

## 📚 Reference (Referência Técnica)

### Arquitetura
- [Architecture](reference/architecture/architecture.md)
- [Firmware](reference/architecture/firmware.md)
- [Platform](reference/architecture/platform.md)
- [Startup](reference/architecture/startup.md)
- [HAL](reference/architecture/hal.md)
- [Board Profile](reference/architecture/board-profile.md)
- [Device Profile](reference/architecture/device-profile.md)
- [Config Manager](reference/architecture/config-manager.md)
- [Storage](reference/architecture/storage.md)

### Protocolos
- [CTAP2](reference/protocols/ctap2.md)
- [WebAuthn](reference/protocols/webauthn.md)
- [USB HID](reference/protocols/usb-hid.md)
- [CCID](reference/protocols/ccid.md)
- [CBOR](reference/protocols/cbor.md)
- [COSE](reference/protocols/cose.md)

### Crypto
- [Crypto](reference/crypto/crypto.md)
- [Keys](reference/crypto/keys.md)
- [Attestation](reference/crypto/attestation.md)
- [RNG](reference/crypto/rng.md)

### Host
- [Python SDK](reference/host/python-sdk.md)
- [CLI](reference/host/cli.md)
- [Configurator](reference/host/configurator.md)
- [Provisioning](reference/host/provisioning.md)

### Boards
- [RP23xx](reference/boards/rp23xx.md)
- [ESP32-S3](reference/boards/esp32s3.md)
- [STM32](reference/boards/stm32.md)
- [nRF](reference/boards/nrf.md)

### API
- [Firmware API](reference/api/firmware.md)
- [Host SDK API](reference/api/host-sdk.md)

---

## 📖 Explanation (Conceitos)

- [Product](explanation/product.md)
- [Roadmap](explanation/roadmap.md)
- [Threat Model](explanation/threat-model.md)
- [Security Principles](explanation/security-principles.md)
- [Universal Firmware](explanation/universal-firmware.md)
- [Provisioning](explanation/provisioning.md)
- [Commissioning](explanation/commissioning.md)
- [Architecture Decisions](explanation/architecture-decisions.md)

---

## 📜 ADRs (Architecture Decision Records)

- [ADR-0001: Universal Firmware](adr/ADR-0001-rust.md)
- [ADR-0002: HAL](adr/ADR-0002-storage.md)
- [ADR-0003: Board Profile](adr/ADR-0003-usb.md)
- [ADR-0004: Device Profile](adr/ADR-0004-unsafe.md)
- [ADR-0005: Provisioning](adr/ADR-0005-sdk.md)
- [ADR-0006: Storage](adr/ADR-0006-build.md)
- [ADR-0007: Crypto](adr/ADR-0007-crypto.md)
- [ADR-0008: Flash Layout](adr/ADR-0008-flash-layout.md)
- [ADR-0009: Versioning](adr/ADR-0009-versioning.md)