//! HAL RNG traits
//!
//! Abstração de gerador de números aleatórios (TRNG / RNG de hardware).

use super::error::HalError;

/// Provedor de números aleatórios de entropia (TRNG / RNG de hardware)
pub trait RngProvider {
    /// Preenche o buffer fornecido com bytes aleatórios seguros
    fn fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), HalError>;
    /// Gera um número aleatório de 32 bits
    fn next_u32(&mut self) -> Result<u32, HalError> {
        let mut buf = [0u8; 4];
        self.fill_bytes(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }
    /// Verifica se o RNG está saudável
    fn is_healthy(&self) -> bool;
}
