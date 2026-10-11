#![no_std]

/// Erros comuns da camada criptográfica.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CryptoError {
    Unsupported,
    InvalidInput,
}

/// Contrato para uma operação de assinatura sem acoplar o núcleo a um provedor.
pub trait Signer {
    fn sign(&self, message: &[u8], output: &mut [u8]) -> Result<usize, CryptoError>;
}
