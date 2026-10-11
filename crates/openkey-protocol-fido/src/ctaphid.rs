extern crate alloc;

use alloc::vec::Vec;

pub const PACKET_SIZE: usize = 64;
pub const BROADCAST_CHANNEL: u32 = 0xffff_ffff;
pub const CMD_INIT: u8 = 0x06;
pub const CMD_CANCEL: u8 = 0x11;
pub const CMD_KEEPALIVE: u8 = 0x3b;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HidError {
    InvalidFrame,
    UnknownChannel,
    Timeout,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HidSession {
    pub channel: u32,
    deadline: u64,
    pub keepalive: bool,
    pub cancelled: bool,
}

impl HidSession {
    pub fn new(channel: u32, now: u64, timeout: u64) -> Self {
        Self {
            channel,
            deadline: now.saturating_add(timeout),
            keepalive: false,
            cancelled: false,
        }
    }

    pub fn tick(&mut self, now: u64) -> Result<(), HidError> {
        if self.cancelled {
            return Err(HidError::Cancelled);
        }
        if now >= self.deadline {
            return Err(HidError::Timeout);
        }
        Ok(())
    }

    pub fn keepalive(&mut self) -> Result<(), HidError> {
        if self.cancelled {
            return Err(HidError::Cancelled);
        }
        self.keepalive = true;
        Ok(())
    }

    pub fn cancel(&mut self) {
        self.cancelled = true;
    }
}

#[derive(Debug, Default)]
pub struct ChannelManager {
    next_channel: u32,
    channels: Vec<u32>,
}

impl ChannelManager {
    pub fn new() -> Self {
        Self {
            next_channel: 1,
            channels: Vec::new(),
        }
    }

    pub fn init(&mut self) -> u32 {
        let channel = self.next_channel.max(1);
        self.next_channel = channel.wrapping_add(1);
        self.channels.push(channel);
        channel
    }

    pub fn contains(&self, channel: u32) -> bool {
        self.channels.contains(&channel)
    }

    pub fn frame(
        &self,
        channel: u32,
        command: u8,
        payload: &[u8],
    ) -> Result<Vec<[u8; PACKET_SIZE]>, HidError> {
        if channel != BROADCAST_CHANNEL && !self.contains(channel) {
            return Err(HidError::UnknownChannel);
        }
        if payload.len() > 7609 {
            return Err(HidError::InvalidFrame);
        }
        let mut frames = Vec::new();
        let mut first = [0; PACKET_SIZE];
        first[..4].copy_from_slice(&channel.to_be_bytes());
        first[4] = command | 0x80;
        first[5..7].copy_from_slice(&(payload.len() as u16).to_be_bytes());
        let first_len = payload.len().min(PACKET_SIZE - 7);
        first[7..7 + first_len].copy_from_slice(&payload[..first_len]);
        frames.push(first);
        let mut offset = first_len;
        let mut sequence = 0u8;
        while offset < payload.len() {
            let mut continuation = [0; PACKET_SIZE];
            continuation[..4].copy_from_slice(&channel.to_be_bytes());
            continuation[4] = sequence & 0x7f;
            let length = (payload.len() - offset).min(PACKET_SIZE - 5);
            continuation[5..5 + length].copy_from_slice(&payload[offset..offset + length]);
            frames.push(continuation);
            offset += length;
            sequence = sequence.wrapping_add(1);
        }
        Ok(frames)
    }

    pub fn cancel(&self, session: &mut HidSession) -> Result<(), HidError> {
        if !self.contains(session.channel) {
            return Err(HidError::UnknownChannel);
        }
        session.cancel();
        Ok(())
    }
}

pub fn decode_frame(frame: &[u8; PACKET_SIZE]) -> Result<(u32, u8, &[u8]), HidError> {
    let channel = u32::from_be_bytes([frame[0], frame[1], frame[2], frame[3]]);
    if frame[4] & 0x80 == 0 {
        return Err(HidError::InvalidFrame);
    }
    let command = frame[4] & 0x7f;
    let length = u16::from_be_bytes([frame[5], frame[6]]) as usize;
    let available = length.min(PACKET_SIZE - 7);
    Ok((channel, command, &frame[7..7 + available]))
}

#[cfg(test)]
mod tests {
    use super::{
        decode_frame, ChannelManager, HidError, HidSession, CMD_CANCEL, CMD_INIT, PACKET_SIZE,
    };

    #[test]
    fn init_e_framing_preservam_canal_e_payload() {
        let mut manager = ChannelManager::new();
        let channel = manager.init();
        let payload = [0x55; 80];
        let frames = manager.frame(channel, CMD_INIT, &payload).unwrap();
        assert_eq!(frames.len(), 2);
        let (decoded_channel, command, first) = decode_frame(&frames[0]).unwrap();
        assert_eq!((decoded_channel, command), (channel, CMD_INIT));
        assert_eq!(first, &payload[..PACKET_SIZE - 7]);
    }

    #[test]
    fn cancel_keepalive_e_timeout_sao_deterministicos() {
        let mut manager = ChannelManager::new();
        let channel = manager.init();
        let mut session = HidSession::new(channel, 10, 5);
        session.keepalive().unwrap();
        assert!(session.keepalive);
        manager.cancel(&mut session).unwrap();
        assert_eq!(session.tick(11), Err(HidError::Cancelled));
        assert_eq!(CMD_CANCEL, 0x11);
    }
}
