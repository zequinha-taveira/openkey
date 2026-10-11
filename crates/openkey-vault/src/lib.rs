#![no_std]

/// Erros de armazenamento independentes do backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaultError {
    NotFound,
    AlreadyExists,
    InvalidKey,
    StorageFailure,
    OutputTooSmall,
}

/// Armazenamento que pode ser implementado em flash, secure element ou RAM.
pub trait KeyStore {
    type Error;

    fn read(&self, key_id: &[u8], output: &mut [u8]) -> Result<usize, Self::Error>;
    fn write(&mut self, key_id: &[u8], value: &[u8]) -> Result<(), Self::Error>;
    fn delete(&mut self, key_id: &[u8]) -> Result<(), Self::Error>;
}
