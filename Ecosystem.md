# OpenKey Ecosystem (`Ecosystem.md`)

**Versão:** 1.0  
**Status:** Rascunho / Aprovado (Draft / Approved)  
**Licença:** Open Source (Apache 2.0 / MIT Dual License)

---

## 🎯 Visão

O **OpenKey Ecosystem** é um conjunto integrado de projetos open source para desenvolvimento, teste, implantação e gerenciamento de autenticadores de hardware e chaves de segurança baseados em padrões abertos (**FIDO2 / CTAP2.1** e **W3C WebAuthn**).

O objetivo principal é fornecer uma plataforma completa e unificada para fabricantes, pesquisadores de segurança, empresas e desenvolvedores — cobrindo desde a implementação do firmware embarcado e da camada de abstração de hardware até ferramentas de gerenciamento desktop/mobile, SDKs de comunicação, infraestrutura de simulação em software e documentação técnica detalhada.

---

## 🧱 Princípios Fundamentais

1. **Open Source First**: Todo o código-fonte, especificações e esquemáticos são 100% abertos e livres de blobs binários proprietários.
2. **Open Standards First**: Conformidade estrita e rastreável com os padrões da FIDO Alliance, W3C, USB-IF, IETF e NIST.
3. **Security by Design**: Segurança incorporada desde a base com Safe Rust no núcleo, falha segura (*fail-closed*), zeroização de memória e tempo constante.
4. **Platform Independent**: O núcleo de segurança é totalmente agnóstico de hardware e desacoplado de fabricantes de silício.
5. **Modular Architecture**: Separação clara de responsabilidades entre transporte, protocolo, lógica de credenciais e abstração de plataforma (PAL).
6. **Test Driven Development**: Desenvolvimento guiado por testes unitários, testes de integração, testes de interoperabilidade e fuzzing.
7. **Documentation First**: Documentação mantida como artefato de primeira classe junto ao código-fonte.
8. **API First**: Interfaces públicas e contratos de API estáveis para firmware, SDKs e ferramentas host.
9. **Interoperability by Default**: Compatibilidade garantida com navegadores modernos, sistemas operacionais e bibliotecas FIDO2 padrão da indústria.

---

## 📐 Visão Geral do Ecossistema

```text
                    OpenKey Ecosystem

                    Documentation
                          │
        ┌─────────────────┼─────────────────┐
        │                 │                 │
     Firmware          Host SDK         Developer Tools
        │                 │                 │
        ├──────────────┬──┴──────────────┐  │
        │              │                 │  │
       CLI            GUI          Simulator│
        │              │                 │  │
        └──────────────┼─────────────────┘  │
                       │                    │
                 Test Framework ────────────┘
                       │
                Continuous Integration (CI/CD)
```

---

## 🧩 Componentes do Ecossistema

### 1. OpenKey Core (`core/`)
Framework universal e agnóstico para autenticadores de hardware desenvolvido em Rust `no_std`.

**Responsável por:**
- Motor de protocolo CTAP2.0 e CTAP2.1.
- Autenticador WebAuthn (Level 2/3).
- Encoding/Decoding CBOR estático canônico.
- Suporte à estrutura de objetos criptográficos COSE.
- Gerenciamento seguro de credenciais residentes e não-residentes.
- Protocolo `authenticatorClientPIN` e gestão de `pinUvAuthToken`.
- Abstração de transporte (CTAPHID / USB HID / NFC).
- Persistência e Wear-Leveling na Flash.
- Primitivas criptográficas e verificação contínua de entropia do TRNG.

---

### 2. OpenKey SDK (`host/sdk`)
Biblioteca oficial para comunicação, automação e integração com dispositivos OpenKey e o Simulador de Software.

**Responsabilidades:**
- Descoberta e enumeração de dispositivos USB HID e leitores PC/SC.
- Comunicação de baixo nível via pacotes CTAPHID.
- Automação de provisionamento e alteração de PIN.
- Atualização segura de firmware via USB.
- Leitura de diagnósticos e logs do dispositivo.
- Automação de testes.

