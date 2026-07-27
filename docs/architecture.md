# Arquitetura do Framework OpenKey

**Versão:** 1.0  
**Status:** Aprovado

---

## 📌 Visão Geral

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

## 🧱 Organização do Monorepo

```text
openkey/
├── core/                           # Security Core & Protocol Framework
├── platform/                       # HAL traits, Board/Device Profiles, Config Manager
├── protocols/                      # CTAP2, CBOR, HID, WebAuthn
├── storage/                        # Persistent storage & wear-leveling
├── crypto/                         # Crypto abstractions: ECC, SHA, AES, RNG
├── boards/                         # Board implementations (RP2350, STM32, nRF, ESP32)
├── host/                           # Host ecosystem (SDK, CLI, GUI, Simulator)
├── tools/                          # Development tools
├── docs/                           # Documentation
└── ...
```

---

## 🔒 Segurança

Veja [security-principles.md](security-principles.md) e [threat-model.md](threat-model.md).