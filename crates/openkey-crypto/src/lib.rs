#![no_std]

//! Interfaces para primitivas criptográficas fornecidas por adaptadores revisados.

pub trait EntropySource {
    type Error;
    fn fill(&mut self, output: &mut [u8]) -> Result<(), Self::Error>;
}

pub trait Signer {
    type Error;
    fn sign(&mut self, message: &[u8], signature: &mut [u8]) -> Result<usize, Self::Error>;
}
