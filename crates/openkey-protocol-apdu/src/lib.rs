#![no_std]

//! Parser mínimo de cabeçalho APDU; transporte é responsabilidade de outra crate.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CommandHeader {
    pub cla: u8,
    pub ins: u8,
    pub p1: u8,
    pub p2: u8,
}

pub fn parse_header(input: &[u8]) -> Option<CommandHeader> {
    if input.len() < 4 {
        return None;
    }
    Some(CommandHeader {
        cla: input[0],
        ins: input[1],
        p1: input[2],
        p2: input[3],
    })
}
