# OpenKey 🔑

[![License](https://img.shields.io/badge/License-Apache%202.0%20%7C%20MIT-blue.svg)](LICENSE)
[![Documentation](https://img.shields.io/badge/Docs-Architecture%20%26%20Protocols-green.svg)](docs/README.md)

OpenKey é uma chave de segurança de hardware de código aberto de alta segurança baseada nas especificações **FIDO2**, **CTAP2**, **WebAuthn**, **USB HID** e **CBOR**, desenvolvida em Rust para garantir segurança de memória e robustez criptográfica.

## 📐 Estrutura do Monorepo

```text
openkey/
├── firmware/          # Código Rust embarcado (no_std) para o microcontrolador
├── host/              # Suíte de software host
│   ├── sdk/           # SDK cliente (Rust / Python / C)
│   ├── cli/           # Ferramenta de linha de comando para gerenciamento
│   ├── gui/           # Aplicativo desktop gráfico
│   ├── simulator/     # Simulador de chave FIDO2 em software para testes
│   └── tests/         # Suíte de testes de integração e interoperabilidade
├── docs/              # Arquitetura, segurança, protocolos, APIs e ADRs
├── examples/          # Exemplos de integração e uso
├── fuzz/              # Harnesses de fuzzing para depuradores CTAP2/CBOR
├── hardware/          # Esquemas de hardware, PCB (KiCad) e modelos 3D
├── scripts/           # Scripts de automação, depuração e build
├── tools/             # Ferramentas auxiliares de desenvolvimento e gravação
├── third_party/       # Dependências de fornecedores e HALs
├── .github/           # Workflows de CI/CD e templates
└── (Governança)       # AGENTS.md, SECURITY.md, CONTRIBUTING.md, LICENSE, etc.
```

## 📄 Documentos Fundamentais

- 🌐 [Ecosystem.md](Ecosystem.md) — Visão geral e arquitetura do OpenKey Ecosystem.
- 🎯 [Product.md](Product.md) — Visão do produto, objetivos estratégicos e diferenciais.
- 🚀 [Development Plan.md](Development%20Plan.md) — Plano de desenvolvimento incremental em 12 fases.
- 📋 [spec.md](spec.md) — Especificação técnica funcional e não-funcional.

## 📚 Filosofia de Documentação

Nossa documentação em [`docs/`](docs/README.md) segue uma separação rigorosa de responsabilidades:

- 🏗️ [`docs/architecture/`](docs/architecture/overview.md) — Explica a estrutura interna e como o sistema funciona.
- 🛡️ [`docs/security/`](docs/security/threat-model.md) — Modelos de ameaças, políticas de memória segura e criptografia.
- 🔌 [`docs/protocols/`](docs/protocols/ctap2.md) — Especificações de implementação do FIDO2, CTAP2, WebAuthn, HID e CBOR.
- 🛠️ [`docs/development/`](docs/development/roadmap.md) — Guias de compilação, testes, depuração e publicação.
- 📡 [`docs/api/`](docs/api/firmware.md) — Especificação de APIs do Firmware, SDK, CLI e GUI.
- 📜 [`docs/adr/`](docs/adr/README.md) — Registros das decisões de arquitetura permanentes (ADRs).
- 📖 [`docs/references/`](docs/references/standards.md) — Normas FIDO/W3C/NIST, glossário e bibliografia.

## 🚀 Começando

Para construir o simulador e rodar os testes no seu computador:

```bash
# Clonar o repositório
git clone https://github.com/openkey/openkey.git
cd openkey

# Rodar os testes do host e simulador
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
