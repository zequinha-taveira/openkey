# Especificação da API Interna do Firmware (`docs/api/firmware.md`)

## 📡 Interfaces e Traços Internos do Firmware

```rust
pub trait AuthenticatorStorage {
    fn read_credential(&mut self, credential_id: &[u8]) -> Result<Credential, StorageError>;
    fn write_credential(&mut self, credential: &Credential) -> Result<(), StorageError>;
}

pub trait CryptoEngine {
    fn generate_keypair(&mut self) -> Result<(PublicKey, PrivateKey), CryptoError>;
    fn sign(&mut self, key: &PrivateKey, message: &[u8]) -> Result<Signature, CryptoError>;
}
```
