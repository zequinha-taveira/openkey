# Especificação Técnica do Framework OpenKey (`spec.md`)

**Versão:** 1.0.0 (Especificação Normativa)  
**Status:** Aprovado  
**Normas de Referência:** FIDO2 CTAP2.0/CTAP2.1, W3C WebAuthn (Level 2/3), USB HID, ISO/IEC 18013-5, NIST SP 800-90B, RFC 8949 (CBOR), RFC 9052 (COSE).

---

## 📌 1. Objetivo e Escopo

Este documento estabelece a **especificação técnica funcional e não-funcional** do **OpenKey Universal Security Key Framework**.

Todas as implementações do framework — incluindo o `openkey-core`, a *Platform Abstraction Layer* (PAL), a implementação de referência **RP2350** e o **Simulador de Software** — devem obedecer estritamente aos requisitos definidos nesta especificação.

### 1.1 Escopo do Framework
- Protocolo CTAP2.0 e CTAP2.1 (Client-to-Authenticator Protocol).
- Autenticador WebAuthn (W3C WebAuthn Level 2/3).
- Transporte USB HID e abstrações para transportes futuros (NFC, BLE).
- Formato de enquadramento CTAPHID.
- Codificação/Decodificação CBOR canônica estática.
- Armazenamento seguro de credenciais residentes (RK) e não-residentes.
- Protocolo de PIN do Usuário (`authenticatorClientPIN` / `pinUvAuthToken`).
- Mecanismo de Atualização Segura de Firmware (*Signed Firmware Update*).

### 1.2 Fora do Escopo Inicial
- Protocolos legados não-FIDO (OTP, OpenPGP, PIV, OATH).
- Interfaces NFC/BLE no núcleo inicial (suportadas via abstração extensível na PAL).

---

## 🛠️ 2. Requisitos Funcionais (RF)

| ID | Requisito Funcional | Descrição e Critério de Aceitação | Módulo Responsável |
| :--- | :--- | :--- | :--- |
| **RF-001** | Enumeração USB HID | O dispositivo/simulador deve enumerar como classe USB HID utilizando as definições de Usage Page e Usage FIDO (`0xF1D0`). | `pal::usb` / `transport` |
| **RF-002** | Protocolo CTAPHID | Implementar framing CTAPHID com suporte a inicialização de canal (`INIT`), mensagens (`MSG`), ping (`PING`), cancelamento (`CANCEL`) e erro (`ERROR`). | `core::transport` |
| **RF-003** | Comando `authenticatorGetInfo` | Responder ao comando CTAP2 `authenticatorGetInfo` retornando versões suportadas (`FIDO_2_0`, `FIDO_2_1`), extensões, aAGUID, opções e limites de transporte. | `core::ctap` |
| **RF-004** | Comando `authenticatorMakeCredential` | Suportar a criação de novas credenciais WebAuthn, gerando par de chaves assimétricas (ECDSA P-256 / Ed25519) e emitindo o atestado assinado (*attestation statement*). | `core::ctap::make_cred` |
| **RF-005** | Comando `authenticatorGetAssertion` | Suportar autenticação WebAuthn, verificando o `rpIdHash`, confirmando a presença do usuário (UP) ou verificação (UV), incrementando o contador e assinando a afirmação. | `core::ctap::get_assert` |
| **RF-006** | Gestão de PIN (`ClientPIN`) | Implementar o protocolo `authenticatorClientPIN` (V1 e V2), permitindo definição, alteração e verificação de PIN do usuário via troca de chaves ECDH e derivação de `pinUvAuthToken`. | `core::pin` |
| **RF-007** | Gestão de Credenciais Residentes | Armazenar e gerenciar credenciais residentes (Discoverable Credentials), permitindo enumeração e exclusão autorizadas via `authenticatorCredentialManagement`. | `core::storage` |
| **RF-008** | Contador Monotônico de Assinatura | Manter um contador global monotônico persistente incrementado atomicamente a cada assinatura emitida para prevenção de replay e detecção de clonagem. | `core::storage` |
| **RF-009** | Teste de Presença do Usuário (UP) | Exigir confirmação física do usuário (toque no sensor capacitivo/botão no RP2350 ou evento no simulador) para autorizar qualquer operação de criação ou afirmação de credencial. | `pal::user_presence` |
| **RF-010** | Reset de Fábrica (`authenticatorReset`) | Permitir redefinição de fábrica do dispositivo sob confirmação de presença do usuário nos primeiros 10 segundos após a energização, apagando todas as credenciais e hashes de PIN. | `core::ctap::reset` |
| **RF-011** | Atualização Segura de Firmware | Suportar atualização segura de firmware por meio de imagem assinada criptograficamente via bootloader dual-bank na plataforma de hardware. | `pal::bootloader` |
| **RF-012** | Proteção Contra Força Bruta no PIN | Bloquear temporariamente a verificação de PIN após 3 falhas consecutivas e exigir factory reset após 8 falhas acumuladas. | `core::pin` |
| **RF-013** | Suporte a Múltiplos Protocolos no Simulador | O simulador de software deve responder através de soquetes locais/IPC emulando exatamente a interface CTAPHID do hardware. | `targets::simulator` |
| **RF-014** | Formato CBOR Canônico Estático | Todos os enquadramentos de entrada e saída devem validar a especificidade canônica do CBOR (ordenação de chaves, sem inteiros redundantes) sem alocação dinâmica. | `core::cbor` |
| **RF-015** | Diagnósticos e Leitura de Status | Fornecer interfaces de diagnóstico não-sensíveis para leitura de saúde de memória, contadores de uso e versão do firmware. | `core::diagnostics` |

