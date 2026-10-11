//! Módulo COSE (RFC 9052) — COSE_Sign1 para WebAuthn / CTAP2

use crate::cbor::{CborDecoder, CborEncoder, CborError, Result};

/// Identificador de Algoritmo COSE para ES256 (ECDSA P-256 + SHA-256)
pub const COSE_ALG_ES256: i64 = -7;
/// Identificador de Algoritmo COSE para EdDSA (Ed25519)
pub const COSE_ALG_EDDSA: i64 = -8;

/// Algoritmos criptográficos suportados em COSE Sign1
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoseAlgorithm {
    /// ECDSA sobre curva secp256r1 (P-256) com SHA-256 (-7)
    Es256,
    /// Ed25519 / EdDSA (-8)
    EdDsa,
}

impl CoseAlgorithm {
    /// Retorna o valor numérico do algoritmo COSE
    pub fn to_i64(&self) -> i64 {
        match self {
            Self::Es256 => COSE_ALG_ES256,
            Self::EdDsa => COSE_ALG_EDDSA,
        }
    }

    /// Tenta converter um `i64` para `CoseAlgorithm`
    pub fn from_i64(val: i64) -> Option<Self> {
        match val {
            COSE_ALG_ES256 => Some(Self::Es256),
            COSE_ALG_EDDSA => Some(Self::EdDsa),
            _ => None,
        }
    }
}

/// Estrutura de cabeçalho e payload COSE Sign1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoseSign1<'a> {
    /// Bytes brutos do protected header (mapa CBOR serializado em ByteString)
    pub protected_bytes: &'a [u8],
    /// Algoritmo extraído do protected header
    pub algorithm: CoseAlgorithm,
    /// Payload de dados assinados
    pub payload: &'a [u8],
    /// Assinatura criptográfica
    pub signature: &'a [u8],
}

/// Constrói o `protected_header` em CBOR para um dado algoritmo (ex: `{1: -7}`)
pub fn encode_protected_header(alg: CoseAlgorithm, out_buf: &mut [u8]) -> Result<usize> {
    let mut enc = CborEncoder::new(out_buf);
    enc.encode_map_header(1)?;
    enc.encode_int(1)?; // Header param 1 = 'alg'
    enc.encode_int(alg.to_i64())?;
    Ok(enc.position())
}

/// Constrói a estrutura `Sig_structure` (RFC 9052 Section 4.4) para assinatura/verificação:
/// ```cbor
/// Sig_structure = [
///     context: "Signature1",
///     body_protected: bstr,
///     external_aad: bstr,
///     payload: bstr
/// ]
/// ```
pub fn encode_sig_structure(
    protected_bytes: &[u8],
    external_aad: &[u8],
    payload: &[u8],
    out_buf: &mut [u8],
) -> Result<usize> {
    let mut enc = CborEncoder::new(out_buf);
    enc.encode_array_header(4)?;
    enc.encode_str("Signature1")?;
    enc.encode_bytes(protected_bytes)?;
    enc.encode_bytes(external_aad)?;
    enc.encode_bytes(payload)?;
    Ok(enc.position())
}

/// Empacota uma estrutura `COSE_Sign1` completa em CBOR:
/// ```cbor
/// COSE_Sign1 = [
///     protected: bstr,
///     unprotected: map ({}),
///     payload: bstr,
///     signature: bstr
/// ]
/// ```
pub fn encode_cose_sign1(
    protected_bytes: &[u8],
    payload: &[u8],
    signature: &[u8],
    out_buf: &mut [u8],
) -> Result<usize> {
    let mut enc = CborEncoder::new(out_buf);
    enc.encode_array_header(4)?;
    enc.encode_bytes(protected_bytes)?;
    enc.encode_map_header(0)?; // Unprotected header vazio ({})
    enc.encode_bytes(payload)?;
    enc.encode_bytes(signature)?;
    Ok(enc.position())
}

/// Parseia e decodifica uma estrutura `COSE_Sign1` a partir de um payload CBOR
pub fn parse_cose_sign1<'a>(cbor_bytes: &'a [u8]) -> Result<CoseSign1<'a>> {
    let mut dec = CborDecoder::new(cbor_bytes);
    let count = dec.decode_array_header()?;
    if count != 4 {
        return Err(CborError::InvalidMajorType(4));
    }

    // 1. Protected header (bstr)
    let protected_bytes = dec.decode_bytes()?;

    // Parse do algoritmo dentro do protected header
    let mut prot_dec = CborDecoder::new(protected_bytes);
    let map_count = prot_dec.decode_map_header()?;
    let mut algorithm = None;

    for _ in 0..map_count {
        let key = prot_dec.decode_int()?;
        let val = prot_dec.decode_int()?;
        if key == 1 {
            algorithm = CoseAlgorithm::from_i64(val);
        }
    }

    let algorithm = algorithm.ok_or(CborError::UnsupportedSimpleValue(0))?;

    // 2. Unprotected header (map - ignorado / deve ser lido)
    dec.skip_value(0)?;

    // 3. Payload (bstr)
    let payload = dec.decode_bytes()?;

    // 4. Signature (bstr)
    let signature = dec.decode_bytes()?;

    dec.finish()?;

    Ok(CoseSign1 {
        protected_bytes,
        algorithm,
        payload,
        signature,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cose_sign1_encode_and_parse_roundtrip() {
        let mut prot_buf = [0u8; 16];
        let prot_len = encode_protected_header(CoseAlgorithm::Es256, &mut prot_buf).unwrap();
        let protected_bytes = &prot_buf[..prot_len];

        let payload = b"user authentication assertion payload";
        let signature = b"mock_ecdsa_p256_signature_64_bytes_long_mock_mock_mock_mock_mock";

        let mut cose_buf = [0u8; 256];
        let cose_len =
            encode_cose_sign1(protected_bytes, payload, signature, &mut cose_buf).unwrap();

        let parsed = parse_cose_sign1(&cose_buf[..cose_len]).unwrap();
        assert_eq!(parsed.algorithm, CoseAlgorithm::Es256);
        assert_eq!(parsed.protected_bytes, protected_bytes);
        assert_eq!(parsed.payload, payload);
        assert_eq!(parsed.signature, signature);
    }

    #[test]
    fn test_sig_structure_encoding() {
        let mut prot_buf = [0u8; 16];
        let prot_len = encode_protected_header(CoseAlgorithm::EdDsa, &mut prot_buf).unwrap();
        let protected_bytes = &prot_buf[..prot_len];

        let mut sig_struct_buf = [0u8; 128];
        let len = encode_sig_structure(protected_bytes, b"", b"data_to_sign", &mut sig_struct_buf)
            .unwrap();

        let mut dec = CborDecoder::new(&sig_struct_buf[..len]);
        assert_eq!(dec.decode_array_header().unwrap(), 4);
        assert_eq!(dec.decode_str().unwrap(), "Signature1");
        assert_eq!(dec.decode_bytes().unwrap(), protected_bytes);
        assert_eq!(dec.decode_bytes().unwrap(), b"");
        assert_eq!(dec.decode_bytes().unwrap(), b"data_to_sign");
        dec.finish().unwrap();
    }
}
