//! HAL SPI traits
//!
//! Abstração de barramento SPI para comunicação com periféricos.

use super::error::HalError;

/// Modo de clock SPI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpiMode {
    Mode0,
    Mode1,
    Mode2,
    Mode3,
}

/// Ordem de bits SPI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpiBitOrder {
    MsbFirst,
    LsbFirst,
}

/// Provedor de barramento SPI
pub trait SpiProvider {
    /// Configura o SPI com a frequência, modo e ordem de bits especificados
    fn configure(
        &mut self,
        frequency: u32,
        mode: SpiMode,
        bit_order: SpiBitOrder,
    ) -> Result<(), HalError>;
    /// Transmite e recebe dados simultaneamente (full-duplex)
    fn transfer(&mut self, tx: &[u8], rx: &mut [u8]) -> Result<(), HalError>;
    /// Transmite dados sem receber (write-only)
    fn write(&mut self, tx: &[u8]) -> Result<(), HalError>;
}
