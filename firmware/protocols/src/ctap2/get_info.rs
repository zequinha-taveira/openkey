//! Comando `authenticatorGetInfo` (0x04) do CTAP2

use crate::cbor::{CborEncoder, Result};

/// Estrutura contendo informações e capacidades do autenticador
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetInfoResponse<'a> {
    pub versions: &'a [&'static str],
    pub extensions: &'a [&'static str],
    pub aaguid: [u8; 16],
    pub rk: bool,
    pub up: bool,
    pub plat: bool,
    pub client_pin: bool,
    pub max_msg_size: u32,
    pub pin_uv_auth_protocols: &'a [u8],
}

impl<'a> GetInfoResponse<'a> {
    /// Cria uma resposta `GetInfoResponse` com valores padrão do OpenKey
    pub fn default_openkey(aaguid: [u8; 16], pin_set: bool) -> Self {
        Self {
            versions: &["FIDO_2_0", "FIDO_2_1"],
            extensions: &["hmac-secret", "credProtect"],
            aaguid,
            rk: true,
            up: true,
            plat: false,
            client_pin: pin_set,
            max_msg_size: 1200,
            pin_uv_auth_protocols: &[1, 2],
        }
    }

    /// Serializa a resposta em um mapa CBOR estritamente ordenado por chaves canônicas (RFC 8949)
    /// Chaves do mapa `authenticatorGetInfo`:
    /// Key 1 (0x01): versions (Array of str)
    /// Key 2 (0x02): extensions (Array of str)
    /// Key 3 (0x03): aaguid (ByteString 16 bytes)
    /// Key 4 (0x04): options (Map { "clientPin": bool, "plat": bool, "rk": bool, "up": bool })
    /// Key 5 (0x05): maxMsgSize (Unsigned)
    /// Key 6 (0x06): pinUvAuthProtocols (Array of Unsigned)
    pub fn encode_cbor(&self, out_buf: &mut [u8]) -> Result<usize> {
        let mut enc = CborEncoder::new(out_buf);
        enc.encode_map_header(6)?;

        // Key 1: versions
        enc.encode_int(1)?;
        enc.encode_array_header(self.versions.len() as u32)?;
        for v in self.versions {
            enc.encode_str(v)?;
        }

        // Key 2: extensions
        enc.encode_int(2)?;
        enc.encode_array_header(self.extensions.len() as u32)?;
        for ext in self.extensions {
            enc.encode_str(ext)?;
        }

        // Key 3: aaguid
        enc.encode_int(3)?;
        enc.encode_bytes(&self.aaguid)?;

        // Key 4: options (map)
        // Canonical map keys for options: "clientPin" (len 9), "plat" (len 4), "rk" (len 2), "up" (len 2)
        // Canonical order: "rk", "up", "plat", "clientPin"
        enc.encode_int(4)?;
        enc.encode_map_header(4)?;
        enc.encode_str("rk")?;
        enc.encode_bool(self.rk)?;
        enc.encode_str("up")?;
        enc.encode_bool(self.up)?;
        enc.encode_str("plat")?;
        enc.encode_bool(self.plat)?;
        enc.encode_str("clientPin")?;
        enc.encode_bool(self.client_pin)?;

        // Key 5: maxMsgSize
        enc.encode_int(5)?;
        enc.encode_unsigned(self.max_msg_size as u64)?;

        // Key 6: pinUvAuthProtocols
        enc.encode_int(6)?;
        enc.encode_array_header(self.pin_uv_auth_protocols.len() as u32)?;
        for &proto in self.pin_uv_auth_protocols {
            enc.encode_unsigned(proto as u64)?;
        }

        Ok(enc.position())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cbor::CborDecoder;

    #[test]
    fn test_get_info_response_encoding() {
        let aaguid = [0x01; 16];
        let resp = GetInfoResponse::default_openkey(aaguid, false);
        let mut buf = [0u8; 512];
        let len = resp.encode_cbor(&mut buf).unwrap();

        let mut dec = CborDecoder::new(&buf[..len]);
        dec.decode_map_canonical(|entry_dec| {
            entry_dec.skip_value()?;
            entry_dec.skip_value()?;
            Ok(())
        })
        .unwrap();
    }
}
