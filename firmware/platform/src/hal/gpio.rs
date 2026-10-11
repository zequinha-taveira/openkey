//! HAL GPIO traits
//!
//! Abstração de GPIO para controle de pinos digitais, LEDs e botões.

use super::error::HalError;

/// Direção de um pino GPIO
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpioDirection {
    Input,
    Output,
    InputPullUp,
    InputPullDown,
    Analog,
}

/// Nível lógico de um pino GPIO
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpioLevel {
    Low,
    High,
}

/// Provedor de GPIO para controle de pinos digitais
pub trait GpioProvider {
    /// Configura a direção de um pino GPIO
    fn set_direction(&mut self, pin: u8, direction: GpioDirection) -> Result<(), HalError>;
    /// Lê o nível lógico de um pino GPIO
    fn read_pin(&mut self, pin: u8) -> Result<GpioLevel, HalError>;
    /// Escreve um nível lógico em um pino GPIO
    fn write_pin(&mut self, pin: u8, level: GpioLevel) -> Result<(), HalError>;
    /// Alterna o nível lógico de um pino GPIO
    fn toggle_pin(&mut self, pin: u8) -> Result<(), HalError>;
}
