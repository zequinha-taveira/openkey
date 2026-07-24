# ADR-0007: Seleção da Suíte Criptográfica e Validação de Hardware TRNG

- **Status**: Aceito
- **Data**: 2026-07-24
- **Autores**: Equipe de Criptografia

## 📌 Contexto

O padrão FIDO2 especifica suporte a algoritmos ECC (P-256 e Ed25519) e requer geração de números aleatórios de alta entropia para proteção contra ataques de predição de chaves.

## 💡 Decisão

Adotaremos aceleradores criptográficos de hardware do microcontrolador e implementações em tempo constante auditadas em Rust (`p256`, `ed25519-dalek`, `aes-gcm`). A entropia do TRNG de hardware será submetida contínuamente aos testes de validação **NIST SP 800-90B**.

## 🚀 Consequências

### Positivas
- Resistência contra ataques de análise de potência e canal lateral por tempo.
- Alta velocidade de geração e assinatura de credenciais no dispositivo.

### Compromissos (Trade-offs)
- Exige validação minuciosa dos registros do TRNG a cada inicialização de hardware.