**Linguagem Inicial:** Python (`python-fido2` / `hidapi`).  
*Arquitetura preparada para futuras implementações em Rust nativo, C e JavaScript/TypeScript.*

---

### 3. OpenKey CLI (`host/cli`)
Ferramenta oficial de linha de comando (`openkey-cli`) para administradores, desenvolvedores e usuários avançados.

**Exemplos de Uso:**
- Listar dispositivos conectados e emulados (`openkey-cli list`).
- Obter informações de status e versão do firmware (`openkey-cli info`).
- Atualizar o firmware do dispositivo de forma segura (`openkey-cli update`).
- Executar suítes de testes de diagnóstico (`openkey-cli test`).
- Provisionar chaves e credenciais de teste (`openkey-cli provision`).
- Exportar relatórios de diagnóstico e saúde do hardware (`openkey-cli status`).

---

### 4. OpenKey Manager (`host/gui`)
Aplicação gráfica desktop multiplataforma (Windows, macOS, Linux).

**Funcionalidades:**
- Gerenciamento intuitivo de credenciais residentes (visualizar, listar e remover credenciais).
- Assistente visual de atualização de firmware.
- Painel de diagnósticos e estatísticas do dispositivo.
- Configuração e alteração do PIN do Usuário.
- Visualizador de logs de eventos e pacotes CTAP.
- Ferramenta para testes visuais de interoperabilidade.

---

### 5. OpenKey Simulator (`host/simulator` / `targets/simulator`)
Implementação virtual do firmware executável nativamente em ambiente Desktop.

**Objetivos:**
- Desenvolvimento e depuração sem necessidade de hardware físico.
- Testes unitários e de integração automatizados em pipeline.
- Integração contínua (CI/CD) rápida e confiável.
- Sessões de Fuzzing ostensivo (`cargo-fuzz` / `libFuzzer`) no protocolo CTAP2 e CBOR parser.
- Depuração de código usando ferramentas nativas (lldb/gdb/valgrind/miri).

---

### 6. OpenKey Test Framework (`fuzz/` & `host/tests`)
Infraestrutura completa de testes integrados:
- **Testes de Protocolo**: Validação de conformidade com as especificações CTAP2.0/CTAP2.1.
- **Testes de Interoperabilidade**: Suíte de testes com `libfido2`, `python-fido2` e navegadores.
- **Testes de Regressão**: Garantia de que novas alterações não quebrem funcionalidades existentes.
- **Testes de Desempenho & Latência**: Benchmarks de tempo de resposta criptográfico.
- **Fuzzing de Segurança**: Injeção de pacotes malformados no CBOR e CTAPHID parsers.

---

### 7. OpenKey Documentation (`docs/`)
O centro unificado de documentação oficial do projeto.

**Inclui:**
- **Arquitetura (`docs/architecture/`)**: Visão em camadas, responsabilidade dos módulos e fluxos de dados.
- **Segurança (`docs/security/`)**: Modelo de ameaças (STRIDE), Princípios de Engenharia Segura e política `unsafe`.
- **Especificações (`spec.md`)**: Requisitos funcionais (RF) e não-funcionais (RNF).
- **APIs (`docs/api/`)**: Documentação de referência para firmware, SDKs e ferramentas CLI/GUI.
- **Setup & Plataformas (`docs/setup/`)**: Guias de instalação para Linux, Windows, macOS, Android e iOS.
- **Decisões Arquiteturais (`docs/adr/`)**: Registros permanentes de decisões de engenharia (ADRs).

---

## 💻 Plataformas e Alvos Suportados

### Targets de Hardware (Microcontroladores)
- **RP2350 (Raspberry Pi Foundation)**: Implementação de referência primária em hardware.
- **STM32 (STMicroelectronics)**: Alvo futuro suportado pela PAL.
- **nRF52 / nRF53 (Nordic Semiconductor)**: Alvos futuros para Bluetooth Low Energy (BLE).
- **ESP32-C6 / ESP32-P4 (Espressif)**: Alvos futuros em arquitetura RISC-V.

