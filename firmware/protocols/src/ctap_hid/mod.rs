//! Módulo CTAP HID Framing (FIDO CTAP2.1 Spec Section 11)

use crate::cbor::Result;

/// Tamanho fixo do pacote USB HID (64 bytes)
pub const CTAPHID_PACKET_SIZE: usize = 64;

/// Tamanho máximo do payload na primeira mensagem (Init packet: 64 - 4 (cid) - 1 (cmd) - 2 (len) = 57 bytes)
pub const CTAPHID_INIT_PAYLOAD_SIZE: usize = 57;

/// Tamanho máximo do payload nos pacotes de continuação (Cont packet: 64 - 4 (cid) - 1 (seq) = 59 bytes)
pub const CTAPHID_CONT_PAYLOAD_SIZE: usize = 59;

/// Canal de broadcast reservado (0xFFFFFFFF)
pub const CTAPHID_BROADCAST_CID: u32 = 0xffff_ffff;

/// Bit de comando (Bit 7 ativado em todos os pacotes de inicialização)
pub const CTAPHID_CMD_BIT: u8 = 0x80;

/// Comandos CTAP HID (FIDO CTAP2 Spec)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CtapHidCommand {
    /// Echo back payload
    Ping = 0x01,
    /// CTAP1 / U2F Raw Message
    Msg = 0x03,
    /// Alocação / Inicialização de Canal
    Init = 0x06,
    /// Piscada do LED de identificação
    Wink = 0x08,
    /// CTAP2 CBOR Message
    Cbor = 0x10,
    /// Cancelamento de operação
    Cancel = 0x11,
    /// Keep-alive status update
    KeepAlive = 0x3b,
    /// Resposta de erro
    Error = 0x3f,
}

impl CtapHidCommand {
    /// Converte do byte de comando (com bit 0x80 removido)
    pub fn from_u8(cmd_byte: u8) -> Option<Self> {
        let code = cmd_byte & !CTAPHID_CMD_BIT;
        match code {
            0x01 => Some(Self::Ping),
            0x03 => Some(Self::Msg),
            0x06 => Some(Self::Init),
            0x08 => Some(Self::Wink),
            0x10 => Some(Self::Cbor),
            0x11 => Some(Self::Cancel),
            0x3b => Some(Self::KeepAlive),
            0x3f => Some(Self::Error),
            _ => None,
        }
    }

    /// Retorna o valor de byte com o bit de comando (0x80) ativado
    pub fn to_u8(&self) -> u8 {
        (*self as u8) | CTAPHID_CMD_BIT
    }
}

/// Códigos de erro CTAP HID
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CtapHidErrorCode {
    InvalidCmd = 0x01,
    InvalidPar = 0x02,
    InvalidLen = 0x03,
    InvalidSeq = 0x04,
    MsgTimeout = 0x05,
    ChannelBusy = 0x06,
    LockRequired = 0x0a,
    InvalidChannel = 0x0b,
    Other = 0x7f,
}

/// Representação de um pacote CTAP HID bruto de 64 bytes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CtapHidPacket<'a> {
    /// Pacote de Inicialização
    Init {
        cid: u32,
        cmd: u8,
        payload_len: u16,
        data: &'a [u8],
    },
    /// Pacote de Continuação
    Cont { cid: u32, seq: u8, data: &'a [u8] },
}

impl<'a> CtapHidPacket<'a> {
    /// Parseia um buffer de 64 bytes em um `CtapHidPacket`
    pub fn parse(buf: &'a [u8; CTAPHID_PACKET_SIZE]) -> Option<Self> {
        let cid = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        let cmd_or_seq = buf[4];

        if (cmd_or_seq & CTAPHID_CMD_BIT) != 0 {
            // Pacote de Inicialização
            let payload_len = u16::from_be_bytes([buf[5], buf[6]]);
            let data = &buf[7..64];
            Some(Self::Init {
                cid,
                cmd: cmd_or_seq,
                payload_len,
                data,
            })
        } else {
            // Pacote de Continuação
            let seq = cmd_or_seq & 0x7f;
            let data = &buf[5..64];
            Some(Self::Cont { cid, seq, data })
        }
    }

    /// Serializa o pacote para um buffer de 64 bytes
    pub fn serialize(&self, out_buf: &mut [u8; CTAPHID_PACKET_SIZE]) {
        out_buf.fill(0);
        match self {
            Self::Init {
                cid,
                cmd,
                payload_len,
                data,
            } => {
                out_buf[0..4].copy_from_slice(&cid.to_be_bytes());
                out_buf[4] = *cmd;
                out_buf[5..7].copy_from_slice(&payload_len.to_be_bytes());
                let copy_len = data.len().min(CTAPHID_INIT_PAYLOAD_SIZE);
                out_buf[7..7 + copy_len].copy_from_slice(&data[..copy_len]);
            }
            Self::Cont { cid, seq, data } => {
                out_buf[0..4].copy_from_slice(&cid.to_be_bytes());
                out_buf[4] = *seq & 0x7f;
                let copy_len = data.len().min(CTAPHID_CONT_PAYLOAD_SIZE);
                out_buf[5..5 + copy_len].copy_from_slice(&data[..copy_len]);
            }
        }
    }
}

