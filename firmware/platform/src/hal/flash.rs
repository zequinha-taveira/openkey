//! HAL Flash traits
//!
//! Abstração de armazenamento não-volátil (Flash / NVRAM).

/// Tamanho de página típico de flash para alinhamento
pub const FLASH_PAGE_SIZE: u32 = 4096;

/// Erro de operação de flash
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlashError {
    /// Endereço fora dos limites
    OutOfBounds,
    /// Erro de alinhamento
    Misaligned,
    /// Erro de escrita (pode exigir erase prévio)
    WriteError,
    /// Erro de erase
    EraseError,
    /// Falha de hardware
    HardwareFailure,
}

/// Provedor de armazenamento não-volátil (Flash / NVRAM)
pub trait FlashStorageProvider {
    /// Lê dados da memória persistente no offset especificado
    fn read(&mut self, offset: u32, buf: &mut [u8]) -> Result<(), FlashError>;
    /// Escreve dados na memória persistente no offset especificado
    fn write(&mut self, offset: u32, data: &[u8]) -> Result<(), FlashError>;
    /// Apaga um setor de flash a partir do offset especificado
    fn erase(&mut self, offset: u32, len: u32) -> Result<(), FlashError>;
    /// Retorna o tamanho total da memória flash em bytes
    fn total_size(&self) -> u32;
}
