//! HAL Watchdog traits
//!
//! Abstração de watchdog para detecção de travamentos.

use super::error::HalError;

/// Provedor de watchdog para reinicialização automática em caso de travamento
pub trait WatchdogProvider {
    /// Inicializa o watchdog com o timeout especificado em milissegundos
    fn init(&mut self, timeout_ms: u32) -> Result<(), HalError>;
    /// Alimenta o watchdog (reinicia o contador)
    fn feed(&mut self);
    /// Desativa o watchdog
    fn disable(&mut self);
}
