#![no_std]

/// Resultado da validação de uma atualização assinada.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateError {
    InvalidSignature,
    Rollback,
    Unsupported,
}

/// Metadados mínimos que serão assinados pelo manifesto de atualização.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateManifest {
    pub version: u32,
    pub image_len: u32,
}

impl UpdateManifest {
    pub const fn is_newer_than(self, current_version: u32) -> bool {
        self.version > current_version
    }
}
