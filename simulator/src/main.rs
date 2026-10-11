use std::cell::Cell;

use openkey_core::Authenticator;
use openkey_crypto::RngProvider;
use openkey_usb::Transport;
use openkey_vault::KeyStore;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulatorError {
    Timeout,
    PowerLoss,
    NotFound,
    OutputTooSmall,
}

#[derive(Default)]
pub struct MemoryTransport {
    incoming: Vec<Vec<u8>>,
    pub outgoing: Vec<Vec<u8>>,
    pub timeout_next: bool,
    pub reorder: bool,
}

impl MemoryTransport {
    pub fn queue(&mut self, packet: &[u8]) {
        self.incoming.push(packet.to_vec());
    }
}

impl Transport for MemoryTransport {
    type Error = SimulatorError;

    fn receive(&mut self, packet: &mut [u8]) -> Result<usize, Self::Error> {
        if self.timeout_next {
            self.timeout_next = false;
            return Err(SimulatorError::Timeout);
        }
        let frame = if self.reorder {
            self.incoming.pop()
        } else if self.incoming.is_empty() {
            None
        } else {
            Some(self.incoming.remove(0))
        }
        .ok_or(SimulatorError::Timeout)?;
        if packet.len() < frame.len() {
            return Err(SimulatorError::OutputTooSmall);
        }
        packet[..frame.len()].copy_from_slice(&frame);
        Ok(frame.len())
    }

    fn send(&mut self, packet: &[u8]) -> Result<(), Self::Error> {
        self.outgoing.push(packet.to_vec());
        Ok(())
    }
}

#[derive(Default)]
pub struct ControlledRng {
    bytes: Vec<u8>,
    pub fail: bool,
}

impl ControlledRng {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            bytes: bytes.to_vec(),
            fail: false,
        }
    }
}

impl RngProvider for ControlledRng {
    type Error = SimulatorError;

    fn fill_bytes(&mut self, output: &mut [u8]) -> Result<(), Self::Error> {
        if self.fail || self.bytes.is_empty() {
            return Err(SimulatorError::PowerLoss);
        }
        for (index, byte) in output.iter_mut().enumerate() {
            *byte = self.bytes[index % self.bytes.len()];
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DeterministicClock {
    now: u64,
}

impl DeterministicClock {
    pub const fn now(self) -> u64 {
        self.now
    }

    pub fn advance(&mut self, ticks: u64) {
        self.now = self.now.saturating_add(ticks);
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct UserPresence {
    present: bool,
}

impl UserPresence {
    pub const fn is_present(self) -> bool {
        self.present
    }

    pub fn set(&mut self, present: bool) {
        self.present = present;
    }
}

#[derive(Default)]
pub struct MemoryVault {
    entries: Vec<(Vec<u8>, Vec<u8>)>,
    pub power_loss_next_write: bool,
    pub corrupt_next_read: Cell<bool>,
}

impl KeyStore for MemoryVault {
    type Error = SimulatorError;

    fn read(&self, key_id: &[u8], output: &mut [u8]) -> Result<usize, Self::Error> {
        let value = self
            .entries
            .iter()
            .find(|(id, _)| id == key_id)
            .map(|(_, value)| value.as_slice())
            .ok_or(SimulatorError::NotFound)?;
        if output.len() < value.len() {
            return Err(SimulatorError::OutputTooSmall);
        }
        output[..value.len()].copy_from_slice(value);
        if self.corrupt_next_read.replace(false) && !value.is_empty() {
            output[value.len() - 1] ^= 0xff;
        }
        Ok(value.len())
    }

    fn write(&mut self, key_id: &[u8], value: &[u8]) -> Result<(), Self::Error> {
        if self.power_loss_next_write {
            self.power_loss_next_write = false;
            return Err(SimulatorError::PowerLoss);
        }
        if let Some((_, stored)) = self.entries.iter_mut().find(|(id, _)| id == key_id) {
            stored.clear();
            stored.extend_from_slice(value);
        } else {
            self.entries.push((key_id.to_vec(), value.to_vec()));
        }
        Ok(())
    }

    fn delete(&mut self, key_id: &[u8]) -> Result<(), Self::Error> {
        let before = self.entries.len();
        self.entries.retain(|(id, _)| id != key_id);
        if before == self.entries.len() {
            Err(SimulatorError::NotFound)
        } else {
            Ok(())
        }
    }
}

fn main() {
    let mut authenticator = Authenticator::new();
    authenticator
        .provision()
        .expect("simulador deve provisionar");
    let mut transport = MemoryTransport::default();
    transport.send(b"get-info").expect("transporte em memória");
    println!(
        "openkey-simulator ready={} frames={}",
        authenticator.state().is_ready(),
        transport.outgoing.len()
    );
}

#[cfg(test)]
mod tests {
    use super::{
        ControlledRng, DeterministicClock, MemoryTransport, MemoryVault, SimulatorError,
        UserPresence,
    };
    use openkey_crypto::RngProvider;
    use openkey_usb::Transport;
    use openkey_vault::KeyStore;

    #[test]
    fn vault_em_memoria_preserva_dados() {
        let mut vault = MemoryVault::default();
        vault.write(b"id", b"secret").unwrap();
        let mut output = [0; 6];
        assert_eq!(vault.read(b"id", &mut output), Ok(6));
        assert_eq!(&output, b"secret");
    }

    #[test]
    fn vault_simula_falha_de_energia_sem_commit() {
        let mut vault = MemoryVault {
            power_loss_next_write: true,
            ..MemoryVault::default()
        };
        assert_eq!(
            vault.write(b"id", b"secret"),
            Err(SimulatorError::PowerLoss)
        );
        assert_eq!(
            vault.read(b"id", &mut [0; 6]),
            Err(SimulatorError::NotFound)
        );
    }

    #[test]
    fn vault_simula_corrupcao_na_leitura() {
        let mut vault = MemoryVault::default();
        vault.write(b"id", b"secret").unwrap();
        vault.corrupt_next_read.set(true);
        let mut output = [0; 6];
        vault.read(b"id", &mut output).unwrap();
        assert_ne!(&output, b"secret");
    }

    #[test]
    fn rng_controlavel_e_reprodutivel() {
        let mut rng = ControlledRng::new(&[1, 2]);
        let mut output = [0; 5];
        rng.fill_bytes(&mut output).unwrap();
        assert_eq!(output, [1, 2, 1, 2, 1]);
    }

    #[test]
    fn relogio_e_presenca_sao_deterministicos() {
        let mut clock = DeterministicClock::default();
        clock.advance(42);
        assert_eq!(clock.now(), 42);
        let mut presence = UserPresence::default();
        assert!(!presence.is_present());
        presence.set(true);
        assert!(presence.is_present());
    }

    #[test]
    fn transporte_simula_timeout_e_reordenacao() {
        let mut transport = MemoryTransport::default();
        transport.queue(b"first");
        transport.queue(b"second");
        transport.reorder = true;
        let mut output = [0; 6];
        assert_eq!(transport.receive(&mut output), Ok(6));
        assert_eq!(&output, b"second");
        transport.timeout_next = true;
        assert_eq!(transport.receive(&mut output), Err(SimulatorError::Timeout));
    }
}
