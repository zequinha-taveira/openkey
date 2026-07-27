# Crypto

## Arquitetura

Abstrações criptográficas para o OpenKey.

## Componentes

- **ECC** - Curvas elípticas (P-256, Ed25519)
- **SHA** - Hashes (SHA-256)
- **AES** - Criptografia simétrica
- **RNG** - Geração de números aleatórios

## Princípios

- Execução em tempo constante
- Zeroização de memória
- Validação de parâmetros

## Implementação

Traits em `crypto/` com implementações específicas de hardware.