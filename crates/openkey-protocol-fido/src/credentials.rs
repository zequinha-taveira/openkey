extern crate alloc;

use alloc::{vec, vec::Vec};
use openkey_crypto::{P256KeyPair, RngProvider, Signer};
use sha2::{Digest, Sha256};

use crate::webauthn::{flags, AuthenticatorData, ClientDataHash, RpIdHash, WebAuthnError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CredentialError {
    InvalidRpId,
    InvalidUser,
    CredentialNotFound,
    StorageFull,
    Crypto,
    WebAuthn(WebAuthnError),
    NoPendingAssertion,
}

impl From<WebAuthnError> for CredentialError {
    fn from(error: WebAuthnError) -> Self {
        Self::WebAuthn(error)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MakeCredentialRequest<'a> {
    pub rp_id: &'a str,
    pub client_data_hash: ClientDataHash,
    pub user_handle: &'a [u8],
}

#[derive(Debug, Clone, Copy)]
pub struct GetAssertionRequest<'a> {
    pub rp_id: &'a str,
    pub client_data_hash: ClientDataHash,
    pub credential_id: Option<&'a [u8]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CredentialResult {
    pub credential_id: [u8; 16],
    pub authenticator_data: [u8; AuthenticatorData::ENCODED_LEN],
    pub signature: [u8; 64],
    pub public_key_sec1: Option<[u8; 65]>,
    pub public_key_len: usize,
}

struct StoredCredential {
    id: [u8; 16],
    rp_id_hash: RpIdHash,
    user_handle: Vec<u8>,
    key: P256KeyPair,
    sign_count: u32,
}

pub struct CredentialAuthenticator {
    credentials: Vec<StoredCredential>,
    pending_assertion: Option<usize>,
    max_credentials: usize,
}

impl CredentialAuthenticator {
    pub fn new(max_credentials: usize) -> Self {
        Self {
            credentials: Vec::new(),
            pending_assertion: None,
            max_credentials,
        }
    }

    pub fn make_credential<R: RngProvider>(
        &mut self,
        request: MakeCredentialRequest<'_>,
        rng: &mut R,
    ) -> Result<CredentialResult, CredentialError> {
        let rp_id_hash = RpIdHash::compute(request.rp_id)?;
        if request.user_handle.is_empty() || request.user_handle.len() > 64 {
            return Err(CredentialError::InvalidUser);
        }
        if self.credentials.len() >= self.max_credentials {
            return Err(CredentialError::StorageFull);
        }
        let key = P256KeyPair::generate(rng).map_err(|_| CredentialError::Crypto)?;
        let id = credential_id(
            request.rp_id,
            request.user_handle,
            self.credentials.len() as u32,
        );
        let auth_data = AuthenticatorData {
            rp_id_hash,
            flags: flags::USER_PRESENCE | flags::ATTESTED_CREDENTIAL_DATA,
            sign_count: 0,
        };
        let auth_bytes = encode_signed_data(auth_data, request.client_data_hash);
        let mut signature = [0; 64];
        key.sign(&auth_bytes, &mut signature)
            .map_err(|_| CredentialError::Crypto)?;
        let mut public_key_sec1 = [0; 65];
        let public_key_len = key
            .public_key_sec1(&mut public_key_sec1)
            .map_err(|_| CredentialError::Crypto)?;
        self.credentials.push(StoredCredential {
            id,
            rp_id_hash,
            user_handle: request.user_handle.to_vec(),
            key,
            sign_count: 0,
        });
        Ok(CredentialResult {
            credential_id: id,
            authenticator_data: encode_auth_data(auth_data),
            signature,
            public_key_sec1: Some(public_key_sec1),
            public_key_len,
        })
    }

    pub fn get_assertion(
        &mut self,
        request: GetAssertionRequest<'_>,
    ) -> Result<CredentialResult, CredentialError> {
        let rp_id_hash = RpIdHash::compute(request.rp_id)?;
        let index = self
            .credentials
            .iter()
            .position(|credential| {
                credential.rp_id_hash == rp_id_hash
                    && request.credential_id.is_none_or(|id| id == credential.id)
            })
            .ok_or(CredentialError::CredentialNotFound)?;
        self.pending_assertion = Some(index);
        self.sign_assertion(index, request.client_data_hash)
    }

    pub fn get_next_assertion(
        &mut self,
        client_data_hash: ClientDataHash,
    ) -> Result<CredentialResult, CredentialError> {
        let current = self
            .pending_assertion
            .ok_or(CredentialError::NoPendingAssertion)?;
        let index = current
            .checked_add(1)
            .filter(|index| *index < self.credentials.len())
            .ok_or(CredentialError::NoPendingAssertion)?;
        self.pending_assertion = Some(index);
        self.sign_assertion(index, client_data_hash)
    }

    pub fn reset(&mut self) {
        self.credentials.clear();
        self.pending_assertion = None;
    }

    pub fn user_handle(&self, credential_id: &[u8]) -> Option<&[u8]> {
        self.credentials
            .iter()
            .find(|credential| credential.id == credential_id)
            .map(|credential| credential.user_handle.as_slice())
    }

    fn sign_assertion(
        &mut self,
        index: usize,
        client_data_hash: ClientDataHash,
    ) -> Result<CredentialResult, CredentialError> {
        let credential = self
            .credentials
            .get_mut(index)
            .ok_or(CredentialError::CredentialNotFound)?;
        credential.sign_count = credential.sign_count.wrapping_add(1);
        let auth_data = AuthenticatorData {
            rp_id_hash: credential.rp_id_hash,
            flags: flags::USER_PRESENCE,
            sign_count: credential.sign_count,
        };
        let auth_bytes = encode_signed_data(auth_data, client_data_hash);
        let mut signature = [0; 64];
        credential
            .key
            .sign(&auth_bytes, &mut signature)
            .map_err(|_| CredentialError::Crypto)?;
        Ok(CredentialResult {
            credential_id: credential.id,
            authenticator_data: encode_auth_data(auth_data),
            signature,
            public_key_sec1: None,
            public_key_len: 0,
        })
    }
}

fn credential_id(rp_id: &str, user_handle: &[u8], ordinal: u32) -> [u8; 16] {
    let mut hasher = Sha256::new();
    hasher.update(rp_id.as_bytes());
    hasher.update(user_handle);
    hasher.update(ordinal.to_be_bytes());
    let digest = hasher.finalize();
    let mut id = [0; 16];
    id.copy_from_slice(&digest[..16]);
    id
}

fn encode_auth_data(data: AuthenticatorData) -> [u8; AuthenticatorData::ENCODED_LEN] {
    let mut encoded = [0; AuthenticatorData::ENCODED_LEN];
    data.encode(&mut encoded)
        .expect("fixed authenticator data buffer");
    encoded
}

fn encode_signed_data(data: AuthenticatorData, client_data_hash: ClientDataHash) -> Vec<u8> {
    let mut result = vec![0; AuthenticatorData::ENCODED_LEN + client_data_hash.0.len()];
    result[..AuthenticatorData::ENCODED_LEN].copy_from_slice(&encode_auth_data(data));
    result[AuthenticatorData::ENCODED_LEN..].copy_from_slice(&client_data_hash.0);
    result
}

#[cfg(test)]
mod tests {
    use super::{CredentialAuthenticator, GetAssertionRequest, MakeCredentialRequest};
    use crate::webauthn::ClientDataHash;
    use openkey_crypto::RngProvider;

    struct CounterRng(u8);
    impl RngProvider for CounterRng {
        type Error = ();
        fn fill_bytes(&mut self, output: &mut [u8]) -> Result<(), Self::Error> {
            output.fill(self.0);
            self.0 = self.0.wrapping_add(1);
            Ok(())
        }
    }

    #[test]
    fn make_e_get_assertion_assinam_e_incrementam_contador() {
        let mut authenticator = CredentialAuthenticator::new(4);
        let mut rng = CounterRng(7);
        let client_hash = ClientDataHash::compute(b"client");
        let created = authenticator
            .make_credential(
                MakeCredentialRequest {
                    rp_id: "example.com",
                    client_data_hash: client_hash,
                    user_handle: b"user",
                },
                &mut rng,
            )
            .unwrap();
        let assertion = authenticator
            .get_assertion(GetAssertionRequest {
                rp_id: "example.com",
                client_data_hash: client_hash,
                credential_id: Some(&created.credential_id),
            })
            .unwrap();
        assert_eq!(assertion.credential_id, created.credential_id);
        assert_eq!(&assertion.authenticator_data[33..], &1u32.to_be_bytes());
        assert!(assertion.public_key_sec1.is_none());
    }

    #[test]
    fn reset_apaga_credenciais() {
        let mut authenticator = CredentialAuthenticator::new(1);
        authenticator.reset();
        assert!(authenticator
            .get_next_assertion(ClientDataHash::compute(b"x"))
            .is_err());
    }
}
