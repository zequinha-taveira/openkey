# Integração WebAuthn (`docs/protocols/webauthn.md`)

## 🌐 Alinhamento com W3C WebAuthentication (Level 2 / Level 3)

O OpenKey interage com navegadores modernos (Chrome, Firefox, Safari, Edge) fornecendo respostas estruturadas de objetos de atestação e dados do autenticador (*authenticatorData*).

## 📄 Estrutura do `authenticatorData`

```text
+-----------------------+-------+--------------------+---------------------------+
| RP ID Hash (32 bytes) | Flags | Sign Count (4B)   | Attested Credential Data  |
+-----------------------+-------+--------------------+---------------------------+
```

Flags relevantes:
- Bit 0: User Presence (UP)
- Bit 2: User Verified (UV)
- Bit 6: Attested Credential Data included (AT)
- Bit 7: Extension Data included (ED)
