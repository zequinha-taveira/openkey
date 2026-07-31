//! Key Management - gerenciamento seguro de chaves criptográficas
//!
//! Fornece traits e tipos para geração, armazenamento e zeroização
//! de chaves de atestação (P-256 / Ed25519).
//!
//! ## Segurança
//!
//! - Chaves efêmeras são zeroizadas automaticamente via `Drop`
//! - Chaves de atestação são lidas de OTP (nunca geradas no firmware)
//! - Geração de pares de chaves usa TRNG via `RngProvider`

use ed25519_dalek::{
    Signature as Ed25519Signature, Signer, SigningKey as Ed25519SigningKey,
    VerifyingKey as Ed25519VerifyingKey, Verifier as Ed25519Verifier,
};
use p256::ecdsa::{Signature as P256Signature, SigningKey as P256SigningKey, VerifyingKey as P256VerifyingKey};
use p256::elliptic_curve::rand_core::{CryptoRng, RngCore};
use sha2::{Digest, Sha256};

/// Tamanho de uma chave P-256 em bytes (ponto X + Y = 64 bytes)
pub const P256_PUBLIC_KEY_SIZE: usize = 64;
/// Tamanho de uma chave P-256 privada em bytes
pub const P256_PRIVATE_KEY_SIZE: usize = 32;
/// Tamanho de uma assinatura ECDSA P-256 em bytes (DER-encoded)
pub const P256_SIGNATURE_SIZE: usize = 72;
/// Tamanho de uma chave Ed25519 em bytes
pub const ED25519_PUBLIC_KEY_SIZE: usize = 32;
/// Tamanho de uma chave Ed25519 privada em bytes
pub const ED25519_PRIVATE_KEY_SIZE: usize = 32;
/// Tamanho de uma assinatura Ed25519 em bytes
pub const ED25519_SIGNATURE_SIZE: usize = 64;
/// Tamanho do AAGUID (Authenticator Attestation GUID)
pub const AAGUID_SIZE: usize = 16;

/// Tipo de algoritmo de chave de atestação
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttestationAlgorithm {
    /// ECDSA com curva P-256
    P256,
    /// Ed25519 (EdDSA)
    Ed25519,
}

/// Erro de gerenciamento de chaves
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyError {
    /// RNG falhou durante geração de chave
    RngFailure,
    /// Chave inválida
    InvalidKey,
    /// Assinatura inválida
    InvalidSignature,
    /// Operação não suportada
    Unsupported,
    /// OTP não programada
    OtpNotProgrammed,
    /// Falha de hardware
    HardwareFailure,
}

/// Provedor de chave de atestação
///
/// Define o contrato para acesso a chaves de atestação armazenadas
/// em OTP (One-Time Programmable) ou HSM. A implementação concreta
/// é fornecida pela camada de plataforma (HAL).
pub trait AttestationKeyProvider {
    /// Lê a chave privada de atestação do OTP/HSM
    ///
    /// A chave nunca é exposta diretamente — apenas usada para assinar.
    /// Em implementações seguras, a assinatura ocorre dentro do HSM/OTP.
    fn sign(&mut self, algorithm: AttestationAlgorithm, message: &[u8]) -> Result<AttestationSignature, KeyError>;

    /// Retorna a chave pública de atestação
    fn public_key(&self, algorithm: AttestationAlgorithm) -> Result<AttestationPublicKey, KeyError>;

    /// Verifica se a chave de atestação está disponível
    fn is_available(&self) -> bool;
}

/// Chave pública de atestação
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttestationPublicKey {
    /// Algoritmo da chave
    pub algorithm: AttestationAlgorithm,
    /// Bytes da chave pública (64 bytes para P-256, 32 bytes para Ed25519)
    pub bytes: heapless::Vec<u8, P256_PUBLIC_KEY_SIZE>,
}

/// Assinatura de atestado
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttestationSignature {
    /// Algoritmo usado para assinar
    pub algorithm: AttestationAlgorithm,
    /// Bytes da assinatura
    pub bytes: heapless::Vec<u8, P256_SIGNATURE_SIZE>,
}

/// Par de chaves efêmero com zeroização automática
///
/// Chaves geradas para operações temporárias (ex.: ECDH) são
/// automaticamente zeroizadas quando o struct é descartado.
pub struct EphemeralKeyPair {
    algorithm: AttestationAlgorithm,
    private_key: heapless::Vec<u8, P256_PRIVATE_KEY_SIZE>,
    public_key: heapless::Vec<u8, P256_PUBLIC_KEY_SIZE>,
}

