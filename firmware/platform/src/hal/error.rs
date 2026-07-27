//! HAL Error types
//!
//! Tipos de erro fortemente tipados para o HAL, substituindo `()`.

/// Erro genérico do HAL
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HalError {
    /// Operação não suportada nesta plataforma
    Unsupported,
    /// Hardware não inicializado
    NotInitialized,
    /// Falha de hardware
    HardwareFailure,
    /// Parâmetro inválido
    InvalidParameter,
    /// Timeout
    Timeout,
    /// Buffer insuficiente
    BufferTooSmall,
    /// Dispositivo não conectado
    NotConnected,
    /// Erro de comunicação
    CommunicationError,
    /// RNG não saudável
    RngNotHealthy,
}

impl From<crate::hal::flash::FlashError> for HalError {
    fn from(err: crate::hal::flash::FlashError) -> Self {
        match err {
            crate::hal::flash::FlashError::OutOfBounds => HalError::InvalidParameter,
            crate::hal::flash::FlashError::Misaligned => HalError::InvalidParameter,
            crate::hal::flash::FlashError::WriteError => HalError::HardwareFailure,
            crate::hal::flash::FlashError::EraseError => HalError::HardwareFailure,
            crate::hal::flash::FlashError::HardwareFailure => HalError::HardwareFailure,
        }
    }
}
