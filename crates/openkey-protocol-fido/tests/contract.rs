use openkey_protocol_fido::{FidoCommand, FidoError, GetInfoResponse};

#[test]
fn get_info_eh_deterministico() {
    let mut response = [0; 8];
    let size = GetInfoResponse::encode(&mut response).expect("buffer suficiente");
    assert_eq!(&response[..size], b"FIDO_2_1");
}

#[test]
fn entrada_vazia_e_rejeitada() {
    assert_eq!(FidoCommand::decode(&[]), Err(FidoError::EmptyCommand));
}
