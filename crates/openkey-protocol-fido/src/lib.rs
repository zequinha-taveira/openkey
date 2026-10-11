#![no_std]

pub mod cbor;

/// Comandos CTAP2 suportados ou reservados pelo framework.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FidoCommand {
    MakeCredential,
    GetAssertion,
    GetInfo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FidoError {
    EmptyCommand,
    InvalidLength,
    UnsupportedCommand,
    OutputTooSmall,
}

impl FidoCommand {
    pub fn decode(input: &[u8]) -> Result<Self, FidoError> {
        let command = *input.first().ok_or(FidoError::EmptyCommand)?;
        if input.len() != 1 {
            return Err(FidoError::InvalidLength);
        }
        match command {
            0x01 => Ok(Self::MakeCredential),
            0x02 => Ok(Self::GetAssertion),
            0x04 => Ok(Self::GetInfo),
            _ => Err(FidoError::UnsupportedCommand),
        }
    }
}

pub struct GetInfoResponse;

impl GetInfoResponse {
    pub const VERSION: &'static [u8] = b"FIDO_2_1";

    pub fn encode(output: &mut [u8]) -> Result<usize, FidoError> {
        if output.len() < Self::VERSION.len() {
            return Err(FidoError::OutputTooSmall);
        }
        output[..Self::VERSION.len()].copy_from_slice(Self::VERSION);
        Ok(Self::VERSION.len())
    }
}

#[cfg(test)]
mod tests {
    use super::{FidoCommand, FidoError, GetInfoResponse};

    #[test]
    fn decodifica_get_info() {
        assert_eq!(FidoCommand::decode(&[0x04]), Ok(FidoCommand::GetInfo));
    }

    #[test]
    fn rejeita_comando_com_payload_nesta_camada() {
        assert_eq!(
            FidoCommand::decode(&[0x04, 0]),
            Err(FidoError::InvalidLength)
        );
    }

    #[test]
    fn codifica_versao() {
        let mut output = [0; 8];
        assert_eq!(GetInfoResponse::encode(&mut output), Ok(8));
        assert_eq!(&output, b"FIDO_2_1");
    }
}
