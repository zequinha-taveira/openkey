use openkey_protocol_apdu::{ApduCommand, ApduError};

#[test]
fn parser_rejeita_cabecalho_incompleto() {
    assert_eq!(ApduCommand::parse(&[0, 1, 2]), Err(ApduError::TooShort));
}

#[test]
fn parser_le_cabecalho() {
    assert_eq!(
        ApduCommand::parse(&[0, 1, 2, 3]).unwrap(),
        ApduCommand {
            cla: 0,
            ins: 1,
            p1: 2,
            p2: 3
        }
    );
}
