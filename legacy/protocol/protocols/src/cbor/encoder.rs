//! Codificador CBOR Canônico Estático (`no_std`, RFC 8949)

use crate::cbor::error::{CborError, Result};
use crate::cbor::value::CborValue;

/// Codificador CBOR canônico que grava diretamente em um buffer de bytes
#[derive(Debug)]
pub struct CborEncoder<'a> {
    buffer: &'a mut [u8],
    position: usize,
}

impl<'a> CborEncoder<'a> {
    /// Cria um novo codificador que utilizará o buffer fornecido
    pub fn new(buffer: &'a mut [u8]) -> Self {
        Self {
            buffer,
            position: 0,
        }
    }

    /// Retorna o número de bytes gravados até o momento
    pub fn position(&self) -> usize {
        self.position
    }

    /// Retorna a fatia de bytes gravada
    pub fn written_slice(&self) -> &[u8] {
        &self.buffer[..self.position]
    }

    /// Retorna a quantidade de espaço restante no buffer
    pub fn remaining_capacity(&self) -> usize {
        self.buffer.len() - self.position
    }

    /// Escreve um único byte no buffer
    pub fn write_byte(&mut self, b: u8) -> Result<()> {
        if self.position < self.buffer.len() {
            self.buffer[self.position] = b;
            self.position += 1;
            Ok(())
        } else {
            Err(CborError::BufferTooSmall)
        }
    }

    /// Escreve uma fatia de bytes no buffer
    pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        if self.position + bytes.len() <= self.buffer.len() {
            self.buffer[self.position..self.position + bytes.len()].copy_from_slice(bytes);
            self.position += bytes.len();
            Ok(())
        } else {
            Err(CborError::BufferTooSmall)
        }
    }

    /// Codifica o cabeçalho CBOR de forma estritamente canônica (menor representação possível)
    pub fn encode_header(&mut self, major_type: u8, val: u64) -> Result<()> {
        let mt_shifted = major_type << 5;
        match val {
            0..=23 => self.write_byte(mt_shifted | (val as u8)),
            24..=0xff => {
                self.write_byte(mt_shifted | 24)?;
                self.write_byte(val as u8)
            }
            0x100..=0xffff => {
                self.write_byte(mt_shifted | 25)?;
                self.write_bytes(&(val as u16).to_be_bytes())
            }
            0x1_0000..=0xffff_ffff => {
                self.write_byte(mt_shifted | 26)?;
                self.write_bytes(&(val as u32).to_be_bytes())
            }
            _ => {
                self.write_byte(mt_shifted | 27)?;
                self.write_bytes(&val.to_be_bytes())
            }
        }
    }

    /// Codifica um inteiro sem sinal `u64` (Major Type 0)
    pub fn encode_unsigned(&mut self, val: u64) -> Result<()> {
        self.encode_header(0, val)
    }

    /// Codifica um inteiro negativo `-1 - val` (Major Type 1)
    pub fn encode_negative(&mut self, val: u64) -> Result<()> {
        self.encode_header(1, val)
    }

    /// Codifica um inteiro assinado `i64` (Major Type 0 ou 1)
    pub fn encode_int(&mut self, val: i64) -> Result<()> {
        if val >= 0 {
            self.encode_unsigned(val as u64)
        } else {
            let neg_val = (-1 - val) as u64;
            self.encode_negative(neg_val)
        }
    }

    /// Codifica uma sequência de bytes `&[u8]` (Major Type 2)
    pub fn encode_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        self.encode_header(2, bytes.len() as u64)?;
        self.write_bytes(bytes)
    }

    /// Codifica uma string de texto UTF-8 `&str` (Major Type 3)
    pub fn encode_str(&mut self, s: &str) -> Result<()> {
        self.encode_header(3, s.len() as u64)?;
        self.write_bytes(s.as_bytes())
    }

    /// Codifica um cabeçalho de Array (Major Type 4) com a contagem de itens
    pub fn encode_array_header(&mut self, len: u32) -> Result<()> {
        self.encode_header(4, len as u64)
    }

    /// Codifica um cabeçalho de Mapa (Major Type 5) com a contagem de pares
    pub fn encode_map_header(&mut self, len: u32) -> Result<()> {
        self.encode_header(5, len as u64)
    }

    /// Codifica um booleano (Major Type 7: 20=false, 21=true)
    pub fn encode_bool(&mut self, val: bool) -> Result<()> {
        if val {
            self.write_byte(0xf5) // Major 7 (0xe0) | 21 (0x15)
        } else {
            self.write_byte(0xf4) // Major 7 (0xe0) | 20 (0x14)
        }
    }

    /// Codifica o valor `null` (Major Type 7: 22)
    pub fn encode_null(&mut self) -> Result<()> {
        self.write_byte(0xf6) // Major 7 (0xe0) | 22 (0x16)
    }

    /// Codifica um `CborValue` arbitrário
    pub fn encode_value(&mut self, value: &CborValue<'_>) -> Result<()> {
        match *value {
            CborValue::Unsigned(val) => self.encode_unsigned(val),
            CborValue::Negative(val) => self.encode_negative(val),
            CborValue::ByteString(bytes) => self.encode_bytes(bytes),
            CborValue::TextString(s) => self.encode_str(s),
            CborValue::ArrayHeader(len) => self.encode_array_header(len),
            CborValue::MapHeader(len) => self.encode_map_header(len),
            CborValue::Bool(b) => self.encode_bool(b),
            CborValue::Null => self.encode_null(),
        }
    }
}
