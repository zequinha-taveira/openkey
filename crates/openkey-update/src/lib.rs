#![no_std]

//! Política de atualização: verificar antes de ativar e rejeitar downgrade por padrão.

pub trait FirmwareVerifier {
    type Error;
    fn verify(&self, image: &[u8], signature: &[u8]) -> Result<(), Self::Error>;
}

pub fn can_activate(current: u32, candidate: u32) -> bool {
    candidate >= current
}
