# Chaves

## Tipos de Chaves

### RSA
- Tamanhos suportados: 2048, 3072, 4096 bits

### ECDSA
- Curva P-256 (NIST)
- Curva P-384 (NIST)
- Curva P-521 (NIST)

### EdDSA
- Ed25519

## Geração

- Gerada usando RNG hardware (TRNG)
- Armazenamento seguro na Flash
- Backup opcional via attestation

## Uso

- Assinaturas para credenciais
- Derivação de chaves de sessão
- Prova de posse