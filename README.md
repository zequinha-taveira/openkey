# OpenKey 🔑

[![License](https://img.shields.io/badge/License-Apache%202.0%20%7C%20MIT-blue.svg)](LICENSE)
[![Documentation](https://img.shields.io/badge/Docs-Architecture%20%26%20Protocols-green.svg)](docs/README.md)

OpenKey é uma chave de segurança de hardware de código aberto de alta segurança baseada nas especificações **FIDO2**, **CTAP2**, **WebAuthn**, **USB HID** e **CBOR**, desenvolvida em Rust para garantir segurança de memória e robustez criptográfica.

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
├── docs/                  # Toda a documentação (tutoriais, how-to, reference, ADRs)
├── examples/              # Exemplos de integração e uso do SDK
├── hardware/              # Esquemas KiCad, PCB e modelos 3D
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

## 📚 Filosofia de Documentação

Nossa documentação em [`docs/`](docs/README.md) segue uma separação rigorosa de responsabilidades:

- 🏗️ [`docs/architecture/`](docs/architecture/) — Explica a estrutura interna e como o sistema funciona.
- 🛡️ [`docs/security/`](docs/security/) — Modelos de ameaças, políticas de memória segura e criptografia.
- 🔌 [`docs/protocols/`](docs/protocols/) — Especificações de implementação do FIDO2, CTAP2, WebAuthn, HID e CBOR.
- 🛠️ [`docs/development/`](docs/development/) — Guias de compilação, testes, depuração e publicação.
- 📡 [`docs/api/`](docs/api/) — Especificação de APIs do Firmware, SDK, CLI e Configurator.
- 📜 [`docs/adr/`](docs/adr/README.md) — Registros das decisões de arquitetura permanentes (ADRs).
- 📖 [`docs/references/`](docs/references/) — Normas FIDO/W3C/NIST, glossário e bibliografia.

## 🚀 Começando

Para construir o simulador e rodar os testes no seu computador:

```bash
# Clonar o repositório
git clone https://github.com/openkey/openkey.git
cd openkey

# Rodar os testes do workspace (simulador incluso)
cargo test --workspace
```

Para mais detalhes sobre ambiente de desenvolvimento, consulte [`docs/development/testing.md`](docs/development/testing.md).

## 🤝 Contribuição e Governança

Consulte nossos guias de participação:
- [Guia de Contribuição](CONTRIBUTING.md)
- [Política de Segurança](SECURITY.md)
- [Diretrizes para Agentes de IA](AGENTS.md)
- [Modelo de Governança](GOVERNANCE.md)
- [Processo de Release](RELEASING.md)

## 📄 Licença

Este projeto é duplamente licenciado sob **Apache License 2.0** e **MIT License**. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.