---

## ⚡ 3. Requisitos Não-Funcionais (RNF)

### 3.1 Segurança (RNF-SEC)
- **Safe Rust por Padrão**: Todo o `openkey-core` é escrito em Safe Rust. O uso de `unsafe` é restrito à PAL/HAL e deve obedecer à [`unsafe-policy.md`](docs/security/unsafe-policy.md).
- **Execução em Tempo Constante**: Comparações de segredos, hashes e tokens de sessão devem ser estritamente em tempo constante (`subtle::ConstantTimeEq`).
- **Zeroização de Memória**: Estruturas que mantêm chaves privadas, PINs ou tokens de sessão implementam `Zeroize` no encerramento de escopo.
- **Fail Closed**: Diante de qualquer erro de parsing ou exceção inesperada, a sessão é invalidada imediatamente e o sistema retorna ao estado seguro.

### 3.2 Desempenho (RNF-PERF)
- **Latência de Resposta**: O tempo de processamento de comandos CTAP2 no firmware (excluindo a espera pela resposta física de presença do usuário) deve ser menor que **50ms**.
- **Uso de Memória RAM**: O consumo de SRAM do `openkey-core` deve ser previsível e limitado a no máximo **16 KB**, permitindo execução em microcontroladores restritos.

### 3.3 Portabilidade e Desacoplamento (RNF-PORT)
- **Isenção de Hardware no Core**: O `openkey-core` não pode importar crates de hardware específicas (ex: `rp2040-hal`, `stm32-rs`). Toda a comunicação é feita através das traits da PAL.
- **Suporte Multi-MCU**: O framework deve permitir compilação para arquiteturas **ARM Cortex-M** (`thumbv7em-none-eabihf`), **RISC-V** (`riscv32imac-unknown-none-elf`) e arquiteturas Desktop nativas (`x86_64`, `aarch64`).

---

## 🔄 4. Máquina de Estados do Autenticador

```text
[ ENERGIZAÇÃO / BOOT ]
           │
           ▼
   [ INITIALIZING ] ──(Falha Hardware / TRNG)──► [ FATAL_ERROR (Lockdown) ]
           │
           ▼
       [ READY ] ◄─────────────────────────────────────┐
           │                                           │
  (Recebe Comando CTAP)                                │
           │                                           │
           ▼                                           │
   [ PIN_REQUIRED ] ──(PIN Incorreto < 8x)──► [ RETRY_DELAY ]
           │                                           │
      (PIN Válido)                                     │
           │                                           │
           ▼                                           │
 [ AWAIT_USER_PRESENCE (UP) ] ──(Timeout 30s)──────────┤
           │                                           │
      (Toque / UP OK)                                  │
           │                                           │
           ▼                                           │
     [ PROCESSING ] ──(Concluído / Resposta Enviada)───┘
```

---

## 🗄️ 5. Modelo de Persistência

O armazenamento do OpenKey persiste os seguintes dados na memória não-volátil através da PAL:

1. **Configuração do Dispositivo**: AAGUID, chaves de atestação e flags de estado.
2. **Hash de PIN & Salt**: Derivação criptográfica protegida por PBKDF2/Argon2.
3. **Contador Monotônico Global**: Inteiro de 32 bits incrementado atomicamente.
4. **Credenciais Residentes (RK)**: Tabela de credenciais contendo RP ID Hash, User ID, Credential ID e Chave Privada associada.

> **Regra Cardinal de Persistência**: Dados sensíveis efémeros (`pinUvAuthToken`, chaves ECDH de sessão, buffers de pacotes USB) JAMAIS são gravados na memória não-volátil.

---

## 🧪 6. Critérios de Aceitação e Conformance

Uma funcionalidade do OpenKey é considerada concluída (*Done*) quando:
1. **Implementada**: Código escrito no `openkey-core` ou na PAL correspondente.
2. **Testada**: Possui testes unitários automatizados no simulador e testes de integração.
3. **Audita em Fuzzing**: Parsers de mensagens de entrada aprovados em sessões de fuzzing sem pânicos ou estouros de memória.
4. **Documentada**: Documentação de API em `docs/api/` e especificações atualizadas.
5. **Validada em CI**: Aprovada no pipeline automatizado sem avisos de linter.
