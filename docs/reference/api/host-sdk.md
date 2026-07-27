# API do Host SDK

## Python SDK

```python
class SecurityKey:
    def get_info(self) -> DeviceInfo
    def make_credential(self, rp, user) -> Credential
    def get_assertion(self, rp_id, allow_credentials) -> Assertion
    def set_pin(self, pin) -> bool
    def reset(self) -> bool
```

## Rust SDK

```rust
pub struct SecurityKey {
    pub fn new() -> Self
    pub fn get_info(&mut self) -> Result<DeviceInfo, Error>
    pub fn make_credential(&mut self, request: MakeCredentialRequest) -> Result<MakeCredentialResponse, Error>
    pub fn get_assertion(&mut self, request: GetAssertionRequest) -> Result<GetAssertionResponse, Error>
}
```

## Tipos Principais

- `DeviceInfo` - Informações do dispositivo
- `Credential` - Credencial WebAuthn
- `Assertion` - Afirmação de autenticação
- `Error` - Tipos de erro