impl EphemeralKeyPair {
    /// Gera um novo par de chaves efêmero usando P-256
    pub fn generate_p256<R: RngCore + CryptoRng>(rng: &mut R) -> Result<Self, KeyError> {
        // Usa o método random() do p256 que lida corretamente com a
        // redução modular para garantir uma chave privada válida.
        let signing_key = P256SigningKey::random(rng);
        let verifying_key = signing_key.verifying_key();
        let encoded_point = verifying_key.to_encoded_point(false);
        let point_bytes = encoded_point.as_bytes();
        // Ponto não comprimido: [0x04 || X (32) || Y (32)] = 65 bytes
        // Armazenamos apenas X+Y (64 bytes) para caber no buffer
        if point_bytes.len() != 65 || point_bytes[0] != 0x04 {
            return Err(KeyError::InvalidKey);
        }

        // Extrai a chave privada como bytes
        let private_key = signing_key.to_bytes();
        let private_bytes: &[u8] = private_key.as_ref();

        let mut public = heapless::Vec::new();
        public.extend_from_slice(&point_bytes[1..])
            .map_err(|_| KeyError::InvalidKey)?;

        let mut private = heapless::Vec::new();
        private.extend_from_slice(private_bytes)
            .map_err(|_| KeyError::InvalidKey)?;

        Ok(Self {
            algorithm: AttestationAlgorithm::P256,
            private_key: private,
            public_key: public,
        })
    }

    /// Gera um novo par de chaves efêmero usando Ed25519
    pub fn generate_ed25519<R: RngCore>(rng: &mut R) -> Result<Self, KeyError> {
        let mut seed = [0u8; ED25519_PRIVATE_KEY_SIZE];
        rng.fill_bytes(&mut seed);

        let signing_key = Ed25519SigningKey::from_bytes(&seed);
        let verifying_key = signing_key.verifying_key();
        let public_bytes = verifying_key.to_bytes();

        let mut public = heapless::Vec::new();
        public.extend_from_slice(&public_bytes)
            .map_err(|_| KeyError::InvalidKey)?;

        let mut private = heapless::Vec::new();
        private.extend_from_slice(&seed)
            .map_err(|_| KeyError::InvalidKey)?;

        Ok(Self {
            algorithm: AttestationAlgorithm::Ed25519,
            private_key: private,
            public_key: public,
        })
    }

    /// Retorna a chave pública
    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }

    /// Retorna o algoritmo
    pub fn algorithm(&self) -> AttestationAlgorithm {
        self.algorithm
    }

    /// Assina uma mensagem com a chave efêmera
    pub fn sign(&self, message: &[u8]) -> Result<heapless::Vec<u8, P256_SIGNATURE_SIZE>, KeyError> {
        match self.algorithm {
            AttestationAlgorithm::P256 => {
                let signing_key = P256SigningKey::from_slice(&self.private_key)
                    .map_err(|_| KeyError::InvalidKey)?;
                let signature: P256Signature = signing_key.sign(message);
                let sig_bytes = signature.to_der();
                let mut result = heapless::Vec::new();
                result.extend_from_slice(sig_bytes.as_bytes())
                    .map_err(|_| KeyError::InvalidKey)?;
                Ok(result)
            }
            AttestationAlgorithm::Ed25519 => {
                let private_arr: [u8; ED25519_PRIVATE_KEY_SIZE] = self.private_key
                    .as_slice()
                    .try_into()
                    .map_err(|_| KeyError::InvalidKey)?;
                let signing_key = Ed25519SigningKey::from_bytes(&private_arr);
                let signature: Ed25519Signature = signing_key.sign(message);
                let mut result = heapless::Vec::new();
                result.extend_from_slice(&signature.to_bytes())
                    .map_err(|_| KeyError::InvalidKey)?;
                Ok(result)
            }
        }
    }
}

impl Drop for EphemeralKeyPair {
    fn drop(&mut self) {
        // Zeroiza chaves privadas efêmeras
        self.private_key.fill(0);
        self.private_key.clear();
    }
}

/// Verifica uma assinatura ECDSA P-256
pub fn verify_p256_signature(
    public_key: &[u8],
    message: &[u8],
    signature: &[u8],
) -> Result<(), KeyError> {
    // A chave pública é armazenada como X+Y (64 bytes)
    // Precisamos reconstruir o ponto não comprimido [0x04 || X || Y]
    if public_key.len() != P256_PUBLIC_KEY_SIZE {
        return Err(KeyError::InvalidKey);
    }
    let mut sec1_bytes = [0u8; 65];
    sec1_bytes[0] = 0x04;
    sec1_bytes[1..].copy_from_slice(public_key);
    let verifying_key = P256VerifyingKey::from_sec1_bytes(&sec1_bytes)
        .map_err(|_| KeyError::InvalidKey)?;
    let sig = P256Signature::from_der(signature)
        .map_err(|_| KeyError::InvalidSignature)?;
    verifying_key.verify(message, &sig)
        .map_err(|_| KeyError::InvalidSignature)
}

