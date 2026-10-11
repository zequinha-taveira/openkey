#![no_std]

//! Fronteira CTAP/FIDO sem dependência de USB ou de uma MCU.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Command {
    MakeCredential,
    GetAssertion,
    Reset,
}

pub fn decode_command(input: &[u8]) -> Option<Command> {
    match input.first().copied() {
        Some(0x01) => Some(Command::MakeCredential),
        Some(0x02) => Some(Command::GetAssertion),
        Some(0x07) => Some(Command::Reset),
        _ => None,
    }
}
