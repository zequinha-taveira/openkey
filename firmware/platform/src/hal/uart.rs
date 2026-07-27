//! HAL UART traits
//!
//! Abstração de UART para comunicação serial.

use super::error::HalError;

/// Provedor de UART para comunicação serial
pub trait UartProvider {
    /// Inicializa a UART com a frequência de clock e baud rate especificados
    fn init(&mut self, baud_rate: u32) -> Result<(), HalError>;
    /// Envia dados pela UART
    fn write(&mut self, data: &[u8]) -> Result<(), HalError>;
    /// Recebe dados da UART (não bloqueante, retorna quantos bytes foram lidos)
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, HalError>;
    /// Verifica se há dados disponíveis para leitura
    fn available(&self) -> usize;
}
