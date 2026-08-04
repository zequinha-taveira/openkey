//! Códigos de Comando do Protocolo CTAP2 (FIDO CTAP2.1 Specification)

/// Comandos CTAP2
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctap2Command {
    MakeCredential = 0x01,
    GetAssertion = 0x02,
    GetInfo = 0x04,
    ClientPin = 0x06,
    Reset = 0x07,
    GetNextAssertion = 0x08,
    BioEnrollment = 0x09,
    CredentialManagement = 0x0a,
    Selection = 0x0b,
    LargeBlobs = 0x0c,
    Config = 0x0d,
}

impl Ctap2Command {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0x01 => Some(Self::MakeCredential),
            0x02 => Some(Self::GetAssertion),
            0x04 => Some(Self::GetInfo),
            0x06 => Some(Self::ClientPin),
            0x07 => Some(Self::Reset),
            0x08 => Some(Self::GetNextAssertion),
            0x09 => Some(Self::BioEnrollment),
            0x0a => Some(Self::CredentialManagement),
            0x0b => Some(Self::Selection),
            0x0c => Some(Self::LargeBlobs),
            0x0d => Some(Self::Config),
            _ => None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        *self as u8
    }
}
