#![no_std]

use p256::ecdsa::{
    signature::hazmat::{PrehashSigner, PrehashVerifier},
    Signature, SigningKey, VerifyingKey,
};
use sha2::{Digest, Sha256};

/// Erros comuns da camada criptográfica.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CryptoError {
    Unsupported,
    InvalidInput,
    OutputTooSmall,
    Randomness,
}

/// Fonte de aleatoriedade fornecida pelo target ou pelo simulador.
pub trait RngProvider {
    type Error;

    fn fill_bytes(&mut self, output: &mut [u8]) -> Result<(), Self::Error>;
}

/// Contrato de assinatura sem acoplar o core a uma biblioteca criptográfica.
pub trait Signer {
    fn sign(&self, message: &[u8], output: &mut [u8]) -> Result<usize, CryptoError>;
}

pub trait Verifier {
    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<(), CryptoError>;
}

pub struct P256KeyPair {
    signing_key: SigningKey,
}

impl P256KeyPair {
    pub fn generate<R: RngProvider>(rng: &mut R) -> Result<Self, CryptoError> {
        let mut secret = [0u8; 32];
        rng.fill_bytes(&mut secret)
            .map_err(|_| CryptoError::Randomness)?;
        let signing_key =
            SigningKey::from_bytes((&secret).into()).map_err(|_| CryptoError::InvalidInput)?;
        Ok(Self { signing_key })
    }

    pub fn from_secret(secret: &[u8; 32]) -> Result<Self, CryptoError> {
        let signing_key =
            SigningKey::from_bytes(secret.into()).map_err(|_| CryptoError::InvalidInput)?;
        Ok(Self { signing_key })
    }

    pub fn public_key_sec1(&self, output: &mut [u8]) -> Result<usize, CryptoError> {
        let encoded = self.signing_key.verifying_key().to_encoded_point(false);
        if output.len() < encoded.as_bytes().len() {
            return Err(CryptoError::OutputTooSmall);
        }
        output[..encoded.as_bytes().len()].copy_from_slice(encoded.as_bytes());
        Ok(encoded.as_bytes().len())
    }
}

impl Signer for P256KeyPair {
    fn sign(&self, message: &[u8], output: &mut [u8]) -> Result<usize, CryptoError> {
        if output.len() < 64 {
            return Err(CryptoError::OutputTooSmall);
        }
        let digest = Sha256::digest(message);
        let signature: Signature = self
            .signing_key
            .sign_prehash(&digest)
            .map_err(|_| CryptoError::InvalidInput)?;
        output[..64].copy_from_slice(&signature.to_bytes());
        Ok(64)
    }
}

pub struct P256Verifier {
    verifying_key: VerifyingKey,
}

impl P256Verifier {
    pub fn from_sec1(public_key: &[u8]) -> Result<Self, CryptoError> {
        let verifying_key =
            VerifyingKey::from_sec1_bytes(public_key).map_err(|_| CryptoError::InvalidInput)?;
        Ok(Self { verifying_key })
    }
}

impl Verifier for P256Verifier {
    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<(), CryptoError> {
        let signature = Signature::from_slice(signature).map_err(|_| CryptoError::InvalidInput)?;
        let digest = Sha256::digest(message);
        self.verifying_key
            .verify_prehash(&digest, &signature)
            .map_err(|_| CryptoError::InvalidInput)
    }
}

#[cfg(test)]
mod tests {
    use super::{P256KeyPair, P256Verifier, RngProvider, Signer, Verifier};

    struct FixedRng;

    impl RngProvider for FixedRng {
        type Error = ();

        fn fill_bytes(&mut self, output: &mut [u8]) -> Result<(), Self::Error> {
            output.fill(7);
            Ok(())
        }
    }

    #[test]
    fn gera_assina_e_verifica_p256() {
        let pair = P256KeyPair::generate(&mut FixedRng).unwrap();
        let mut public_key = [0; 65];
        let public_key_len = pair.public_key_sec1(&mut public_key).unwrap();
        let verifier = P256Verifier::from_sec1(&public_key[..public_key_len]).unwrap();
        let mut signature = [0; 64];
        pair.sign(b"openkey", &mut signature).unwrap();
        verifier.verify(b"openkey", &signature).unwrap();
        assert!(verifier.verify(b"tampered", &signature).is_err());
    }
}
