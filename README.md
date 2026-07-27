# OpenKey 🔑

[![License](https://img.shields.io/badge/License-Apache%202.0%20%7C%20MIT-blue.svg)](LICENSE)
[![Documentation](https://img.shields.io/badge/Docs-Architecture%20%26%20Protocols-green.svg)](docs/README.md)

OpenKey é um **framework open-source universal** para chaves de segurança FIDO2/WebAuthn, desenvolvido em Rust para garantir segurança de memória e robustez criptográfica. O OpenKey é um projeto de software — não possui nem exige uma placa própria (PCB). Utiliza placas de desenvolvimento existentes como plataformas de execução.

## 📐 Estrutura do Monorepo

```text
openkey/
├── firmware/              # Todo o código embarcado (no_std)
│   ├── core/              # Núcleo de segurança e protocolo CTAP2
│   ├── platform/          # HAL traits, Board/Device Profiles, Configuration Manager
│   │   └── mcu/           # Implementações de MCU (rp23xx, esp32s3, stm32, nrf54)
│   ├── protocols/         # Protocolos CTAP2, CBOR, HID, WebAuthn
│   ├── storage/           # Gerenciamento de armazenamento persistente e wear-leveling
│   ├── crypto/            # Abstrações criptográficas: ECC, SHA, AES, RNG
│   ├── usb/               # Camada de transporte USB HID
│   ├── config/            # Gerenciamento de configuração do firmware
│   └── boot/              # Bootloader e inicialização segura
│
├── boards/                # Perfis de hardware (apenas YAML — sem código Rust)
│   ├── profiles/          # Perfis por família de MCU (rp23xx, esp32s3, stm32, nrf)
│   ├── templates/         # Templates de perfil para novos boards
│   └── examples/          # Exemplos de perfis comentados
│
├── host/                  # Todo software executado no computador
│   ├── sdk-python/        # SDK Python para comunicação com dispositivos OpenKey
│   ├── cli/               # Ferramenta de linha de comando
│   ├── configurator/      # Aplicativo desktop para configuração e gerenciamento
│   ├── provisioner/       # Ferramenta de provisionamento de fábrica
│   ├── updater/           # Atualização segura de firmware (DFU)
│   └── diagnostics/       # Diagnóstico e análise do dispositivo
│
├── tools/                 # Ferramentas internas
│   ├── manufacturing/     # Gravação via SWD/JTAG, injeção de chaves de fábrica
│   ├── migration/         # Scripts de migração de dados entre versões
│   ├── scripts/           # Automação: build, lint, release, Docker
│   ├── generators/        # Geradores de Board Profiles, docs, certificados
│   └── simulator/         # Simulador de software da chave FIDO2
│
├── tests/                 # Testes separados por objetivo
│   ├── unit/              # Testes unitários por crate
│   ├── integration/       # Testes E2E: SDK ↔ Simulador ↔ Firmware
│   ├── interoperability/  # Interoperabilidade com clientes FIDO2 reais
│   ├── hardware/          # Testes que requerem hardware físico
│   └── regression/        # Regressão para bugs conhecidos
│
├── docs/                  # Documentação Diátaxis (tutorials, how-to, reference, explanation)
├── examples/              # Exemplos de integração e uso do SDK
├── fuzz/                  # Harnesses de fuzzing (CBOR, CTAP2, HID)
├── third_party/           # Dependências de fornecedores e HALs
├── cmake/                 # Suporte a build CMake (para integração C/C++)
├── packaging/             # Empacotamento para distribuição (deb, rpm, zip)
├── scripts/               # Scripts de automação da raiz (CI, setup)
└── .github/               # Workflows de CI/CD e templates
```

## 📄 Documentos Fundamentais

- 🌐 [Ecosystem.md](Ecosystem.md) — Visão geral e arquitetura do OpenKey Ecosystem.
- 🎯 [Product.md](Product.md) — Visão do produto, objetivos estratégicos e diferenciais.
- 🚀 [Development Plan.md](Development%20Plan.md) — Plano de desenvolvimento incremental em 12 fases.
- 📋 [spec.md](spec.md) — Especificação técnica funcional e não-funcional.

## 🔌 Hardware de Referência

O OpenKey **não vende hardware**. Utiliza placas comerciais existentes:

| Família | Placas de Referência |
|---------|---------------------|
| **RP23xx** | Pico 2, Pico 2 W, XIAO RP2350, Tiny2350, Feather RP2350 |
| **RP2040** | Pico, Pico W, Tiny2040, XIAO RP2040, Feather RP2040 |
| **Futuro** | ESP32-S3, STM32, nRF52/nRF54 |

Consulte [`docs/explanation/hardware-strategy.md`](docs/explanation/hardware-strategy.md) para a filosofia completa.

## 📚 Documentação (Diátaxis)

Nossa documentação em [`docs/`](docs/README.md) segue o modelo [Diátaxis](https://diataxis.fr/):

- 🎓 [`docs/tutorials/`](docs/tutorials/) — Aprendizado guiado passo a passo.
- 🔧 [`docs/how-to/`](docs/how-to/) — Tarefas práticas: build, flash, provisionar.
- 📚 [`docs/reference/`](docs/reference/) — Referência técnica: arquitetura, protocolos, crypto, APIs, ADRs.
- 📖 [`docs/explanation/`](docs/explanation/) — Conceitos: filosofia, threat model, decisões.
- 📐 [`docs/diagrams/`](docs/diagrams/) — Diagramas de arquitetura (.drawio).

## 🚀 Começando

Para construir o simulador e rodar os testes no seu computador:

```bash
# Clonar o repositório
git clone https://github.com/openkey/openkey.git
cd openkey

# Rodar os testes do workspace (simulador incluso)
cargo test --workspace
```

Para mais detalhes, consulte [`docs/tutorials/getting-started.md`](docs/tutorials/getting-started.md).

## 🤝 Contribuição e Governança

Consulte nossos guias de participação:
- [Guia de Contribuição](CONTRIBUTING.md)
- [Política de Segurança](SECURITY.md)
- [Diretrizes para Agentes de IA](AGENTS.md)
- [Modelo de Governança](GOVERNANCE.md)
- [Processo de Release](RELEASING.md)

## 📄 Licença

Este projeto é duplamente licenciado sob **Apache License 2.0** e **MIT License**. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.
