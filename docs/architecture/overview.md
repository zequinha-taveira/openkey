# Arquitetura Geral do Framework OpenKey (`docs/architecture/overview.md`)

**Versão:** 1.0  
**Status:** Aprovado  
**Documento Principal:** [`architecture.md`](architecture.md)

---

## 📌 Visão Geral

O **OpenKey** é um **framework open-source universal, modular e agnóstico de hardware** para o desenvolvimento de autenticadores de hardware e chaves de segurança compatíveis com **FIDO2 / CTAP2.1** e **W3C WebAuthn**.

O framework separa rigorosamente a lógica de protocolo e de segurança criptográfica da plataforma de hardware. A implementação para o microcontrolador **RP2350** atua como a **plataforma de referência primária em hardware**, acompanhada pelo **Simulador de Software Desktop** como alvo oficial para testes e fuzzing.

```mermaid
graph TD
    ClientApp[Navegador / Sistema Operacional / Apps Host] -->|WebAuthn / CTAP2 USB HID| HostSDK[OpenKey Host SDK / CLI / GUI]
    HostSDK -->|Relatórios USB HID 64-bytes| TransportLayer[Camada de Transporte CTAPHID / USB / NFC]
    TransportLayer -->|Payloads CTAP2 / CBOR| SecurityCore[Security Core Agnóstico no_std]
    
    subgraph Security Core (openkey-core)
        SecurityCore -->|Validação CBOR| CBORParser[CBOR Static Parser]
        SecurityCore -->|Máquina de Estados| CTAPEngine[CTAP2.1 Protocol Engine]
        SecurityCore -->|Gestão de PIN| PINManager[ClientPIN & Session Token]
        SecurityCore -->|Gestão de Credenciais| CredManager[Credential Manager]
    end
    
    SecurityCore -->|Interfaces Normativas| PAL[Platform Abstraction Layer - PAL]
    
    subgraph Targets (Implementações de Hardware & Software)
        PAL -->|HAL Trait Impl| TargetRP2350[Reference Target: RP2350 Microcontroller]
        PAL -->|PAL Trait Impl| TargetSim[Official Target: Software Simulator]
        PAL -->|PAL Trait Impl| TargetFuture[Future Targets: STM32 / nRF52 / ESP32]
    end
```

---

## 🧩 Camadas e Subsistemas Principais

1. **Security Core (`firmware/core/`)**:
   - Desenvolvido em Rust `no_std`, 100% livre de acoplamento a registradores de hardware.
   - Implementa parsers CBOR canônicos estáticos sem alocação dinâmica na heap.
   - Gerencia estado de credenciais, protocolo ClientPIN, tokens de sessão e abstrações de tempo constante.

2. **Platform Abstraction Layer (`firmware/pal/`)**:
   - Contratos normativos (traits em Rust) definindo abstrações para Flash Storage, TRNG, GPIOs (User Presence) e relatórios USB HID.

3. **Targets & Drivers (`firmware/targets/`)**:
   - `rp2350`: Implementação de referência em hardware para o RP2350 (Pico SDK / Rust HAL).
   - `simulator`: Alvo oficial em software executável em Linux, macOS e Windows para testes e fuzzing.
   - *Plataformas Futuras*: STM32, nRF52/53, ESP32-C6/P4.

4. **Ecossistema Host (`host/`)**:
   - `sdk`: Biblioteca Python e bindings Rust.
   - `cli`: Ferramenta oficial de linha de comando `openkey-cli`.
   - `gui`: Aplicação desktop gráfica **OpenKey Manager**.
