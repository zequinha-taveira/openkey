//! Módulo CTAP2 (Client-to-Authenticator Protocol v2.0 / v2.1)

pub mod commands;
pub mod get_info;
pub mod status;

pub use commands::Ctap2Command;
pub use get_info::GetInfoResponse;
pub use status::Ctap2Status;

/// Engine principal de despacho e execução de comandos CTAP2
pub struct Ctap2Engine;

impl Ctap2Engine {
    /// Processa uma requisição CTAP2 composta por `cmd_byte` + `payload_cbor`
    /// Escreve a resposta em `out_buf` no formato `[ status_byte, cbor_bytes... ]`
    /// Retorna o tamanho total da resposta gerada em bytes
    pub fn handle_request(
        cmd_byte: u8,
        payload_cbor: &[u8],
        aaguid: [u8; 16],
        pin_set: bool,
        out_buf: &mut [u8],
    ) -> usize {
        if out_buf.is_empty() {
            return 0;
        }

        let cmd = match Ctap2Command::from_u8(cmd_byte) {
            Some(cmd) => cmd,
            None => {
                out_buf[0] = Ctap2Status::ErrInvalidCommand.to_u8();
                return 1;
            }
        };

        match cmd {
            Ctap2Command::GetInfo => {
                if !payload_cbor.is_empty() {
                    out_buf[0] = Ctap2Status::ErrInvalidLength.to_u8();
                    return 1;
                }

                let get_info = GetInfoResponse::default_openkey(aaguid, pin_set);
                match get_info.encode_cbor(&mut out_buf[1..]) {
                    Ok(cbor_len) => {
                        out_buf[0] = Ctap2Status::Ok.to_u8();
                        1 + cbor_len
                    }
                    Err(_) => {
                        out_buf[0] = Ctap2Status::ErrOther.to_u8();
                        1
                    }
                }
            }
            Ctap2Command::MakeCredential
            | Ctap2Command::GetAssertion
            | Ctap2Command::ClientPin
            | Ctap2Command::Reset
            | Ctap2Command::GetNextAssertion
            | Ctap2Command::BioEnrollment
            | Ctap2Command::CredentialManagement
            | Ctap2Command::Selection
            | Ctap2Command::LargeBlobs
            | Ctap2Command::Config => {
                // Stubs para comandos adicionais — não implementados; retornam erro explícito
                // em vez de um status OK enganoso (que faria o host falhar ao parsear CBOR).
                out_buf[0] = Ctap2Status::ErrNotAllowed.to_u8();
                1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ctap2_engine_get_info() {
        let aaguid = [0x42; 16];
        let mut resp_buf = [0u8; 256];
        let len = Ctap2Engine::handle_request(
            Ctap2Command::GetInfo.to_u8(),
            &[],
            aaguid,
            false,
            &mut resp_buf,
        );

        assert!(len > 1);
        assert_eq!(resp_buf[0], Ctap2Status::Ok.to_u8());
    }

    #[test]
    fn test_ctap2_engine_invalid_command() {
        let mut resp_buf = [0u8; 16];
        let len = Ctap2Engine::handle_request(0xfe, &[], [0; 16], false, &mut resp_buf);
        assert_eq!(len, 1);
        assert_eq!(resp_buf[0], Ctap2Status::ErrInvalidCommand.to_u8());
    }

    #[test]
    fn test_ctap2_engine_unimplemented_stubs_return_error() {
        let stub_commands = [
            Ctap2Command::MakeCredential,
            Ctap2Command::GetAssertion,
            Ctap2Command::ClientPin,
            Ctap2Command::Reset,
            Ctap2Command::GetNextAssertion,
            Ctap2Command::BioEnrollment,
            Ctap2Command::CredentialManagement,
            Ctap2Command::Selection,
            Ctap2Command::LargeBlobs,
            Ctap2Command::Config,
        ];
        for cmd in stub_commands {
            let mut resp_buf = [0u8; 16];
            let len = Ctap2Engine::handle_request(cmd.to_u8(), &[], [0; 16], false, &mut resp_buf);
            assert_eq!(len, 1, "command {:?}", cmd);
            assert_eq!(
                resp_buf[0],
                Ctap2Status::ErrNotAllowed.to_u8(),
                "command {:?}",
                cmd
            );
        }
    }
}
