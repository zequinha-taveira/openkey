//! HAL Timer traits
//!
//! Abstração de temporizadores para medição de tempo e timeouts.

/// Provedor de temporizador para medição de tempo e delays
pub trait TimerProvider {
    /// Retorna o tempo atual em milissegundos desde o boot
    fn millis(&self) -> u64;
    /// Retorna o tempo atual em microssegundos desde o boot
    fn micros(&self) -> u64;
    /// Retorna o tempo atual em nanossegundos desde o boot
    fn nanos(&self) -> u128;
    /// Delay em milissegundos (bloqueante)
    fn delay_ms(&mut self, ms: u32);
    /// Delay em microssegundos (bloqueante)
    fn delay_us(&mut self, us: u32);
}
