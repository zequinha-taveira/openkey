# Documentação do OpenKey 📖

Bem-vindo ao centro de documentação oficial do monorepo **OpenKey** (Versão Estável `v1.0.0`).

A documentação segue o modelo [Diátaxis](https://diataxis.fr/):
**Tutorials** → **How-to** → **Reference** → **Explanation**.

---

## 🧭 Mapa da Documentação

```text
docs/
│
├── README.md                 # Índice da documentação
│
├── tutorials/                # Aprendizado
│   ├── getting-started.md
│   ├── first-build.md
│   ├── first-provisioning.md
│   ├── first-security-key.md
│   └── add-new-board.md
│
├── how-to/                   # Tarefas
│   ├── build-rp23xx.md
│   ├── flash-firmware.md
│   ├── provision-device.md
│   ├── update-firmware.md
│   ├── recover-device.md
│   ├── create-board-profile.md
│   ├── create-device-profile.md
│   └── release.md
│
├── reference/                # Referência técnica
│   ├── architecture/
│   ├── protocols/
│   ├── crypto/
│   ├── host/
│   ├── boards/
│   ├── api/
│   └── adr/
│
├── explanation/              # Conceitos
│   ├── product.md
│   ├── development-plan.md
│   ├── threat-model.md
│   ├── security-principles.md
│   ├── universal-firmware.md
│   ├── provisioning.md
│   ├── commissioning.md
│   ├── architecture-decisions.md
│   └── roadmap.md
│
└── diagrams/
    ├── architecture.drawio
    ├── provisioning.drawio
    ├── storage.drawio
    ├── usb.drawio
    └── startup.drawio
```

---

## 🎓 Tutorials (Aprendizado)

Ensinam alguém a aprender. Guiam passo a passo do zero até um resultado funcional.

- [Getting Started](tutorials/getting-started.md)
- [Primeiro Build](tutorials/first-build.md)
- [Primeiro Provisionamento](tutorials/first-provisioning.md)
- [Primeira Chave de Segurança](tutorials/first-security-key.md)
- [Adicionar Novo Board](tutorials/add-new-board.md)

---

## 🔧 How-to (Tarefas)

Mostram como executar uma tarefa específica. Pressupõem conhecimento básico.

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

Documentação técnica precisa. Descreve APIs, estruturas, protocolos e componentes.

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

### Criptografia
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

### ADR (Architecture Decision Records)
- [Índice de ADRs](reference/adr/README.md)

### Desenvolvimento
- [Git Branch Strategy](development/git-branch-strategy.md)

---

## 📖 Explanation (Conceitos)

Explica o "porquê" por trás das decisões. Discute contexto, filosofia e trade-offs.

- [Product](explanation/product.md)
- [Development Plan](explanation/development-plan.md)
- [Threat Model](explanation/threat-model.md)
- [Security Principles](explanation/security-principles.md)
- [Universal Firmware](explanation/universal-firmware.md)
- [Provisioning](explanation/provisioning.md)
- [Commissioning](explanation/commissioning.md)
- [Architecture Decisions](explanation/architecture-decisions.md)
- [Roadmap](explanation/roadmap.md)

---

## 📐 Diagramas

Diagramas de arquitetura e fluxos editáveis em formato `.drawio`:

- [Architecture](diagrams/architecture.drawio)
- [Provisioning](diagrams/provisioning.drawio)
- [Storage](diagrams/storage.drawio)
- [USB](diagrams/usb.drawio)
- [Startup](diagrams/startup.drawio)