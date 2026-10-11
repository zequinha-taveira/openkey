#![no_std]

/// Contrato inicial para armazenamento seguro de material de chave.
pub trait KeyStore {
    fn contains(&self, key_id: &[u8]) -> bool;
}
