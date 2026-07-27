# Modelo de Ameaças do OpenKey

## 🎯 Escopo

Este documento descreve o modelo de ameaças (STRIDE) aplicado ao OpenKey Framework, identificando os riscos potenciais e as defesas implementadas.

## 📋 Ameaças Identificadas

| Categoria | Ameaça | Defesa |
|-----------|--------|--------|
| **Spoofing** | Engenharia social para obter presença de usuário | Botão físico dedicado, verificação de usuário (UV) |
| **Tampering** | Modificação do firmware | Assinaturas digitais, bootloader dual-bank |
| **Repudiation** | Negação de operações | Contador de assinatura monotônico |
| **Information Disclosure** | Exfiltração de chaves privadas | Armazenamento seguro, proteção contra side-channels |
| **Denial of Service** | Bloqueio do dispositivo | Watchdog, timeout de PIN |
| **Elevation of Privilege** | Escalonamento de privilégios | Políticas de segurança rigorosas |

## 🔒 Defesas Implementadas

### Hardware
- TRNG para geração de números aleatórios
- Watchdog para detecção de travamentos
- Flash protegida contra leitura não autorizada

### Software
- Verificação de presença de usuário obrigatória
- Contador de tentativas de PIN com bloqueio
- Criptografia com curvas elípticas modernas (P-256, Ed25519)

## 📖 Documentação Relacionada

- [Security Principles](security-principles.md)
- [Cryptography](security/cryptography.md)
- [ADR-0004: Isolamento e Auditoria de Código `unsafe`](adr/ADR-0004-unsafe.md)