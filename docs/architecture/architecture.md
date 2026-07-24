# Arquitetura do Framework OpenKey (`docs/architecture/architecture.md`)

**Versão:** 1.0  
**Status:** Aprovado  
**Documentos Relacionados:** [`Product.md`](../../Product.md), [`spec.md`](../../spec.md), [`Development Plan.md`](../../Development%20Plan.md), [`threat-model.md`](../security/threat-model.md), [`security-principles.md`](../security/security-principles.md).

---

## 📌 1. Visão Geral

O **OpenKey** é um **framework open-source universal e modular para autenticadores de hardware** compatíveis com os padrões **FIDO2 / CTAP2.1** e **W3C WebAuthn**.

A premissa arquitetural fundamental do OpenKey é o **desacoplamento total entre o núcleo de segurança (*Security Core*) e a plataforma de hardware**. Toda a lógica de estado, protocolo CTAP2, parsing CBOR, gerenciamento de credenciais e segurança criptográfica reside em módulos agnósticos de hardware (`no_std`). 

O microcontrolador **RP2350** da Raspberry Pi Foundation serve como a **implementação de referência primária em hardware**, enquanto o **Simulador de Software Desktop** atua como alvo oficial de desenvolvimento e fuzzing.

```text
                 Applications
                       │
              Python SDK / CLI / GUI
                       │
          USB • NFC • BLE • Outros
                       │
              Transport Abstraction
                       │
                Protocol Framework
                       │
          CTAP2 • CBOR • COSE • WebAuthn
                       │
                 Security Core
      ┌─────────────────────────────────┐
      │ Credential Manager              │
      │ PIN Manager                     │
      │ Policy Engine                   │
      │ Crypto Abstraction              │
      │ Storage Manager                 │
      └─────────────────────────────────┘
                       │
             Platform Abstraction Layer (PAL)
                       │
    ┌──────────┬──────────┬──────────┬──────────┐
    │ RP2350   │ Software │ STM32    │ nRF52/53 │
    │ (Ref)    │ Simulator│ (Futuro) │ (Futuro) │
    └──────────┴──────────┴──────────┴──────────┘
```

---

## 🎯 2. Princípios Cordeiros da Arquitetura

1. **Hardware é um Detalhe de Implementação**: Módulos de protocolo e segurança não conhecem registradores, SDKs de fabricante ou especificações do MCU.
2. **Protocolos Não Conhecem Hardware**: O motor CTAP2 interage apenas com abstrações de alto nível enviadas pela camada de transporte.
3. **Criptografia Não Conhece Transporte**: A camada de criptografia opera em buffers de memória puros sem conhecimento de onde os dados vieram.
4. **Transporte Não Conhece Armazenamento**: O enquadramento USB HID/CTAPHID processa pacotes sem acessar o banco de credenciais.
5. **Direção Única de Dependências**: As dependências fluem estritamente do topo (Aplicações/Transporte) para a base (PAL/Hardware). O núcleo nunca importa módulos da PAL ou de targets de hardware.
6. **Interfaces Públicas Estáveis**: Abstrações definidas através de traits em Rust garantem desacoplamento entre componentes.

---

## 🧱 3. Organização do Monorepo e Responsabilidade dos Módulos

```text
openkey/
├── firmware/
│   ├── core/                  # Security Core & Protocol Framework (100% Agnóstico no_std)
│   │   ├── ctap/              # Motor CTAP2.1 (GetInfo, MakeCredential, GetAssertion)
│   │   ├── cbor/              # Parser/Serializador CBOR Canônico Estático
│   │   ├── pin/               # ClientPIN Protocol & pinUvAuthToken Manager
│   │   ├── credentials/       # Credential Manager & Resident Key Storage
│   │   └── crypto/            # Traits Criptográficas & Abstrações Constant-Time
│   │
│   ├── transport/             # Camada de Transporte Abstrata (CTAPHID framing)
│   │
│   ├── pal/                   # Platform Abstraction Layer (Traits Rust)
│   │   ├── flash.rs           # Interface de Armazenamento Não-Volátil
│   │   ├── rng.rs             # Interface do TRNG (Hardware/Software)
│   │   ├── usb.rs             # Interface de Transporte USB HID
│   │   └── gpio.rs            # Interface de Presença de Usuário (UP) / LEDs
│   │
│   └── targets/               # Implementações Concretas de Target
│       ├── rp2350/            # Implementação de Referência RP2350 (Hardware)
│       └── simulator/         # Alvo de Execução Local e Fuzzing (Software)
│
├── host/                      # Ecossistema Host (SDK, CLI, GUI)
│   ├── sdk/                   # Python SDK & Rust Bindings
│   ├── cli/                   # Ferramenta openkey-cli
│   └── gui/                   # Aplicação Desktop OpenKey Manager
│
└── docs/                      # Centro de Documentação Arquitetural e de Segurança
```

