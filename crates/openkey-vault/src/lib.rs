#![no_std]

//! Contratos de cofre lógico: chaves são referenciadas por identificadores opacos.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KeyId(u32);
impl KeyId {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }
}

pub trait KeyVault {
    type Error;
    fn contains(&self, key: KeyId) -> Result<bool, Self::Error>;
    fn use_key(
        &mut self,
        key: KeyId,
        message: &[u8],
        output: &mut [u8],
    ) -> Result<usize, Self::Error>;
}
