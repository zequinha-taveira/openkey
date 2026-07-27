# ADR-0012: Limite Crypto/Platform para Configuração Autenticada

- **Status**: Aceito
- **Data**: 2026-07-27

## Contexto

Platform precisa usar criptografia para persistir configuração, mas Core já
depende de Platform. A dependência anterior de Crypto em Core criaria um ciclo.

## Decisão

Crypto não depende de Core nem Platform e encapsula AES-256-GCM do RustCrypto.
Platform define `ConfigKeyProvider`, obtém nonce do TRNG e gerencia Flash. A
chave efêmera e os buffers temporários são zeroizados.

## Consequências

- Não há chave fixa no firmware.
- Providers de provisionamento ou Secure Element devem implementar a interface.
- Configuração v1 não autenticada é rejeitada.