### Detalhamento das Responsabilidades

#### `firmware/core` (Security Core)
- **Função**: Gerenciar o ciclo de vida do autenticador, máquina de estados CTAP2, serialização CBOR, lógica de PIN e regras de credenciais.
- **Restrição**: Código 100% puro em Rust (`no_std`), livre de dependências de hardware ou bibliotecas específicas de plataforma.

#### `firmware/transport` (Transport Abstraction)
- **Função**: Enquadrar, fragmentar e remontar pacotes `CTAPHID`.
- **Restrição**: Comunica-se com a PAL para receber pacotes brutos USB HID ou de soquetes.

#### `firmware/pal` (Platform Abstraction Layer)
- **Função**: Definir os contratos normativos (traits) para o hardware:
  - `RngProvider`: Geração de números aleatórios com verificação de entropia.
  - `FlashStorageProvider`: Leitura, escrita e apagamento por bloco na Flash.
  - `UserPresenceProvider`: Leitura do estado de confirmação física de presença (botão/touch).
  - `UsbTransportProvider`: Recepção e envio de relatórios USB HID de 64 bytes.

#### `firmware/targets/rp2350` (Plataforma de Referência Primária)
- **Função**: Conectar o `openkey-core` e a `pal` ao hardware real do RP2350 usando o Pico SDK / Rust HAL.
- **Recursos**: TRNG de hardware, Flash Dual-Bank, periférico USB nativo com TinyUSB.

#### `firmware/targets/simulator` (Simulador de Software Oficial)
- **Função**: Fornecer um alvo executável nativo em sistemas operacionais desktop.
- **Recursos**: Armazenamento em arquivo/memória, gerador de entropia de sistema e emulação de transporte USB HID via soquetes locais/IPC para integração com CI e ferramentas de Fuzzing (`cargo-fuzz`).

---

## 🔁 4. Fluxo de Dados e Processamento de Comandos

```text
[ Host Application (Navegador / CLI) ]
                 │
                 ▼ (Relatórios USB HID 64-bytes)
[ Target Driver (RP2350 USB / Simulator Socket) ]
                 │
                 ▼ (Pacotes Brutos via PAL)
[ Transport Layer (CTAPHID Protocol Engine) ]
                 │
                 ▼ (Payload Desfragmentado CTAP2)
[ Security Core: CTAP2 Engine ] ◄──► [ CBOR Parser ]
                 │
                 ├── (Verificação de PIN) ──► [ PIN Manager ]
                 │
                 ├── (Verificação Física) ──► [ PAL User Presence ]
                 │
                 ├── (Operação Cripto) ───► [ Crypto Engine ]
                 │
                 └── (Armazenamento) ────► [ Storage Manager via PAL Flash ]
                 │
                 ▼ (Resposta Assinada CBOR)
[ Host Application ]
```

---

## 🛡️ 5. Fronteiras de Confiança e Isolamento de Segurança

1. **Fronteira de Hardware**: O Host (sistema operacional/navegador) é considerado **potencialmente não confiável**. Nenhuma entrada do host é aceita sem sanitização estrita no CBOR parser.
2. **Fronteira de Memória**: O `Security Core` armazena segredos em estruturas que implementam `Zeroize` no encerramento de escopo. A PAL garante isolamento de Flash via RDP Level 2 e MPU no RP2350.
3. **Fronteira de Execução (`Unsafe Policy`)**: Código `unsafe` é **proibido** no `Security Core`. O uso de `unsafe` é restrito às implementações da PAL em `targets/` e deve conter comentários explicativos `// SAFETY:` em conformidade com o [`ADR-0004`](../adr/ADR-0004-unsafe.md).

---

## 🔌 6. Diretrizes para Adição de Novas Plataformas (Targets)

Toda nova plataforma de hardware (ex: STM32, nRF52/53, ESP32-C6/P4) deve ser adicionada em `firmware/targets/<plataforma>/` implementando **apenas** os contratos da PAL:
- **Flash Storage**: Driver de gravação/leitura de setor com wear-leveling.
- **RNG**: Provedor de entropia de hardware.
- **USB / Transporte**: Driver de relatórios HID.
- **Clock & Timers**: Provedor de tempo monotônico em milissegundos.

> **Garantia de Arquitetura**: Nenhuma alteração no `Security Core` ou no motor CTAP2 é necessária para adicionar um novo microcontrolador ao framework OpenKey.
