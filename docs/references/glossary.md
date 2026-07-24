# Glossário de Termos e Acrônimos (`docs/references/glossary.md`)

## 📖 Glossário

- **CBOR**: *Concise Binary Object Representation* — Formato binário de serialização de dados compacto definido pela RFC 8949.
- **COSE**: *CBOR Object Signing and Encryption* — Padrão para assinaturas e criptografia sobre dados codificados em CBOR (RFC 9052).
- **CTAP / CTAP2**: *Client to Authenticator Protocol* — Protocolo que permite que um cliente (ex: browser) se comunique diretamente com o autenticador de hardware.
- **DFU**: *Device Firmware Update* — Processo de atualização segura do firmware do dispositivo no campo.
- **FIDO2**: Conjunto de especificações abertas (CTAP2 + WebAuthn) para autenticação forte sem senha.
- **HIL**: *Hardware-in-the-Loop* — Suíte de testes automatizados executando em hardware físico real.
- **RP / RP ID**: *Relying Party / Relying Party Identifier* — O serviço/website que solicita a autenticação (ex: `google.com`, `github.com`).
- **RK / Discoverable Credential**: *Resident Key* — Credencial FIDO2 armazenada diretamente na memória Flash do dispositivo sem depender de alça externa (*credential ID handle*).
- **TRNG**: *True Random Number Generator* — Gerador de números aleatórios físicos baseado em ruído térmico/quântico de hardware.
- **UP**: *User Presence* — Verificação de que um humano está fisicamente presente (ex: toque no botão da chave).
- **UV**: *User Verification* — Verificação da identidade do usuário (ex: validação de PIN ou biometria).
- **WebAuthn**: *Web Authentication* — API padrão do W3C incorporada nos navegadores para autenticação com chave de segurança.
