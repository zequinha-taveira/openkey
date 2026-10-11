use core::str;

pub mod value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CborError {
    Empty,
    WrongMajorType,
    Truncated,
    NonCanonical,
    InvalidUtf8,
    OutputTooSmall,
    TrailingData,
    UnsupportedValue,
    DepthExceeded,
    TooManyItems,
}

fn encode_head(major: u8, value: u64, output: &mut [u8]) -> Result<usize, CborError> {
    let prefix = major << 5;
    let (additional, width) = match value {
        0..=23 => (value as u8, 1),
        24..=255 => (24, 2),
        256..=65_535 => (25, 3),
        65_536..=4_294_967_295 => (26, 5),
        _ => (27, 9),
    };
    if output.len() < width {
        return Err(CborError::OutputTooSmall);
    }
    output[0] = prefix | additional;
    match width {
        1 => {}
        2 => output[1] = value as u8,
        3 => output[1..3].copy_from_slice(&(value as u16).to_be_bytes()),
        5 => output[1..5].copy_from_slice(&(value as u32).to_be_bytes()),
        9 => output[1..9].copy_from_slice(&value.to_be_bytes()),
        _ => unreachable!(),
    }
    Ok(width)
}

pub fn encode_uint(value: u64, output: &mut [u8]) -> Result<usize, CborError> {
    encode_head(0, value, output)
}

pub fn encode_bytes(value: &[u8], output: &mut [u8]) -> Result<usize, CborError> {
    let head_size = encode_head(2, value.len() as u64, output)?;
    let end = head_size + value.len();
    if output.len() < end {
        return Err(CborError::OutputTooSmall);
    }
    output[head_size..end].copy_from_slice(value);
    Ok(end)
}

pub fn encode_text(value: &str, output: &mut [u8]) -> Result<usize, CborError> {
    let head_size = encode_head(3, value.len() as u64, output)?;
    let end = head_size + value.len();
    if output.len() < end {
        return Err(CborError::OutputTooSmall);
    }
    output[head_size..end].copy_from_slice(value.as_bytes());
    Ok(end)
}

fn decode_head(input: &[u8], expected_major: u8) -> Result<(u64, usize), CborError> {
    let first = *input.first().ok_or(CborError::Empty)?;
    if first >> 5 != expected_major {
        return Err(CborError::WrongMajorType);
    }
    let additional = first & 0x1f;
    let (value, width) = match additional {
        0..=23 => (additional as u64, 1),
        24 => (*input.get(1).ok_or(CborError::Truncated)? as u64, 2),
        25 => (
            u16::from_be_bytes([
                *input.get(1).ok_or(CborError::Truncated)?,
                *input.get(2).ok_or(CborError::Truncated)?,
            ]) as u64,
            3,
        ),
        26 => {
            let bytes = [
                *input.get(1).ok_or(CborError::Truncated)?,
                *input.get(2).ok_or(CborError::Truncated)?,
                *input.get(3).ok_or(CborError::Truncated)?,
                *input.get(4).ok_or(CborError::Truncated)?,
            ];
            (u32::from_be_bytes(bytes) as u64, 5)
        }
        27 => {
            let mut bytes = [0; 8];
            for (index, byte) in bytes.iter_mut().enumerate() {
                *byte = *input.get(index + 1).ok_or(CborError::Truncated)?;
            }
            (u64::from_be_bytes(bytes), 9)
        }
        _ => return Err(CborError::NonCanonical),
    };
    let canonical_width = match value {
        0..=23 => 1,
        24..=255 => 2,
        256..=65_535 => 3,
        65_536..=4_294_967_295 => 5,
        _ => 9,
    };
    if width != canonical_width {
        return Err(CborError::NonCanonical);
    }
    Ok((value, width))
}

pub fn decode_uint(input: &[u8]) -> Result<(u64, usize), CborError> {
    decode_head(input, 0)
}

pub fn decode_bytes(input: &[u8]) -> Result<(&[u8], usize), CborError> {
    let (length, head_size) = decode_head(input, 2)?;
    let length = usize::try_from(length).map_err(|_| CborError::Truncated)?;
    let end = head_size.checked_add(length).ok_or(CborError::Truncated)?;
    if input.len() < end {
        return Err(CborError::Truncated);
    }
    Ok((&input[head_size..end], end))
}

pub fn decode_text(input: &[u8]) -> Result<(&str, usize), CborError> {
    let (bytes, used) = decode_head(input, 3).and_then(|(length, head_size)| {
        let length = usize::try_from(length).map_err(|_| CborError::Truncated)?;
        let end = head_size.checked_add(length).ok_or(CborError::Truncated)?;
        if input.len() < end {
            return Err(CborError::Truncated);
        }
        Ok((&input[head_size..end], end))
    })?;
    let value = str::from_utf8(bytes).map_err(|_| CborError::InvalidUtf8)?;
    Ok((value, used))
}

#[cfg(test)]
mod tests {
    use super::{
        decode_bytes, decode_text, decode_uint, encode_bytes, encode_text, encode_uint, CborError,
    };

    #[test]
    fn inteiro_usa_a_menor_representacao() {
        let mut output = [0; 9];
        assert_eq!(encode_uint(23, &mut output), Ok(1));
        assert_eq!(&output[..1], &[0x17]);
        assert_eq!(encode_uint(24, &mut output), Ok(2));
        assert_eq!(&output[..2], &[0x18, 0x18]);
    }

    #[test]
    fn rejeita_inteiro_nao_canonico() {
        assert_eq!(decode_uint(&[0x18, 0x17]), Err(CborError::NonCanonical));
    }

    #[test]
    fn bytes_e_texto_fazem_roundtrip() {
        let mut output = [0; 16];
        let used = encode_bytes(b"key", &mut output).unwrap();
        assert_eq!(decode_bytes(&output[..used]).unwrap(), (b"key" as &[u8], 4));
        let used = encode_text("ok", &mut output).unwrap();
        assert_eq!(decode_text(&output[..used]).unwrap(), ("ok", 3));
    }

    #[test]
    fn rejeita_texto_invalido() {
        assert_eq!(
            decode_text(&[0x63, 0xff, 0xff, 0xff]),
            Err(CborError::InvalidUtf8)
        );
    }
}
