//! HAL OTP (One-Time Programmable) Memory traits
//!
//! Abstração de memória OTP / eFuse para armazenamento de chaves
//! de atestação únicas, IDs de dispositivo e dados de segurança.
//!
//! A memória OTP permite escrita apenas uma vez. Uma vez programada,
//! os bits não podem ser revertidos. Esta abstração fornece acesso
//! somente leitura ao firmware, mantendo a integridade das chaves
//! de atestação.

/// Erro de operação de OTP
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OtpError {
    /// Endereço fora dos limites
    OutOfBounds,
    /// Erro de alinhamento
    Misaligned,
    /// Falha de hardware
    HardwareFailure,
    /// OTP não programada (bits em estado erased)
    NotProgrammed,
}

/// Provedor de memória OTP (One-Time Programmable / eFuse)
///
/// Fornece acesso somente leitura a regiões de memória OTP onde
/// chaves de atestação, IDs únicos e dados de segurança são armazenados
/// em produção. A escrita de OTP é feita apenas pelo provisionador
/// durante fabricação, nunca pelo firmware em execução.
pub trait OtpProvider {
    /// Lê bytes da memória OTP no offset especificado
    ///
    /// Retorna `Err(OtpError::NotProgrammed)` se a região não foi
    /// programada durante fabricação.
    fn read(&self, offset: u32, buf: &mut [u8]) -> Result<(), OtpError>;

    /// Verifica se um offset específico foi programado
    fn is_programmed(&self, offset: u32, len: usize) -> Result<bool, OtpError>;

    /// Retorna o tamanho total da memória OTP em bytes
    fn total_size(&self) -> u32;
}