### Sistemas Operacionais Desktop
- **Linux** (Debian, Ubuntu, Fedora, Arch, Raspberry Pi OS).
- **Windows** (Windows 10 / Windows 11).
- **macOS** (macOS 12+ em Intel e Apple Silicon).

### Plataformas Móveis
- **Android** (via USB OTG e Credential Manager API).
- **iOS** (via USB-C e WebAuthn Framework).

---

## 📂 Estrutura Geral do Monorepo

```text
openkey/
├── core/                      # Núcleo de segurança (Security Core)
├── platform/                  # HAL traits, Board/Device Profiles, Configuration Manager
├── protocols/                 # Protocolos CTAP2, CBOR, HID, WebAuthn
├── storage/                   # Gerenciamento de armazenamento persistente e wear-leveling
├── crypto/                    # Abstrações criptográficas
├── boards/                    # Implementações de board (RP2350, STM32, nRF, ESP32)
├── host/                      # Ecossistema host (sdk, cli, gui, simulator)
│   ├── sdk/                   # OpenKey Python SDK & Bindings
│   ├── cli/                   # Interface de Linha de Comando (openkey-cli)
│   ├── gui/                   # Aplicação Desktop Graphic Manager
│   └── simulator/             # Alvo do Simulador de Software
├── tools/                     # Scripts de diagnóstico e ferramentas de desenvolvimento
├── docs/                      # Centro unificado de documentação técnica
├── examples/                  # Exemplos de uso do SDK e integrações
├── hardware/                  # Esquemáticos e layouts de PCB open hardware
├── fuzz/                      # Harnesses de fuzzing para cargo-fuzz
├── scripts/                   # Scripts de automação de CI/CD e release
└── .github/                   # Workflows de Integração Contínua (GitHub Actions)
```

---

## 🔄 Fluxo de Integração do Ecossistema

```text
               Firmware (no_std) / Simulador
                             │
            USB HID / CTAPHID / Outros Transportes
                             │
                      OpenKey SDK
                             │
      ┌──────────────────────┼──────────────────────┐
      │                      │                      │
   openkey-cli         OpenKey Manager         Scripts / CI
   (Linha Comando)      (Desktop GUI)         (Automação Testes)
```

---

## 🔭 Objetivos de Longo Prazo

- **Framework Reutilizável**: Tornar o OpenKey o padrão open-source para firmware de chaves de segurança.
- **SDKs Nativos Multi-Linguagem**: Expandir os SDKs oficiais para Rust nativo, C/C++ e TypeScript.
- **Ferramentas Desktop Multiplataforma**: Entregar o OpenKey Manager com suporte nativo a instaladores em Linux, Windows e macOS.
- **Simulador de Alta Fidelidade**: Permitir simulação completa de eventos de presença de usuário, interrupções físicas e falhas de energia.
- **Auditoria de Segurança**: Preparar a base de código para auditorias independentes de segurança de terceiros.

---

## 💡 Filosofia de Ecossistema

Cada componente do ecossistema OpenKey é desenhado para ser **independente, bem documentado e reutilizável**. A evolução de um componente específico (ex: melhorias na GUI ou no SDK Python) não exige modificações desnecessárias no *Security Core* ou no firmware embarcado.

Todo o ecossistema compartilha obrigatoriamente:
- Especificações técnicas normativas ([`spec.md`](spec.md)).
- Princípios arquiteturais ([`docs/architecture/architecture.md`](docs/architecture/architecture.md)).
- Princípios de segurança e políticas de código ([`docs/security/security-principles.md`](docs/security/security-principles.md)).
- APIs públicas e convenções de versionamento semântico ([`docs/adr/ADR-0009-versioning.md`](docs/adr/ADR-0009-versioning.md)).
- Infraestrutura de testes automatizados e integração contínua.

---

## 🏁 Missão do Ecossistema

> *Construir um ecossistema aberto, modular e auditável que facilite o desenvolvimento e a adoção de autenticadores compatíveis com padrões abertos, promovendo interoperabilidade, segurança e sustentabilidade a longo prazo.*
