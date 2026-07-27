# OpenKey — Visão e Objetivos do Produto (`Product.md`)

**Versão:** 1.0  
**Status:** Aprovado  
**Licença:** Open Source (Apache 2.0 / MIT Dual License)

---

## 🎯 1. Visão do Produto

O **OpenKey** é um **framework open-source universal, modular e agnóstico de hardware** para o desenvolvimento de firmware de chaves de segurança e autenticadores de hardware baseados em padrões abertos (**FIDO2 / CTAP2.1** e **W3C WebAuthn**).

O OpenKey desvincula completamente a lógica de protocolo, segurança criptográfica e gerenciamento de credenciais da plataforma de hardware subjacente. A implementação para o microcontrolador **RP2350** da Raspberry Pi Foundation é a **plataforma de referência primária em hardware**, acompanhada pelo **Simulador de Software Desktop** como alvo oficial para desenvolvimento, testes e fuzzing.

### Validade e Valores Fundamentais
1. **Segurança por Padrão**: Safe Rust no núcleo, falha segura (*fail-closed*), zeroização de memória e criptografia de tempo constante.
2. **Independência de Hardware**: O núcleo do protocolo não possui acoplamento a registradores, SDKs proprietários ou vendors de silício.
3. **Auditabilidade & Transparência**: Código 100% aberto, sem blobs binários fechados, com decisões registradas em ADRs (Architecture Decision Records).
4. **Interoperabilidade Estrita**: Conformidade rigorosa com especificações da FIDO Alliance, W3C, USB-IF, IETF e NIST.
5. **Arquitetura Modular**: Camadas de transporte, protocolo, segurança e abstração de plataforma (PAL) com fronteiras estritas.

---

## 🚀 2. Missão e Objetivos Estratégicos

### Missão
Criar uma base tecnológica reutilizável, robusta e universal para autenticadores de hardware, permitindo que fabricantes, pesquisadores, empresas e entusiastas construam dispositivos de segurança confiáveis utilizando o mesmo núcleo de software auditado.

### Objetivos Estratégicos
- **Arquitetura Agnóstica**: Manter o *Security Core* e os parsers de protocolo 100% desacoplados do hardware.
- **Portabilidade Multi-MCU**: Suportar diferentes famílias de microcontroladores (ARM Cortex-M, RISC-V) através da *Platform Abstraction Layer* (PAL).
- **Implementação de Referência RP2350**: Fornecer um firmware completo, de alta performance e pronto para produção para o RP2350.
- **Simulador de Software de Primeira Classe**: Permitir execução local em desenvolvimento, integração contínua (CI) e fuzzing sem necessidade de dispositivo físico.
- **Ecossistema Completo**: Entregar não apenas o firmware, mas também o Host SDK (Python/Rust), ferramentas CLI, interface gráfica GUI e suítes de testes.

---

## ⛔ 3. Não-Objetivos (Out of Scope)

O projeto OpenKey expressamente **não** pretende:
- Depender de bibliotecas ou blobs binários proprietários fechados.
- Copiar arquiteturas fechadas sem respaldo em especificações abertas.
- Implementar funcionalidades fora do escopo de autenticação forte e segurança de credenciais no núcleo do firmware.
- Limitar a arquitetura a um único microcontrolador ou fabricante de hardware.

---

## 👥 4. Público-Alvo

- **Desenvolvedores & Makers**: Para construir suas próprias chaves de segurança personalizadas.
- **Fabricantes de Hardware**: Para integrar um núcleo FIDO2/CTAP2 testado e auditado em seus produtos.
- **Pesquisadores de Segurança**: Para auditar a implementação do protocolo FIDO2, testar vetores de ataque e realizar fuzzing no simulador.
- **Empresas & Instituições**: Para implantar autenticação multifator forte independente de fornecedores proprietários.

---

## 🧩 5. Componentes do Ecossistema OpenKey

Consulte a especificação detalhada do ecossistema em [`Ecosystem.md`](Ecosystem.md).

```
                               OpenKey Ecosystem
                                       │
         ┌─────────────────────────────┼─────────────────────────────┐
         ▼                             ▼                             ▼
+------------------+         +-------------------+         +-------------------+
| Security Core    |         | Host Tools        |         | Targets & HAL     |
| - CTAP2 / CBOR   |         | - OpenKey SDK     |         | - RP2350 Target   |
| - Credential Mgr |         | - OpenKey CLI     |         | - Simulator       |
| - Crypto Engine  |         | - OpenKey Manager |         | - STM32 / nRF /   |
| - PIN Protocol   |         |   (Desktop GUI)   |         |   ESP32 (Futuro)  |
+------------------+         +-------------------+         +-------------------+
```

1. **OpenKey Core Framework (`core/`)**: O núcleo de segurança em Rust `no_std`, cobrindo parsers CBOR canônicos, máquina de estados CTAP2.1, gerenciador de credenciais, política de PIN e abstrações de criptografia.
2. **Platform (`platform/`)**: HAL traits, Board Profile, Device Profile, Configuration Manager e Platform Services.
3. **Reference Target RP2350 (`boards/rp2350/`)**: Firmware de referência utilizando o Raspberry Pi Pico SDK / Rust HAL.
4. **Software Simulator (`host/simulator/`)**: Alvo executável em ambiente desktop (Linux/macOS/Windows) simulando pacotes USB HID e armazenamento.
5. **OpenKey Host SDK (`host/sdk/`)**: SDK Python/Rust para integração, automação, diagnósticos e gerenciamento.
6. **OpenKey CLI (`host/cli/`)**: Ferramenta de linha de comando oficial `openkey-cli`.
7. **OpenKey Manager (`host/gui/`)**: Aplicação desktop gráfica para gerenciamento intuitivo de credenciais e configurações da chave.

---

## 🌟 6. Diferenciais Competitivos

- **Framework Universal**: Código reutilizável entre múltiplos microcontroladores sem duplicar a lógica criptográfica ou de protocolo.
- **Safe Rust First**: Eliminação de categorias inteiras de vulnerabilidades de memória por padrão.
- **Simulador sem Hardware**: Fuzzing avançado e testes automatizados em CI executados diretamente no simulador de software.
- **Conformidade Normativa Rigorosa**: Documentação rastreável através de ADRs, Modelo de Ameaças (STRIDE) e Princípios de Engenharia Segura.

---

## 🗺️ 7. Roadmap de Produto

- **Fase MVP**: Framework Core + Implementação de Referência RP2350 + Simulador de Software básico (CTAP2 GetInfo, MakeCredential, GetAssertion).
- **Fase Beta**: Suporte completo a ClientPIN, Credential Management, Host SDK Python e CLI funcional.
- **Release 1.0 (GA)**: Estabilização das interfaces da PAL, OpenKey Manager GUI, suíte completa de Fuzzing e suporte comunitário a novos alvos (STM32 / nRF52).
- **Certificação FIDO (Futuro)**: Preparação do framework para submissão aos testes formais de certificação FIDO Alliance.
