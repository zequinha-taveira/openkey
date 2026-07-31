//! Erros de codificação e decodificação CBOR (RFC 8949)

use core::fmt;

/// Erros que podem ocorrer no parsing ou escrita de CBOR
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CborError {
    /// Fim inesperado do buffer de dados
    UnexpectedEof,
    /// Codificação de inteiro não-mínima / não-canônica (RFC 8949 4.2)
    NonCanonicalIntEncoding,
    /// Ordenação de chaves em mapa não-canônica (RFC 8949 4.2.1)
    NonCanonicalMapOrdering,
    /// Chave duplicada encontrada em um mapa
    DuplicateMapKey,
    /// Bytes adicionais após o elemento CBOR principal
    TrailingBytes,
    /// Profundidade máxima de aninhamento excedida
    DepthLimitExceeded,
    /// Tipo de dado (Major Type) inválido
    InvalidMajorType(u8),
    /// Valor simples não suportado (Major Type 7)
    UnsupportedSimpleValue(u8),
    /// String de texto com codificação UTF-8 inválida
    StringNotUtf8,
    /// Buffer de saída pequeno demais para codificar o valor
    BufferTooSmall,
    /// Tamanho indefinido (*indefinite length*) não permitido em CBOR canônico
    IndefiniteLengthDisallowed,
}

impl fmt::Display for CborError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEof => write!(f, "CBOR: Fim inesperado do buffer"),
            Self::NonCanonicalIntEncoding => write!(f, "CBOR: Codificação de inteiro não-canônica"),
            Self::NonCanonicalMapOrdering => write!(f, "CBOR: Ordenação de mapa não-canônica"),
            Self::DuplicateMapKey => write!(f, "CBOR: Chave duplicada no mapa"),
            Self::TrailingBytes => write!(f, "CBOR: Bytes extras ao final do payload"),
            Self::DepthLimitExceeded => write!(f, "CBOR: Profundidade limite excedida"),
            Self::InvalidMajorType(mt) => write!(f, "CBOR: Major Type inválido {}", mt),
            Self::UnsupportedSimpleValue(v) => write!(f, "CBOR: Valor simples não suportado {}", v),
            Self::StringNotUtf8 => write!(f, "CBOR: String com UTF-8 inválido"),
            Self::BufferTooSmall => write!(f, "CBOR: Buffer de saída insuficiente"),
            Self::IndefiniteLengthDisallowed => write!(f, "CBOR: Tamanho indefinido não permitido"),
        }
    }
}

pub type Result<T> = core::result::Result<T, CborError>;
