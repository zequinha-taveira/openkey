#![no_std]

/// Resultado da validação de uma atualização assinada.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateError {
    InvalidSignature,
    Rollback,
    Unsupported,
}