/// Montador e Fragmentador de mensagens CTAP HID
#[derive(Debug, Default)]
pub struct CtapHidMessageAssembler {
    active_cid: u32,
    active_cmd: u8,
    total_len: usize,
    received_len: usize,
    next_seq: u8,
}

impl CtapHidMessageAssembler {
    pub fn new() -> Self {
        Self {
            active_cid: 0,
            active_cmd: 0,
            total_len: 0,
            received_len: 0,
            next_seq: 0,
        }
    }

    pub fn reset(&mut self) {
        self.active_cid = 0;
        self.active_cmd = 0;
        self.total_len = 0;
        self.received_len = 0;
        self.next_seq = 0;
    }

    pub fn is_active(&self) -> bool {
        self.total_len > 0 && self.received_len < self.total_len
    }

    pub fn active_cid(&self) -> u32 {
        self.active_cid
    }

    /// Processa um pacote e copia dados no buffer de montagem de payload `out_payload`
    pub fn process_packet<'a>(
        &mut self,
        packet: &CtapHidPacket<'a>,
        out_payload: &mut [u8],
    ) -> Result<Option<(u32, u8, usize)>> {
        match packet {
            CtapHidPacket::Init {
                cid,
                cmd,
                payload_len,
                data,
            } => {
                let len = *payload_len as usize;
                if len > out_payload.len() {
                    return Err(crate::cbor::CborError::BufferTooSmall);
                }

                self.active_cid = *cid;
                self.active_cmd = *cmd;
                self.total_len = len;
                self.received_len = 0;
                self.next_seq = 0;

                let copy_bytes = data.len().min(len);
                out_payload[..copy_bytes].copy_from_slice(&data[..copy_bytes]);
                self.received_len = copy_bytes;

                if self.received_len >= self.total_len {
                    let res = (self.active_cid, self.active_cmd, self.total_len);
                    self.reset();
                    Ok(Some(res))
                } else {
                    Ok(None)
                }
            }
            CtapHidPacket::Cont { cid, seq, data } => {
                if !self.is_active() || *cid != self.active_cid {
                    return Err(crate::cbor::CborError::InvalidMajorType(0));
                }

                if *seq != self.next_seq {
                    self.reset();
                    return Err(crate::cbor::CborError::NonCanonicalIntEncoding);
                }

                self.next_seq = self.next_seq.wrapping_add(1);

                let remaining_needed = self.total_len - self.received_len;
                let copy_bytes = data.len().min(remaining_needed);

                out_payload[self.received_len..self.received_len + copy_bytes]
                    .copy_from_slice(&data[..copy_bytes]);
                self.received_len += copy_bytes;

                if self.received_len >= self.total_len {
                    let res = (self.active_cid, self.active_cmd, self.total_len);
                    self.reset();
                    Ok(Some(res))
                } else {
                    Ok(None)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ctap_hid_packet_init_roundtrip() {
        let mut buf = [0u8; CTAPHID_PACKET_SIZE];
        let payload = b"hello world ctap hid payload test";

        let packet = CtapHidPacket::Init {
            cid: 0x12345678,
            cmd: CtapHidCommand::Cbor.to_u8(),
            payload_len: payload.len() as u16,
            data: payload,
        };

        packet.serialize(&mut buf);
        let parsed = CtapHidPacket::parse(&buf).unwrap();

        match parsed {
            CtapHidPacket::Init {
                cid,
                cmd,
                payload_len,
                data,
            } => {
                assert_eq!(cid, 0x12345678);
                assert_eq!(cmd, CtapHidCommand::Cbor.to_u8());
                assert_eq!(payload_len, payload.len() as u16);
                assert_eq!(&data[..payload.len()], payload);
            }
            _ => panic!("Expected Init packet"),
        }
    }

    #[test]
    fn test_ctap_hid_multi_packet_assembly() {
        // Create 100 bytes payload (spans 1 Init + 1 Cont packet)
        let mut full_payload = [0u8; 100];
        for (i, item) in full_payload.iter_mut().enumerate() {
            *item = i as u8;
        }

        let cid = 0x87654321;
        let mut init_buf = [0u8; CTAPHID_PACKET_SIZE];
        let mut cont_buf = [0u8; CTAPHID_PACKET_SIZE];

        let p_init = CtapHidPacket::Init {
            cid,
            cmd: CtapHidCommand::Cbor.to_u8(),
            payload_len: 100,
            data: &full_payload[..57],
        };
        p_init.serialize(&mut init_buf);

        let p_cont = CtapHidPacket::Cont {
            cid,
            seq: 0,
            data: &full_payload[57..],
        };
        p_cont.serialize(&mut cont_buf);

        let mut assembler = CtapHidMessageAssembler::new();
        let mut assembled_payload = [0u8; 128];

        let pkt1 = CtapHidPacket::parse(&init_buf).unwrap();
        assert!(assembler
            .process_packet(&pkt1, &mut assembled_payload)
            .unwrap()
            .is_none());

        let pkt2 = CtapHidPacket::parse(&cont_buf).unwrap();
        let res = assembler
            .process_packet(&pkt2, &mut assembled_payload)
            .unwrap()
            .unwrap();

        assert_eq!(res.0, cid);
        assert_eq!(res.1, CtapHidCommand::Cbor.to_u8());
        assert_eq!(res.2, 100);
        assert_eq!(&assembled_payload[..100], &full_payload[..]);
    }
}
