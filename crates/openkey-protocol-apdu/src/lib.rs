#![no_std]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApduCommand {
    pub cla: u8,
    pub ins: u8,
    pub p1: u8,
    pub p2: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApduError {
    TooShort,
}

impl ApduCommand {
    pub fn parse(input: &[u8]) -> Result<Self, ApduError> {
        if input.len() < 4 {
            return Err(ApduError::TooShort);
        }
        Ok(Self {
            cla: input[0],
            ins: input[1],
            p1: input[2],
            p2: input[3],
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApduStatus {
    Success,
    InstructionNotSupported,
}
