//! Módulo de integração WebAuthn (W3C WebAuthn Level 2/3)

use crate::cbor::{CborEncoder, Result};

/// Flag de presença do usuário (UP)
pub const WEBAUTHN_FLAG_UP: u8 = 0x01;
/// Flag de verificação do usuário (UV)
pub const WEBAUTHN_FLAG_UV: u8 = 0x04;
/// Flag de credencial atestada incluída (AT)
pub const WEBAUTHN_FLAG_AT: u8 = 0x40;
/// Flag de extensões incluídas (ED)
pub const WEBAUTHN_FLAG_ED: u8 = 0x80;

/// Dados do Autenticador (`authData`) conforme W3C WebAuthn Section 6.1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticatorData<'a> {
    /// Hash SHA-256 do Relying Party ID (rpIdHash) — 32 bytes
    pub rp_id_hash: [u8; 32],
    /// Flags de estado (UP, UV, AT, ED)
    pub flags: u8,
    /// Contador monotônico de assinatura (signCount)
    pub sign_count: u32,
    /// Dados da credencial atestada (se flag AT estiver ativa)
    pub attested_credential_data: Option<AttestedCredentialData<'a>>,
}

/// Dados da Credencial Atestada contidos no `authData`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttestedCredentialData<'a> {
    /// AAGUID do dispositivo — 16 bytes
    pub aaguid: [u8; 16],
    /// Identificador único da credencial
    pub credential_id: &'a [u8],
    /// Chave pública da credencial em formato COSE Key CBOR
    pub credential_public_key: &'a [u8],
}

impl<'a> AuthenticatorData<'a> {
    /// Constrói a representação binária bruta do `authData` em `out_buf`
    /// Formato: `rpIdHash (32b) || flags (1b) || signCount (4b) [|| aaguid (16b) || credIdLen (2b) || credId || credPubKey]`
    pub fn serialize(&self, out_buf: &mut [u8]) -> Result<usize> {
        let min_len = 37;
        if out_buf.len() < min_len {
            return Err(crate::cbor::CborError::BufferTooSmall);
        }

        let mut pos = 0;
        // 1. rpIdHash (32 bytes)
        out_buf[pos..pos + 32].copy_from_slice(&self.rp_id_hash);
        pos += 32;

        // 2. flags (1 byte)
        out_buf[pos] = self.flags;
        pos += 1;

        // 3. signCount (4 bytes BE)
        out_buf[pos..pos + 4].copy_from_slice(&self.sign_count.to_be_bytes());
        pos += 4;

        // 4. Attested Credential Data (se presente)
        if let Some(ref att) = self.attested_credential_data {
            let cred_id_len = att.credential_id.len();
            let total_needed = pos + 16 + 2 + cred_id_len + att.credential_public_key.len();
            if out_buf.len() < total_needed {
                return Err(crate::cbor::CborError::BufferTooSmall);
            }

            // aaguid (16 bytes)
            out_buf[pos..pos + 16].copy_from_slice(&att.aaguid);
            pos += 16;

            // credentialIdLength (2 bytes BE)
            out_buf[pos..pos + 2].copy_from_slice(&(cred_id_len as u16).to_be_bytes());
            pos += 2;

            // credentialId
            out_buf[pos..pos + cred_id_len].copy_from_slice(att.credential_id);
            pos += cred_id_len;

            // credentialPublicKey (COSE Key CBOR bytes)
            out_buf[pos..pos + att.credential_public_key.len()]
                .copy_from_slice(att.credential_public_key);
            pos += att.credential_public_key.len();
        }

        Ok(pos)
    }
}

/// Codifica uma chave pública ECDSA P-256 no formato COSE_Key (CBOR Map)
/// COSE Key Keys (RFC 9052 / CTAP2):
/// 1 (kty): 2 (EC2)
/// 3 (alg): -7 (ES256)
/// -1 (crv): 1 (P-256)
/// -2 (x): 32 bytes de coordenada X
/// -3 (y): 32 bytes de coordenada Y
pub fn encode_p256_cose_key(x: &[u8; 32], y: &[u8; 32], out_buf: &mut [u8]) -> Result<usize> {
    let mut enc = CborEncoder::new(out_buf);
    enc.encode_map_header(5)?;

    // 1 (kty): 2 (EC2)
    enc.encode_int(1)?;
    enc.encode_int(2)?;

    // 3 (alg): -7 (ES256)
    enc.encode_int(3)?;
    enc.encode_int(-7)?;

    // -1 (crv): 1 (P-256)
    enc.encode_int(-1)?;
    enc.encode_int(1)?;

    // -2 (x): 32 bytes
    enc.encode_int(-2)?;
    enc.encode_bytes(x)?;

    // -3 (y): 32 bytes
    enc.encode_int(-3)?;
    enc.encode_bytes(y)?;

    Ok(enc.position())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_data_serialization() {
        let auth_data = AuthenticatorData {
            rp_id_hash: [0xaa; 32],
            flags: WEBAUTHN_FLAG_UP | WEBAUTHN_FLAG_UV,
            sign_count: 42,
            attested_credential_data: None,
        };

        let mut buf = [0u8; 64];
        let len = auth_data.serialize(&mut buf).unwrap();

        assert_eq!(len, 37);
        assert_eq!(&buf[0..32], &[0xaa; 32]);
        assert_eq!(buf[32], 0x05); // UP (1) | UV (4) = 5
        assert_eq!(&buf[33..37], &42u32.to_be_bytes());
    }

    #[test]
    fn test_encode_p256_cose_key() {
        let x = [0x11; 32];
        let y = [0x22; 32];
        let mut buf = [0u8; 128];
        let len = encode_p256_cose_key(&x, &y, &mut buf).unwrap();

        assert!(len > 0);
        let mut dec = crate::cbor::CborDecoder::new(&buf[..len]);
        dec.decode_map_canonical(|entry_dec| {
            entry_dec.skip_value()?;
            entry_dec.skip_value()?;
            Ok(())
        })
        .unwrap();
    }
}
