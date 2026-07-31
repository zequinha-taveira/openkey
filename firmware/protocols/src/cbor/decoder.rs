//! Decodificador CBOR Canônico Estático (`no_std`, RFC 8949)

use crate::cbor::error::{CborError, Result};
use crate::cbor::value::{compare_canonical_map_keys, CborValue};
use core::cmp::Ordering;
use core::str;

/// Decodificador CBOR com validação estrita de regras de canonicidade
#[derive(Debug, Clone)]
pub struct CborDecoder<'a> {
    buffer: &'a [u8],
    position: usize,
}

impl<'a> CborDecoder<'a> {
    /// Cria um novo decodificador apontando para o buffer fornecido
    pub fn new(buffer: &'a [u8]) -> Self {
        Self {
            buffer,
            position: 0,
        }
    }

    /// Retorna o número de bytes restantes no buffer
    pub fn remaining_bytes(&self) -> usize {
        self.buffer.len() - self.position
    }

    /// Retorna a posição atual de leitura
    pub fn position(&self) -> usize {
        self.position
    }

    /// Retorna o slice restante não decodificado
    pub fn remaining_slice(&self) -> &'a [u8] {
        &self.buffer[self.position..]
    }

    /// Garante que todos os bytes do buffer foram consumidos (sem bytes sobressalentes)
    pub fn finish(self) -> Result<()> {
        if self.position == self.buffer.len() {
            Ok(())
        } else {
            Err(CborError::TrailingBytes)
        }
    }

    /// Visualiza o próximo byte sem avançar a posição
    pub fn peek_byte(&self) -> Result<u8> {
        if self.position < self.buffer.len() {
            Ok(self.buffer[self.position])
        } else {
            Err(CborError::UnexpectedEof)
        }
    }

    /// Lê um único byte do buffer
    pub fn read_byte(&mut self) -> Result<u8> {
        if self.position < self.buffer.len() {
            let b = self.buffer[self.position];
            self.position += 1;
            Ok(b)
        } else {
            Err(CborError::UnexpectedEof)
        }
    }

    /// Lê `n` bytes do buffer como uma fatia `&'a [u8]`
    pub fn read_bytes(&mut self, n: usize) -> Result<&'a [u8]> {
        if self.position + n <= self.buffer.len() {
            let slice = &self.buffer[self.position..self.position + n];
            self.position += n;
            Ok(slice)
        } else {
            Err(CborError::UnexpectedEof)
        }
    }

    /// Decodifica o cabeçalho CBOR (major_type, valor, bytes_lidos) com validação de canonicidade
    pub fn decode_header(&mut self) -> Result<(u8, u64)> {
        let initial_byte = self.read_byte()?;
        let major_type = initial_byte >> 5;
        let info = initial_byte & 0x1f;

        let val = match info {
            0..=23 => info as u64,
            24 => {
                let v = self.read_byte()? as u64;
                if v < 24 {
                    return Err(CborError::NonCanonicalIntEncoding);
                }
                v
            }
            25 => {
                let bytes = self.read_bytes(2)?;
                let v = u16::from_be_bytes([bytes[0], bytes[1]]) as u64;
                if v <= 0xff {
                    return Err(CborError::NonCanonicalIntEncoding);
                }
                v
            }
            26 => {
                let bytes = self.read_bytes(4)?;
                let v = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as u64;
                if v <= 0xffff {
                    return Err(CborError::NonCanonicalIntEncoding);
                }
                v
            }
            27 => {
                let bytes = self.read_bytes(8)?;
                let v = u64::from_be_bytes([
                    bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                ]);
                if v <= 0xffff_ffff {
                    return Err(CborError::NonCanonicalIntEncoding);
                }
                v
            }
            31 => return Err(CborError::IndefiniteLengthDisallowed),
            _ => return Err(CborError::InvalidMajorType(major_type)),
        };

        Ok((major_type, val))
    }

    /// Decodifica o próximo elemento CBOR como um `CborValue`
    pub fn decode_value(&mut self) -> Result<CborValue<'a>> {
        let (major_type, val) = self.decode_header()?;
        match major_type {
            0 => Ok(CborValue::Unsigned(val)),
            1 => Ok(CborValue::Negative(val)),
            2 => {
                let len = val as usize;
                let bytes = self.read_bytes(len)?;
                Ok(CborValue::ByteString(bytes))
            }
            3 => {
                let len = val as usize;
                let bytes = self.read_bytes(len)?;
                let s = str::from_utf8(bytes).map_err(|_| CborError::StringNotUtf8)?;
                Ok(CborValue::TextString(s))
            }
            4 => {
                if val > u32::MAX as u64 {
                    return Err(CborError::DepthLimitExceeded);
                }
                Ok(CborValue::ArrayHeader(val as u32))
            }
            5 => {
                if val > u32::MAX as u64 {
                    return Err(CborError::DepthLimitExceeded);
                }
                Ok(CborValue::MapHeader(val as u32))
            }
            7 => match val {
                20 => Ok(CborValue::Bool(false)),
                21 => Ok(CborValue::Bool(true)),
                22 => Ok(CborValue::Null),
                other => Err(CborError::UnsupportedSimpleValue(other as u8)),
            },
            other => Err(CborError::InvalidMajorType(other)),
        }
    }

    /// Decodifica especificamente um inteiro sem sinal (Major Type 0)
    pub fn decode_unsigned(&mut self) -> Result<u64> {
        let (mt, val) = self.decode_header()?;
        if mt == 0 {
            Ok(val)
        } else {
            Err(CborError::InvalidMajorType(mt))
        }
    }

    /// Decodifica um inteiro assinado (Major Type 0 ou 1)
    pub fn decode_int(&mut self) -> Result<i64> {
        let (mt, val) = self.decode_header()?;
        match mt {
            0 => {
                if val <= i64::MAX as u64 {
                    Ok(val as i64)
                } else {
                    Err(CborError::NonCanonicalIntEncoding)
                }
            }
            1 => {
                if val <= (i64::MAX as u64) {
                    Ok(-1 - (val as i64))
                } else {
                    Err(CborError::NonCanonicalIntEncoding)
                }
            }
            _ => Err(CborError::InvalidMajorType(mt)),
        }
    }

    /// Decodifica um ByteString (`&'a [u8]`) (Major Type 2)
    pub fn decode_bytes(&mut self) -> Result<&'a [u8]> {
        let (mt, val) = self.decode_header()?;
        if mt == 2 {
            self.read_bytes(val as usize)
        } else {
            Err(CborError::InvalidMajorType(mt))
        }
    }

    /// Decodifica um TextString (`&'a str`) (Major Type 3)
    pub fn decode_str(&mut self) -> Result<&'a str> {
        let (mt, val) = self.decode_header()?;
        if mt == 3 {
            let bytes = self.read_bytes(val as usize)?;
            str::from_utf8(bytes).map_err(|_| CborError::StringNotUtf8)
        } else {
            Err(CborError::InvalidMajorType(mt))
        }
    }

    /// Decodifica um cabeçalho de Array (Major Type 4) e retorna a contagem de itens
    pub fn decode_array_header(&mut self) -> Result<u32> {
        let (mt, val) = self.decode_header()?;
        if mt == 4 {
            Ok(val as u32)
        } else {
            Err(CborError::InvalidMajorType(mt))
        }
    }

    /// Decodifica um cabeçalho de Mapa (Major Type 5) e retorna a contagem de pares
    pub fn decode_map_header(&mut self) -> Result<u32> {
        let (mt, val) = self.decode_header()?;
        if mt == 5 {
            Ok(val as u32)
        } else {
            Err(CborError::InvalidMajorType(mt))
        }
    }

    /// Captura os bytes crus de um elemento CBOR completo avançando o cursor sobre ele.
    /// Útil para validação de ordenação de chaves em mapas.
    pub fn skip_value_slice(&mut self) -> Result<&'a [u8]> {
        let start_pos = self.position;
        self.skip_value()?;
        let end_pos = self.position;
        Ok(&self.buffer[start_pos..end_pos])
    }

    /// Pula um elemento CBOR completo recursivamente
    pub fn skip_value(&mut self) -> Result<()> {
        let (mt, val) = self.decode_header()?;
        match mt {
            0 | 1 => Ok(()),
            2 | 3 => {
                self.read_bytes(val as usize)?;
                Ok(())
            }
            4 => {
                let count = val as u32;
                for _ in 0..count {
                    self.skip_value()?;
                }
                Ok(())
            }
            5 => {
                let count = val as u32;
                for _ in 0..count {
                    self.skip_value()?; // key
                    self.skip_value()?; // value
                }
                Ok(())
            }
            7 => Ok(()),
            other => Err(CborError::InvalidMajorType(other)),
        }
    }

    /// Decodifica um mapa validando estritamente a ordenação canônica de chaves (RFC 8949 4.2.1)
    /// e que não existem chaves duplicadas.
    /// A closure `entry_callback(key_decoder, val_decoder)` é chamada para cada par.
    pub fn decode_map_canonical<F>(&mut self, mut entry_callback: F) -> Result<()>
    where
        F: FnMut(&mut CborDecoder<'a>) -> Result<()>,
    {
        let count = self.decode_map_header()?;
        let mut prev_key_start: Option<(usize, usize)> = None;

        for _ in 0..count {
            let key_start = self.position;
            // Valida chave lendo os bytes inteiros da chave
            self.skip_value()?;
            let key_end = self.position;

            let current_key_slice = &self.buffer[key_start..key_end];

            if let Some((prev_start, prev_end)) = prev_key_start {
                let prev_key_slice = &self.buffer[prev_start..prev_end];
                match compare_canonical_map_keys(prev_key_slice, current_key_slice) {
                    Ordering::Less => {}
                    Ordering::Equal => return Err(CborError::DuplicateMapKey),
                    Ordering::Greater => return Err(CborError::NonCanonicalMapOrdering),
                }
            }

            prev_key_start = Some((key_start, key_end));

            // Reposiciona para decodificar a chave real e o valor via callback
            let mut key_decoder = CborDecoder {
                buffer: self.buffer,
                position: key_start,
            };

            entry_callback(&mut key_decoder)?;

            // Avança o decoder principal até o final do valor (key_decoder já leu a chave e a callback leu o valor)
            self.position = key_decoder.position;
        }

        Ok(())
    }
}
