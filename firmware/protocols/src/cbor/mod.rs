//! Módulo de processamento CBOR Canônico (RFC 8949)

pub mod decoder;
pub mod encoder;
pub mod error;
pub mod value;

pub use decoder::CborDecoder;
pub use encoder::CborEncoder;
pub use error::{CborError, Result};
pub use value::{compare_canonical_map_keys, CborValue};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_integer_encoding_decoding() {
        let mut buf = [0u8; 32];

        // 1. Small int: 15 (1 byte)
        let mut enc = CborEncoder::new(&mut buf);
        enc.encode_unsigned(15).unwrap();
        assert_eq!(enc.written_slice(), &[15]);

        let mut dec = CborDecoder::new(enc.written_slice());
        assert_eq!(dec.decode_unsigned().unwrap(), 15);

        // 2. 1-byte payload: 100 (2 bytes: 0x18 0x64)
        let mut enc = CborEncoder::new(&mut buf);
        enc.encode_unsigned(100).unwrap();
        assert_eq!(enc.written_slice(), &[0x18, 100]);

        let mut dec = CborDecoder::new(enc.written_slice());
        assert_eq!(dec.decode_unsigned().unwrap(), 100);

        // 3. 2-byte payload: 1000 (3 bytes: 0x19 0x03 0xe8)
        let mut enc = CborEncoder::new(&mut buf);
        enc.encode_unsigned(1000).unwrap();
        assert_eq!(enc.written_slice(), &[0x19, 0x03, 0xe8]);

        let mut dec = CborDecoder::new(enc.written_slice());
        assert_eq!(dec.decode_unsigned().unwrap(), 1000);
    }

    #[test]
    fn test_non_canonical_integer_rejection() {
        // Encoding 5 as 2-byte 0x18 0x05 is NON-CANONICAL
        let non_canonical_5 = [0x18, 0x05];
        let mut dec = CborDecoder::new(&non_canonical_5);
        assert_eq!(
            dec.decode_unsigned(),
            Err(CborError::NonCanonicalIntEncoding)
        );

        // Encoding 100 as 3-byte 0x19 0x00 0x64 is NON-CANONICAL
        let non_canonical_100 = [0x19, 0x00, 0x64];
        let mut dec = CborDecoder::new(&non_canonical_100);
        assert_eq!(
            dec.decode_unsigned(),
            Err(CborError::NonCanonicalIntEncoding)
        );
    }

    #[test]
    fn test_signed_integer_roundtrip() {
        let mut buf = [0u8; 16];

        // Zero
        let mut enc = CborEncoder::new(&mut buf);
        enc.encode_int(0).unwrap();
        let mut dec = CborDecoder::new(enc.written_slice());
        assert_eq!(dec.decode_int().unwrap(), 0);

        // Positive
        let mut enc = CborEncoder::new(&mut buf);
        enc.encode_int(42).unwrap();
        let mut dec = CborDecoder::new(enc.written_slice());
        assert_eq!(dec.decode_int().unwrap(), 42);

        // Negative: -1
        let mut enc = CborEncoder::new(&mut buf);
        enc.encode_int(-1).unwrap();
        assert_eq!(enc.written_slice(), &[0x20]); // Major 1, val 0 => -1 - 0 = -1
        let mut dec = CborDecoder::new(enc.written_slice());
        assert_eq!(dec.decode_int().unwrap(), -1);

        // Negative: -500
        let mut enc = CborEncoder::new(&mut buf);
        enc.encode_int(-500).unwrap();
        let mut dec = CborDecoder::new(enc.written_slice());
        assert_eq!(dec.decode_int().unwrap(), -500);
    }

    #[test]
    fn test_byte_and_text_strings() {
        let mut buf = [0u8; 64];

        // ByteString
        let mut enc = CborEncoder::new(&mut buf);
        enc.encode_bytes(b"openkey").unwrap();
        let mut dec = CborDecoder::new(enc.written_slice());
        assert_eq!(dec.decode_bytes().unwrap(), b"openkey");

        // TextString
        let mut enc = CborEncoder::new(&mut buf);
        enc.encode_str("FIDO2 CTAP2").unwrap();
        let mut dec = CborDecoder::new(enc.written_slice());
        assert_eq!(dec.decode_str().unwrap(), "FIDO2 CTAP2");
    }

    #[test]
    fn test_canonical_map_key_ordering() {
        // According to RFC 8949 4.2.1:
        // Key 1: 1 (encoded as 0x01, len 1)
        // Key 2: 2 (encoded as 0x02, len 1)
        // Key 3: "a" (encoded as 0x61 'a', len 2: 0x61 0x61)
        // Key 4: "aa" (encoded as 0x62 'a' 'a', len 3)

        let mut buf = [0u8; 128];
        let mut enc = CborEncoder::new(&mut buf);
        enc.encode_map_header(4).unwrap();

        // 1 => "one"
        enc.encode_int(1).unwrap();
        enc.encode_str("one").unwrap();

        // 2 => "two"
        enc.encode_int(2).unwrap();
        enc.encode_str("two").unwrap();

        // "a" => 10
        enc.encode_str("a").unwrap();
        enc.encode_int(10).unwrap();

        // "aa" => 20
        enc.encode_str("aa").unwrap();
        enc.encode_int(20).unwrap();

        let encoded_bytes = enc.written_slice();

        let mut dec = CborDecoder::new(encoded_bytes);
        let mut count = 0;
        dec.decode_map_canonical(|entry_dec| {
            count += 1;
            let key = entry_dec.decode_value().unwrap();
            let val = entry_dec.decode_value().unwrap();

            match count {
                1 => {
                    assert_eq!(key, CborValue::Unsigned(1));
                    assert_eq!(val, CborValue::TextString("one"));
                }
                2 => {
                    assert_eq!(key, CborValue::Unsigned(2));
                    assert_eq!(val, CborValue::TextString("two"));
                }
                3 => {
                    assert_eq!(key, CborValue::TextString("a"));
                    assert_eq!(val, CborValue::Unsigned(10));
                }
                4 => {
                    assert_eq!(key, CborValue::TextString("aa"));
                    assert_eq!(val, CborValue::Unsigned(20));
                }
                _ => panic!("Unexpected count"),
            }
            Ok(())
        })
        .unwrap();

        assert_eq!(count, 4);
    }

    #[test]
    fn test_non_canonical_map_key_ordering_rejection() {
        // Map with keys out of order: key 2 before key 1
        let mut buf = [0u8; 64];
        let mut enc = CborEncoder::new(&mut buf);
        enc.encode_map_header(2).unwrap();
        enc.encode_int(2).unwrap();
        enc.encode_str("two").unwrap();
        enc.encode_int(1).unwrap();
        enc.encode_str("one").unwrap();

        let mut dec = CborDecoder::new(enc.written_slice());
        let res = dec.decode_map_canonical(|entry_dec| {
            entry_dec.skip_value()?;
            entry_dec.skip_value()?;
            Ok(())
        });

        assert_eq!(res, Err(CborError::NonCanonicalMapOrdering));
    }

    #[test]
    fn test_duplicate_map_key_rejection() {
        // Map with duplicate key: key 1 twice
        let mut buf = [0u8; 64];
        let mut enc = CborEncoder::new(&mut buf);
        enc.encode_map_header(2).unwrap();
        enc.encode_int(1).unwrap();
        enc.encode_str("first").unwrap();
        enc.encode_int(1).unwrap();
        enc.encode_str("second").unwrap();

        let mut dec = CborDecoder::new(enc.written_slice());
        let res = dec.decode_map_canonical(|entry_dec| {
            entry_dec.skip_value()?;
            entry_dec.skip_value()?;
            Ok(())
        });

        assert_eq!(res, Err(CborError::DuplicateMapKey));
    }

    #[test]
    fn test_trailing_bytes_rejection() {
        let buf = [0x05, 0xff]; // 5 followed by extra byte 0xff
        let mut dec = CborDecoder::new(&buf);
        assert_eq!(dec.decode_unsigned().unwrap(), 5);
        assert_eq!(dec.finish(), Err(CborError::TrailingBytes));
    }
}
