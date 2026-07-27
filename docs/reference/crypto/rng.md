# RNG

## Fontes

- **TRNG** - True Random Number Generator (hardware)
- **DRNG** - Deterministic Random Number Generator (software)

## Validação

- Testes de entropia NIST SP 800-90B
- Verificação contínua de saúde
- Backup DRNG em caso de falha

## Uso

- Geração de chaves
- Salt de derivação de PIN
- Nonces de sessão

## Implementação

Trait `RngProvider` em `platform/src/hal/rng.rs`.