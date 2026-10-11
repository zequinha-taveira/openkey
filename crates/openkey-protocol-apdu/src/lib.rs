#![no_std]

/// Status mínimo de resposta APDU.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApduStatus {
    Success,
    InstructionNotSupported,
}