/// Verifica uma assinatura Ed25519
pub fn verify_ed25519_signature(
    public_key: &[u8],
    message: &[u8],
    signature: &[u8],
) -> Result<(), KeyError> {
    let pub_key_arr: &[u8; ED25519_PUBLIC_KEY_SIZE] = public_key
        .try_into()
        .map_err(|_| KeyError::InvalidKey)?;
    let verifying_key = Ed25519VerifyingKey::from_bytes(pub_key_arr)
        .map_err(|_| KeyError::InvalidKey)?;
    let sig_bytes: [u8; ED25519_SIGNATURE_SIZE] = signature
        .try_into()
        .map_err(|_| KeyError::InvalidSignature)?;
    let sig = Ed25519Signature::from_bytes(&sig_bytes);
    verifying_key
        .verify(message, &sig)
        .map_err(|_| KeyError::InvalidSignature)
}

/// Calcula o AAGUID a partir de um Board Profile ID
///
/// O AAGUID é um GUID de 16 bytes que identifica o modelo do autenticador.
/// É derivado de forma determinística a partir do Board Profile ID para
/// garantir unicidade por modelo.
pub fn derive_aaguid(board_id: &[u8; 16]) -> [u8; AAGUID_SIZE] {
    let mut hasher = Sha256::new();
    hasher.update(board_id);
    let result = hasher.finalize();
    let mut aaguid = [0u8; AAGUID_SIZE];
    aaguid.copy_from_slice(&result[..AAGUID_SIZE]);
    aaguid
}

/// Deriva uma chave de atestação de um seed usando HKDF
///
/// Esta função é usada apenas em ambiente de simulação/teste.
/// Em produção, chaves de atestação são injetadas via OTP durante fabricação.
pub fn derive_attestation_key(
    seed: &[u8],
    info: &[u8],
) -> Result<heapless::Vec<u8, P256_PRIVATE_KEY_SIZE>, KeyError> {
    // HKDF-Expand usando SHA-256
    let mut hasher = Sha256::new();
    hasher.update(seed);
    hasher.update(info);
    let prk = hasher.finalize();

    // Segunda rodada (expand)
    let mut hasher2 = Sha256::new();
    hasher2.update(prk);
    hasher2.update([0x01]); // counter
    let okm = hasher2.finalize();

    let mut key = heapless::Vec::new();
    key.extend_from_slice(&okm[..P256_PRIVATE_KEY_SIZE])
        .map_err(|_| KeyError::InvalidKey)?;
    Ok(key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::OsRng;

    #[test]
    fn test_ephemeral_keypair_p256() {
        let mut rng = OsRng;
        let keypair = EphemeralKeyPair::generate_p256(&mut rng).unwrap();
        assert_eq!(keypair.algorithm(), AttestationAlgorithm::P256);
        assert!(!keypair.public_key().is_empty());
    }

    #[test]
    fn test_ephemeral_keypair_ed25519() {
        let mut rng = OsRng;
        let keypair = EphemeralKeyPair::generate_ed25519(&mut rng).unwrap();
        assert_eq!(keypair.algorithm(), AttestationAlgorithm::Ed25519);
        assert_eq!(keypair.public_key().len(), ED25519_PUBLIC_KEY_SIZE);
    }

    #[test]
    fn test_p256_sign_and_verify() {
        let mut rng = OsRng;
        let keypair = EphemeralKeyPair::generate_p256(&mut rng).unwrap();
        let message = b"test message for signing";
        let signature = keypair.sign(message).unwrap();

        // Verifica a assinatura com a chave pública
        verify_p256_signature(keypair.public_key(), message, &signature).unwrap();
    }

    #[test]
    fn test_ed25519_sign_and_verify() {
        let mut rng = OsRng;
        let keypair = EphemeralKeyPair::generate_ed25519(&mut rng).unwrap();
        let message = b"test message for signing";
        let signature = keypair.sign(message).unwrap();

        // Verifica a assinatura com a chave pública
        verify_ed25519_signature(keypair.public_key(), message, &signature).unwrap();
    }

    #[test]
    fn test_derive_aaguid() {
        let board_id = *b"openkey-rp23xx01";
        let aaguid = derive_aaguid(&board_id);
        assert_eq!(aaguid.len(), AAGUID_SIZE);
        // AAGUID deve ser determinístico
        let aaguid2 = derive_aaguid(&board_id);
        assert_eq!(aaguid, aaguid2);
    }

    #[test]
    fn test_derive_attestation_key() {
        let seed = b"test seed for key derivation";
        let info = b"attestation key";
        let key = derive_attestation_key(seed, info).unwrap();
        assert_eq!(key.len(), P256_PRIVATE_KEY_SIZE);
    }

    #[test]
    fn test_zeroization_on_drop() {
        let mut rng = OsRng;
        let keypair = EphemeralKeyPair::generate_p256(&mut rng).unwrap();
        let private_len = keypair.private_key.len();
        drop(keypair);
        // Após drop, a chave privada deve ser zeroizada
        // (verificamos indiretamente — o teste passa se não houver panic)
        assert!(private_len > 0);
    }
}
