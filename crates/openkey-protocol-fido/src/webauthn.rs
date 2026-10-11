use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebAuthnError {
    InvalidRpId,
    OutputTooSmall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClientDataHash(pub [u8; 32]);

impl ClientDataHash {
    pub fn compute(client_data_json: &[u8]) -> Self {
        Self(Sha256::digest(client_data_json).into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RpIdHash(pub [u8; 32]);

impl RpIdHash {
    pub fn compute(rp_id: &str) -> Result<Self, WebAuthnError> {
        if rp_id.is_empty() || rp_id.len() > 255 || !rp_id.is_ascii() {
            return Err(WebAuthnError::InvalidRpId);
        }
        Ok(Self(Sha256::digest(rp_id.as_bytes()).into()))
    }
}

pub mod flags {
    pub const USER_PRESENCE: u8 = 0x01;
    pub const USER_VERIFICATION: u8 = 0x04;
    pub const ATTESTED_CREDENTIAL_DATA: u8 = 0x40;
    pub const EXTENSION_DATA: u8 = 0x80;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthenticatorData {
    pub rp_id_hash: RpIdHash,
    pub flags: u8,
    pub sign_count: u32,
}

impl AuthenticatorData {
    pub const ENCODED_LEN: usize = 37;

    pub fn encode(&self, output: &mut [u8]) -> Result<usize, WebAuthnError> {
        if output.len() < Self::ENCODED_LEN {
            return Err(WebAuthnError::OutputTooSmall);
        }
        output[..32].copy_from_slice(&self.rp_id_hash.0);
        output[32] = self.flags;
        output[33..37].copy_from_slice(&self.sign_count.to_be_bytes());
        Ok(Self::ENCODED_LEN)
    }

    pub fn increment(&mut self) {
        self.sign_count = self.sign_count.wrapping_add(1);
    }
}

#[cfg(test)]
mod tests {
    use super::{flags, AuthenticatorData, ClientDataHash, RpIdHash};

    #[test]
    fn calcula_hashes_e_codifica_authenticator_data() {
        let client_hash = ClientDataHash::compute(b"{}");
        assert_ne!(client_hash.0, [0; 32]);
        let rp_id_hash = RpIdHash::compute("example.com").unwrap();
        let mut data = AuthenticatorData {
            rp_id_hash,
            flags: flags::USER_PRESENCE,
            sign_count: 41,
        };
        let mut encoded = [0; AuthenticatorData::ENCODED_LEN];
        data.encode(&mut encoded).unwrap();
        assert_eq!(encoded[32], flags::USER_PRESENCE);
        assert_eq!(&encoded[33..], &41u32.to_be_bytes());
        data.increment();
        assert_eq!(data.sign_count, 42);
    }

    #[test]
    fn rejeita_rp_id_invalido() {
        assert!(RpIdHash::compute("").is_err());
        assert!(RpIdHash::compute("não-é-ascii").is_err());
    }
}
