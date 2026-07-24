# Subsistema Criptográfico e TRNG (`docs/architecture/crypto.md`)

## 🔑 Primitivas Criptográficas de Hardware

O subsistema criptográfico do OpenKey é isolado e prioriza o uso de aceleradores de hardware (ECC / AES / SHA) integrados ao SoC.

## 🛡️ Algoritmos Suportados

- **ECDSA secp256r1 (P-256)**: Algoritmo padrão exigido pelo FIDO2 (`alg: -7`).
- **Ed25519**: Algoritmo opcional de alta performance (`alg: -8`).
- **ECDH P-256**: Utilizado para o estabelecimento do canal seguro `ClientPin` / `pinUvAuthToken`.
- **AES-256-GCM / HMAC-SHA-256**: Criptografia autenticada de armazenamento e derivação de chaves.
- **TRNG (True Random Number Generator)**: Gerador físico de entropia com verificações de saúde contínuas (NIST SP 800-90B).
