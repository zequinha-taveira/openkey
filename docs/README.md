# Documentação do OpenKey 📖

Bem-vindo ao centro de documentação oficial do monorepo **OpenKey**. Nossa documentação é estritamente organizada por domínios de conhecimento para garantir clareza, manutenibilidade e rastreabilidade técnica.

---

## 🧭 Mapa da Documentação

```text
docs/
├── setup/                   # Guia de configuração de ambiente host, drivers, OS e builds
├── architecture/            # Explica como o sistema funciona por dentro
├── security/                # Explica como o sistema permanece seguro
├── protocols/               # Explica como FIDO2, CTAP2, WebAuthn, HID e CBOR são implementados
├── development/             # Explica como desenvolver, testar, depurar e publicar
├── api/                     # Documenta as APIs públicas do firmware, SDK, CLI e GUI
├── diagrams/                # Diagramas de arquitetura, sequências e ameaças (.drawio)
├── adr/                     # Registra todas as decisões arquiteturais permanentes (ADRs)
└── references/              # Referências a normas (FIDO, W3C, NIST), glossário e bibliografia
```

---

## 💻 0. Host Setup & Plataformas (`docs/setup/`)
Guias de ambiente de desenvolvimento, drivers USB, toolchains e plataformas.
- [Índice Geral de Setup](setup/README.md)
- [Guia de Setup — Linux](setup/linux.md)
- [Guia de Setup — Windows](setup/windows.md)
- [Guia de Setup — macOS](setup/macos.md)
- [Guia de Setup — Android](setup/android.md)
- [Guia de Setup — iOS](setup/ios.md)
- [Setup da Toolchain Rust](setup/rust.md)
- [Setup da Toolchain Python](setup/python.md)
- [Setup CMake & Pico SDK](setup/cmake.md)
- [Drivers USB e Política VID/PID](setup/usb-drivers.md)
- [Solução de Problemas & Diagnósticos](setup/troubleshooting.md)
- [Guia de Compilação Unificado](setup/build.md)

---

## 🏗️ 1. Arquitetura (`docs/architecture/`)
Explica a visão geral do sistema e subsistemas de hardware/firmware/host.
- [Visão Geral de Arquitetura](architecture/overview.md)
- [Arquitetura do Firmware (`no_std`)](architecture/firmware.md)
- [Arquitetura do Host SDK](architecture/host-sdk.md)
- [Camada de Transporte (USB HID / NFC)](architecture/transport.md)
- [Armazenamento Seguro na Flash](architecture/storage.md)
- [Arquitetura Criptográfica e TRNG](architecture/crypto.md)
- [Sistema de Build Reproduzível](architecture/build.md)

---

## 🛡️ 2. Segurança (`docs/security/`)
Explica os modelos de ameaça, defesas criptográficas e políticas de código.
- [Modelo de Ameaças (STRIDE)](security/threat-model.md)
- [Princípios de Segurança](security/security-principles.md)
- [Práticas de Desenvolvimento Seguro](security/secure-development.md)
- [Política Estrita para Código `unsafe`](security/unsafe-policy.md)
- [Primitivas e Algoritmos Criptográficos](security/cryptography.md)
- [Gerenciamento e Divulgação de Vulnerabilidades](security/vulnerability-management.md)

---

## 🔌 3. Protocolos (`docs/protocols/`)
Explica a implementação dos padrões internacionais de autenticação.
- [Especificação de Implementação CTAP2.0 / CTAP2.1](protocols/ctap2.md)
- [Integração WebAuthn (Level 2 / Level 3)](protocols/webauthn.md)
- [Framing e Relatórios USB HID](protocols/hid.md)
- [Codificação / Decodificação CBOR Canônica](protocols/cbor.md)
- [Máquina de Estados de Protocolo](protocols/protocol-state-machine.md)

---

## 🛠️ 4. Desenvolvimento (`docs/development/`)
Guias práticos para desenvolvedores, testadores e mantenedores.
- [Roadmap do Projeto](development/roadmap.md)
- [Estratégia de Testes e Fuzzing](development/testing.md)
- [Matriz de Interoperabilidade de Browsers e Sistemas OS](development/interoperability.md)
- [Guia de Estilo de Código](development/coding-style.md)
- [Como Contribuir](development/contributing.md)
- [Depuração e Hardware-in-the-Loop (HIL)](development/debugging.md)
- [Integração Contínua (CI)](development/ci.md)
- [Processo de Release e Empacotamento](development/release.md)

---

## 📡 5. Especificações de API (`docs/api/`)
Documentação de referência de interfaces e contratos de chamadas.
- [API Interna do Firmware](api/firmware.md)
- [API do SDK Python / Rust](api/python-sdk.md)
- [Interface CLI (`openkey-cli`)](api/cli.md)
- [Interface Desktop GUI (`openkey-gui`)](api/gui.md)

---

## 📐 6. Diagramas (`docs/diagrams/`)
Diagramas de arquitetura editáveis em formato `.drawio`:
- [Visão Geral de Arquitetura](diagrams/architecture.drawio)
- [Fluxo de Protocolo CTAP](diagrams/ctap.drawio)
- [Framing USB HID](diagrams/usb.drawio)
- [Subsistema de Storage Flash](diagrams/storage.drawio)
- [Layout de Memória Flash](diagrams/flash.drawio)
- [Sequência de Boot Seguro](diagrams/boot.drawio)
- [Diagrama do Modelo de Ameaça](diagrams/threat-model.drawio)

---

## 📜 7. Registros de Decisão de Arquitetura (`docs/adr/`)
Histórico imutável de decisões de engenharia.
- [Índice Geral de ADRs](adr/README.md)
- [ADR-0001: Seleção da Linguagem Rust para Firmware](adr/ADR-0001-rust.md)
- [ADR-0002: Armazenamento Seguro e Wear-Leveling](adr/ADR-0002-storage.md)
- [ADR-0003: Pilha USB HID Customizada](adr/ADR-0003-usb.md)
- [ADR-0004: Isolamento e Auditoria de Código `unsafe`](adr/ADR-0004-unsafe.md)
- [ADR-0005: Arquitetura do Host SDK e Bindings Multi-linguagem](adr/ADR-0005-sdk.md)
- [ADR-0006: Pipeline de Compilação Reproduzível](adr/ADR-0006-build.md)
- [ADR-0007: Escolha da Suíte Criptográfica e TRNG Hardware](adr/ADR-0007-crypto.md)
- [ADR-0008: Layout da Memória Flash e Bootloader Dual-Bank](adr/ADR-0008-flash-layout.md)
- [ADR-0009: Política de Versionamento do Monorepo](adr/ADR-0009-versioning.md)

---

## 📖 8. Referências (`docs/references/`)
Padrões externos, literatura e definições.
- [Padrões e Normas Técnicas](references/standards.md)
- [Bibliografia Criptográfica e Acadêmica](references/bibliography.md)
- [Glossário de Termos e Acrônimos](references/glossary.md)
