extern crate alloc;

use alloc::vec;
use sha2::{Digest, Sha256};

use crate::cbor::{value::Value, CborError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ctap2Error {
    Cbor(CborError),
    OutputTooSmall,
    InvalidPin,
    PinBlocked,
}

impl From<CborError> for Ctap2Error {
    fn from(error: CborError) -> Self {
        Self::Cbor(error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthenticatorInfo {
    pub aaguid: [u8; 16],
    pub max_msg_size: u32,
    pub resident_key: bool,
    pub user_presence: bool,
    pub user_verification: bool,
}

impl Default for AuthenticatorInfo {
    fn default() -> Self {
        Self {
            aaguid: [0; 16],
            max_msg_size: 1200,
            resident_key: true,
            user_presence: true,
            user_verification: false,
        }
    }
}

impl AuthenticatorInfo {
    pub fn encode(&self, output: &mut [u8]) -> Result<usize, Ctap2Error> {
        let options = Value::Map(vec![
            (Value::Text("rk"), Value::Bool(self.resident_key)),
            (Value::Text("up"), Value::Bool(self.user_presence)),
            (Value::Text("uv"), Value::Bool(self.user_verification)),
        ]);
        let value = Value::Map(vec![
            (
                Value::Unsigned(1),
                Value::Array(vec![Value::Text("FIDO_2_1")]),
            ),
            (Value::Unsigned(2), Value::Array(vec![])),
            (Value::Unsigned(3), Value::Bytes(&self.aaguid)),
            (Value::Unsigned(4), options),
            (
                Value::Unsigned(5),
                Value::Unsigned(self.max_msg_size as u64),
            ),
        ]);
        value.encode(output).map_err(|error| match error {
            CborError::OutputTooSmall => Ctap2Error::OutputTooSmall,
            other => Ctap2Error::Cbor(other),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ctap2State {
    pin_hash: Option<[u8; 32]>,
    retries: u8,
    pub reset_generation: u32,
}

impl Default for Ctap2State {
    fn default() -> Self {
        Self {
            pin_hash: None,
            retries: 8,
            reset_generation: 0,
        }
    }
}

impl Ctap2State {
    pub fn set_pin(&mut self, pin: &[u8]) -> Result<(), Ctap2Error> {
        if pin.is_empty() || pin.len() > 63 {
            return Err(Ctap2Error::InvalidPin);
        }
        self.pin_hash = Some(Sha256::digest(pin).into());
        self.retries = 8;
        Ok(())
    }

    pub fn verify_pin(&mut self, pin: &[u8]) -> Result<(), Ctap2Error> {
        if self.retries == 0 {
            return Err(Ctap2Error::PinBlocked);
        }
        let candidate: [u8; 32] = Sha256::digest(pin).into();
        if self.pin_hash == Some(candidate) {
            self.retries = 8;
            Ok(())
        } else {
            self.retries -= 1;
            Err(Ctap2Error::InvalidPin)
        }
    }

    pub fn reset(&mut self) {
        self.pin_hash = None;
        self.retries = 8;
        self.reset_generation = self.reset_generation.wrapping_add(1);
    }
}

#[cfg(test)]
mod tests {
    use super::{AuthenticatorInfo, Ctap2State};
    use crate::cbor::value::{Limits, Value};

    #[test]
    fn get_info_eh_cbor_completo() {
        let mut output = [0; 256];
        let used = AuthenticatorInfo::default().encode(&mut output).unwrap();
        assert!(matches!(
            Value::decode(&output[..used], Limits::default()),
            Ok(Value::Map(_))
        ));
    }

    #[test]
    fn client_pin_e_reset_sao_deterministicos() {
        let mut state = Ctap2State::default();
        state.set_pin(b"1234").unwrap();
        assert!(state.verify_pin(b"0000").is_err());
        state.verify_pin(b"1234").unwrap();
        state.reset();
        assert!(state.verify_pin(b"1234").is_err());
        assert_eq!(state.reset_generation, 1);
    }
}
