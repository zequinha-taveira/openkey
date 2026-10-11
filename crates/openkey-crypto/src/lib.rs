#![no_std]

/// Erros comuns da camada criptográfica.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CryptoError {
    Unsupported,
    InvalidInput,
    OutputTooSmall,
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
