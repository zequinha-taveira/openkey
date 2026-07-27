# Attestation

## Tipos

- **None** - Sem atestado
- **Self** - AutowebAuthn
- **Basic** - Attestation básica
- **AttCA** - Attestation com CA
- **EntityCA** - Attestation com Entity CA

## Fluxo

1. Geração de par de chaves
2. Criação de estrutura de atestado
3. Assinatura com chave privada
4. Envio ao host

## Formato

CBOR estruturado conforme WebAuthn Level 3.