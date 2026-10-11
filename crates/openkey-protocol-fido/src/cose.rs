use alloc::vec;

use crate::cbor::{
    value::{Limits, Value},
    CborError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoseError {
    Cbor(CborError),
    InvalidShape,
    MissingField,
    UnsupportedAlgorithm,
    OutputTooSmall,
}

impl From<CborError> for CoseError {
    fn from(error: CborError) -> Self {
        Self::Cbor(error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoseKey<'a> {
    pub key_type: u64,
    pub algorithm: i64,
    pub curve: u64,
    pub x: &'a [u8],
    pub y: Option<&'a [u8]>,
}

impl<'a> CoseKey<'a> {
    pub fn decode(input: &'a [u8]) -> Result<Self, CoseError> {
        let value = Value::decode(input, Limits::default())?;
        let entries = match value {
            Value::Map(entries) => entries,
            _ => return Err(CoseError::InvalidShape),
        };
        let mut key_type = None;
        let mut algorithm = None;
        let mut curve = None;
        let mut x = None;
        let mut y = None;
        for (key, value) in entries {
            match key {
                Value::Unsigned(1) => key_type = Some(take_uint(value)?),
                Value::Unsigned(3) => algorithm = Some(take_int(value)?),
                Value::Negative(-1) => curve = Some(take_uint(value)?),
                Value::Negative(-2) => x = Some(take_bytes(value)?),
                Value::Negative(-3) => y = Some(take_bytes(value)?),
                _ => {}
            }
        }
        let key = Self {
            key_type: key_type.ok_or(CoseError::MissingField)?,
            algorithm: algorithm.ok_or(CoseError::MissingField)?,
            curve: curve.ok_or(CoseError::MissingField)?,
            x: x.ok_or(CoseError::MissingField)?,
            y,
        };
        if key.key_type != 2
            || key.algorithm != -7
            || key.curve != 1
            || key.x.len() != 32
            || key.y.is_some_and(|value| value.len() != 32)
        {
            return Err(CoseError::UnsupportedAlgorithm);
        }
        Ok(key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoseSign1<'a> {
    pub protected: &'a [u8],
    pub payload: Option<&'a [u8]>,
    pub signature: &'a [u8],
}

impl<'a> CoseSign1<'a> {
    pub fn decode(input: &'a [u8]) -> Result<Self, CoseError> {
        let value = Value::decode(input, Limits::default())?;
        let values = match value {
            Value::Array(values) if values.len() == 4 => values,
            _ => return Err(CoseError::InvalidShape),
        };
        let mut values = values.into_iter();
        let protected = take_bytes(values.next().ok_or(CoseError::InvalidShape)?)?;
        if !matches!(values.next().ok_or(CoseError::InvalidShape)?, Value::Map(_)) {
            return Err(CoseError::InvalidShape);
        }
        let payload = match values.next().ok_or(CoseError::InvalidShape)? {
            Value::Null => None,
            value => Some(take_bytes(value)?),
        };
        let signature = take_bytes(values.next().ok_or(CoseError::InvalidShape)?)?;
        if signature.len() != 64 {
            return Err(CoseError::InvalidShape);
        }
        Ok(Self {
            protected,
            payload,
            signature,
        })
    }

    pub fn sig_structure(
        &self,
        external_aad: &[u8],
        output: &mut [u8],
    ) -> Result<usize, CoseError> {
        let payload = self.payload.ok_or(CoseError::InvalidShape)?;
        let value = Value::Array(vec![
            Value::Text("Signature1"),
            Value::Bytes(self.protected),
            Value::Bytes(external_aad),
            Value::Bytes(payload),
        ]);
        value.encode(output).map_err(|error| match error {
            CborError::OutputTooSmall => CoseError::OutputTooSmall,
            other => CoseError::Cbor(other),
        })
    }
}

fn take_uint(value: Value<'_>) -> Result<u64, CoseError> {
    match value {
        Value::Unsigned(value) => Ok(value),
        _ => Err(CoseError::InvalidShape),
    }
}

fn take_int(value: Value<'_>) -> Result<i64, CoseError> {
    match value {
        Value::Unsigned(value) => i64::try_from(value).map_err(|_| CoseError::InvalidShape),
        Value::Negative(value) => Ok(value),
        _ => Err(CoseError::InvalidShape),
    }
}

fn take_bytes<'a>(value: Value<'a>) -> Result<&'a [u8], CoseError> {
    match value {
        Value::Bytes(value) => Ok(value),
        _ => Err(CoseError::InvalidShape),
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use super::{CoseKey, CoseSign1};

    #[test]
    fn decodifica_cose_key_es256() {
        let mut input = vec![0xa5, 0x01, 0x02, 0x03, 0x26, 0x20, 0x01, 0x21, 0x58, 0x20];
        input.extend_from_slice(&[0x11; 32]);
        input.extend_from_slice(&[0x22, 0x58, 0x20]);
        input.extend_from_slice(&[0x33; 32]);
        let key = CoseKey::decode(&input).unwrap();
        assert_eq!((key.key_type, key.algorithm, key.curve), (2, -7, 1));
    }

    #[test]
    fn rejeita_signature_com_tamanho_invalido() {
        let input = [0x84, 0x40, 0xa0, 0x40, 0x01];
        assert!(CoseSign1::decode(&input).is_err());
    }

    #[test]
    fn rejeita_chave_com_algoritmo_nao_suportado() {
        let mut input = vec![0xa5, 0x01, 0x02, 0x03, 0x27, 0x20, 0x01, 0x21, 0x58, 0x20];
        input.extend_from_slice(&[0x11; 32]);
        input.extend_from_slice(&[0x22, 0x58, 0x20]);
        input.extend_from_slice(&[0x33; 32]);
        assert!(CoseKey::decode(&input).is_err());
    }
}
