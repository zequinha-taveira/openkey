//! HAL I2C traits
//!
//! Abstração de barramento I2C para comunicação com periféricos.

use super::error::HalError;

/// Provedor de barramento I2C
pub trait I2cProvider {
    /// Configura a frequência do I2C em Hz
    fn configure(&mut self, frequency: u32) -> Result<(), HalError>;
    /// Lê dados de um dispositivo I2C
    fn read(&mut self, address: u8, buf: &mut [u8]) -> Result<(), HalError>;
    /// Escreve dados para um dispositivo I2C
    fn write(&mut self, address: u8, data: &[u8]) -> Result<(), HalError>;
    /// Escreve dados seguidos de uma leitura (write-then-read)
    fn write_read(&mut self, address: u8, tx: &[u8], rx: &mut [u8]) -> Result<(), HalError>;
}
