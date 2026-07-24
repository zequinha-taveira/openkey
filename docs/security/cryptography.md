# Especificação de Criptografia (`docs/security/cryptography.md`)

## 🔑 Algoritmos e Parâmetros Criptográficos

- **ECDSA P-256 (secp256r1)**: Chaves assimétricas de credenciais FIDO2 conforme RFC 5480.
- **AES-256-GCM**: Criptografia autenticada para preservação de segredos e credenciais residentes na Flash.
- **HMAC-SHA-256**: Utilizado na extensão `hmac-secret` e na verificação de tokens de autenticação `pinUvAuthToken`.
- **ECDH (Elliptic Curve Diffie-Hellman)**: Troca de chaves e estabelecimento de canal cifrado de curto prazo para proteção do PIN via USB.
