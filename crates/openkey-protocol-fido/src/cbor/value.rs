extern crate alloc;

use alloc::{boxed::Box, vec, vec::Vec};
use core::str;

use super::{decode_head, encode_head, CborError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value<'a> {
    Unsigned(u64),
    Negative(i64),
    Bytes(&'a [u8]),
    Text(&'a str),
    Array(Vec<Value<'a>>),
    Map(Vec<(Value<'a>, Value<'a>)>),
    Tag(u64, Box<Value<'a>>),
    Bool(bool),
    Null,
}

#[derive(Debug, Clone, Copy)]
pub struct Limits {
    pub max_depth: usize,
    pub max_items: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_depth: 16,
            max_items: 256,
        }
    }
}

impl<'a> Value<'a> {
    pub fn decode(input: &'a [u8], limits: Limits) -> Result<Self, CborError> {
        let mut decoder = Decoder {
            input,
            position: 0,
            limits,
        };
        let value = decoder.value(0)?;
        if decoder.position != input.len() {
            return Err(CborError::TrailingData);
        }
        Ok(value)
    }

    pub fn encode(&self, output: &mut [u8]) -> Result<usize, CborError> {
        match self {
            Self::Unsigned(value) => encode_head(0, *value, output),
            Self::Negative(value) => {
                let encoded = value
                    .checked_neg()
                    .and_then(|v| v.checked_sub(1))
                    .ok_or(CborError::UnsupportedValue)?;
                encode_head(1, encoded as u64, output)
            }
            Self::Bytes(value) => encode_bytes(value, output),
            Self::Text(value) => encode_text(value, output),
            Self::Bool(value) => {
                if output.is_empty() {
                    return Err(CborError::OutputTooSmall);
                }
                output[0] = if *value { 0xf5 } else { 0xf4 };
                Ok(1)
            }
            Self::Null => {
                if output.is_empty() {
                    return Err(CborError::OutputTooSmall);
                }
                output[0] = 0xf6;
                Ok(1)
            }
            Self::Tag(tag, value) => {
                let head = encode_head(6, *tag, output)?;
                let used = value.encode(&mut output[head..])?;
                Ok(head + used)
            }
            Self::Array(values) => {
                let mut used = encode_head(4, values.len() as u64, output)?;
                for value in values {
                    used += value.encode(&mut output[used..])?;
                }
                Ok(used)
            }
            Self::Map(entries) => {
                let mut used = encode_head(5, entries.len() as u64, output)?;
                let mut previous = Vec::new();
                for (key, value) in entries {
                    let mut key_buffer = vec![0; 1024];
                    let key_size = key.encode(&mut key_buffer)?;
                    let key_bytes = &key_buffer[..key_size];
                    if !previous.is_empty()
                        && canonical_key_cmp(&previous, key_bytes) != core::cmp::Ordering::Less
                    {
                        return Err(CborError::NonCanonical);
                    }
                    previous.clear();
                    previous.extend_from_slice(key_bytes);
                    used += key_size;
                    output[used - key_size..used].copy_from_slice(key_bytes);
                    used += value.encode(&mut output[used..])?;
                }
                Ok(used)
            }
        }
    }
}

fn encode_bytes(value: &[u8], output: &mut [u8]) -> Result<usize, CborError> {
    let head = encode_head(2, value.len() as u64, output)?;
    let end = head
        .checked_add(value.len())
        .ok_or(CborError::OutputTooSmall)?;
    if output.len() < end {
        return Err(CborError::OutputTooSmall);
    }
    output[head..end].copy_from_slice(value);
    Ok(end)
}

fn encode_text(value: &str, output: &mut [u8]) -> Result<usize, CborError> {
    let head = encode_head(3, value.len() as u64, output)?;
    let end = head
        .checked_add(value.len())
        .ok_or(CborError::OutputTooSmall)?;
    if output.len() < end {
        return Err(CborError::OutputTooSmall);
    }
    output[head..end].copy_from_slice(value.as_bytes());
    Ok(end)
}

fn canonical_key_cmp(left: &[u8], right: &[u8]) -> core::cmp::Ordering {
    left.len().cmp(&right.len()).then_with(|| left.cmp(right))
}

struct Decoder<'a> {
    input: &'a [u8],
    position: usize,
    limits: Limits,
}

impl<'a> Decoder<'a> {
    fn value(&mut self, depth: usize) -> Result<Value<'a>, CborError> {
        if depth > self.limits.max_depth {
            return Err(CborError::DepthExceeded);
        }
        let first = *self.input.get(self.position).ok_or(CborError::Empty)?;
        let major = first >> 5;
        match major {
            0 => {
                let (value, used) = decode_head(&self.input[self.position..], 0)?;
                self.position += used;
                Ok(Value::Unsigned(value))
            }
            1 => {
                let (value, used) = decode_head(&self.input[self.position..], 1)?;
                self.position += used;
                let value = i64::try_from(value)
                    .ok()
                    .and_then(|v| v.checked_add(1))
                    .and_then(|v| v.checked_neg())
                    .ok_or(CborError::UnsupportedValue)?;
                Ok(Value::Negative(value))
            }
            2 | 3 => {
                let (length, head) = decode_head(&self.input[self.position..], major)?;
                let length = usize::try_from(length).map_err(|_| CborError::Truncated)?;
                let start = self.position + head;
                let end = start.checked_add(length).ok_or(CborError::Truncated)?;
                if end > self.input.len() {
                    return Err(CborError::Truncated);
                }
                self.position = end;
                if major == 2 {
                    Ok(Value::Bytes(&self.input[start..end]))
                } else {
                    Ok(Value::Text(
                        str::from_utf8(&self.input[start..end])
                            .map_err(|_| CborError::InvalidUtf8)?,
                    ))
                }
            }
            4 => self.array(depth),
            5 => self.map(depth),
            6 => {
                let (tag, used) = decode_head(&self.input[self.position..], 6)?;
                self.position += used;
                Ok(Value::Tag(tag, Box::new(self.value(depth + 1)?)))
            }
            7 => {
                self.position += 1;
                match first & 0x1f {
                    20 => Ok(Value::Bool(false)),
                    21 => Ok(Value::Bool(true)),
                    22 => Ok(Value::Null),
                    _ => Err(CborError::UnsupportedValue),
                }
            }
            _ => Err(CborError::UnsupportedValue),
        }
    }

    fn array(&mut self, depth: usize) -> Result<Value<'a>, CborError> {
        let (length, used) = decode_head(&self.input[self.position..], 4)?;
        self.position += used;
        let length = usize::try_from(length).map_err(|_| CborError::TooManyItems)?;
        if length > self.limits.max_items {
            return Err(CborError::TooManyItems);
        }
        let mut values = Vec::with_capacity(length);
        for _ in 0..length {
            values.push(self.value(depth + 1)?);
        }
        Ok(Value::Array(values))
    }

    fn map(&mut self, depth: usize) -> Result<Value<'a>, CborError> {
        let (length, used) = decode_head(&self.input[self.position..], 5)?;
        self.position += used;
        let length = usize::try_from(length).map_err(|_| CborError::TooManyItems)?;
        if length > self.limits.max_items {
            return Err(CborError::TooManyItems);
        }
        let mut entries = Vec::with_capacity(length);
        let mut previous: Option<Vec<u8>> = None;
        for _ in 0..length {
            let start = self.position;
            let key = self.value(depth + 1)?;
            let encoded = self.input[start..self.position].to_vec();
            if let Some(old) = &previous {
                if canonical_key_cmp(old, &encoded) != core::cmp::Ordering::Less {
                    return Err(CborError::NonCanonical);
                }
            }
            previous = Some(encoded);
            let value = self.value(depth + 1)?;
            entries.push((key, value));
        }
        Ok(Value::Map(entries))
    }
}

#[cfg(test)]
mod tests {
    use super::{Limits, Value};

    #[test]
    fn decodifica_array_mapa_tag_booleano_e_nulo() {
        let input = [0x84, 0x01, 0xf4, 0xf6, 0xc1, 0x18, 0x2a];
        let value = Value::decode(&input, Limits::default()).unwrap();
        assert_eq!(
            value,
            Value::Array(alloc::vec![
                Value::Unsigned(1),
                Value::Bool(false),
                Value::Null,
                Value::Tag(1, alloc::boxed::Box::new(Value::Unsigned(42)))
            ])
        );
    }

    #[test]
    fn rejeita_mapa_fora_da_ordem_canonica() {
        assert!(Value::decode(&[0xa2, 0x02, 0x00, 0x01, 0x00], Limits::default()).is_err());
    }

    #[test]
    fn aplica_limite_recursivo() {
        assert_eq!(
            Value::decode(
                &[0x81, 0x81, 0x00],
                Limits {
                    max_depth: 1,
                    max_items: 4
                }
            ),
            Err(super::CborError::DepthExceeded)
        );
    }
}
