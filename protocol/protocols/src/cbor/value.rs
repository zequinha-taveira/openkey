//! Tipos de dados CBOR (RFC 8949) com representação zero-copy

use core::cmp::Ordering;

/// Tipos básicos de dados CBOR decodificados sem alocação dinânica
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CborValue<'a> {
    /// Major Type 0: Inteiro sem sinal (0 ..= 2^64-1)
    Unsigned(u64),
    /// Major Type 1: Inteiro negativo (-1 - n, onde n é u64)
    Negative(u64),
    /// Major Type 2: Sequência de bytes (`&[u8]`)
    ByteString(&'a [u8]),
    /// Major Type 3: String de texto UTF-8 (`&str`)
    TextString(&'a str),
    /// Major Type 4: Cabeçalho de Array (número de elementos)
    ArrayHeader(u32),
    /// Major Type 5: Cabeçalho de Mapa (número de pares chave-valor)
    MapHeader(u32),
    /// Major Type 7: Booleano (`true` / `false`)
    Bool(bool),
    /// Major Type 7: Nulo (`null`)
    Null,
}

impl<'a> CborValue<'a> {
    /// Retorna o valor numérico assinado se for um inteiro (Unsigned ou Negative)
    pub fn as_i64(&self) -> Option<i64> {
        match *self {
            CborValue::Unsigned(val) => {
                if val <= i64::MAX as u64 {
                    Some(val as i64)
                } else {
                    None
                }
            }
            CborValue::Negative(val) => {
                if val <= (i64::MAX as u64) {
                    Some(-1 - (val as i64))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Retorna `u64` se for `Unsigned`
    pub fn as_u64(&self) -> Option<u64> {
        match *self {
            CborValue::Unsigned(val) => Some(val),
            _ => None,
        }
    }

    /// Retorna a fatia de bytes se for `ByteString`
    pub fn as_bytes(&self) -> Option<&'a [u8]> {
        match *self {
            CborValue::ByteString(bytes) => Some(bytes),
            _ => None,
        }
    }

    /// Retorna a string UTF-8 se for `TextString`
    pub fn as_str(&self) -> Option<&'a str> {
        match *self {
            CborValue::TextString(s) => Some(s),
            _ => None,
        }
    }

    /// Retorna o booleano se for `Bool`
    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            CborValue::Bool(b) => Some(b),
            _ => None,
        }
    }
}

/// Compara dois slices de bytes codificados em CBOR segundo a regra RFC 8949 Section 4.2.1:
/// Canonical Map Key Sorting Requirement:
/// 1. Slices com comprimentos diferentes: o slice mais curto vem PRIMEIRO.
/// 2. Slices com o mesmo comprimento: ordem lexicográfica por bytes.
pub fn compare_canonical_map_keys(a: &[u8], b: &[u8]) -> Ordering {
    if a.len() != b.len() {
        a.len().cmp(&b.len())
    } else {
        a.cmp(b)
    }
}
