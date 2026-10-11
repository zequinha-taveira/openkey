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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoardError {
    InvalidSignature,
    Rollback,
    Flash(SimulatorError),
}

/// Placa virtual para validar o contrato de um target RP2350 sem acessar hardware.
/// O nome do perfil descreve a capacidade simulada, não uma dependência de MCU.
pub struct GenericTestBoard {
    pub usb_hid: MemoryTransport,
    pub rng: ControlledRng,
    pub flash: MemoryVault,
    pub clock: DeterministicClock,
    pub presence: UserPresence,
    firmware_version: u32,
    pending_version: Option<u32>,
}

impl GenericTestBoard {
    pub const PROFILE: &'static str = "generic-rp2350-test";

    pub fn new() -> Self {
        Self {
            usb_hid: MemoryTransport::default(),
            rng: ControlledRng::new(&[0x42, 0x24]),
            flash: MemoryVault::default(),
            clock: DeterministicClock::default(),
            presence: UserPresence::default(),
            firmware_version: 0,
            pending_version: None,
        }
    }

    pub const fn firmware_version(&self) -> u32 {
        self.firmware_version
    }

    pub fn stage_update(
        &mut self,
        manifest: openkey_update::UpdateManifest,
        signature_valid: bool,
    ) -> Result<(), BoardError> {
        if !signature_valid {
            return Err(BoardError::InvalidSignature);
        }
        if !manifest.is_newer_than(self.firmware_version) {
            return Err(BoardError::Rollback);
        }
        self.flash
            .write(b"pending-image", &manifest.image_len.to_be_bytes())
            .map_err(BoardError::Flash)?;
        self.pending_version = Some(manifest.version);
        Ok(())
    }

    pub fn boot(&mut self) -> Result<(), BoardError> {
        let mut image_length = [0; 4];
        self.flash
            .read(b"pending-image", &mut image_length)
            .map_err(BoardError::Flash)?;
        if u32::from_be_bytes(image_length) == 0 {
            return Err(BoardError::Rollback);
        }
        self.firmware_version = self.pending_version.take().ok_or(BoardError::Rollback)?;
        self.flash
            .delete(b"pending-image")
            .map_err(BoardError::Flash)
    }
}

impl Default for GenericTestBoard {
    fn default() -> Self {
        Self::new()
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
        BoardError, ControlledRng, DeterministicClock, GenericTestBoard, MemoryTransport,
        MemoryVault, SimulatorError, UserPresence,
    };
    use openkey_crypto::RngProvider;
    use openkey_update::UpdateManifest;
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

    #[test]
    fn placa_generica_emula_perifericos_e_boot_update() {
        let mut board = GenericTestBoard::new();
        assert_eq!(GenericTestBoard::PROFILE, "generic-rp2350-test");
        board.presence.set(true);
        board.usb_hid.send(b"ctap").unwrap();
        assert!(board.presence.is_present());
        board
            .stage_update(
                UpdateManifest {
                    version: 7,
                    image_len: 128,
                },
                true,
            )
            .unwrap();
        board.boot().unwrap();
        assert_eq!(board.firmware_version(), 7);
    }

    #[test]
    fn placa_generica_rejeita_assinatura_invalida_e_rollback() {
        let mut board = GenericTestBoard::new();
        assert_eq!(
            board.stage_update(
                UpdateManifest {
                    version: 1,
                    image_len: 64,
                },
                false,
            ),
            Err(BoardError::InvalidSignature)
        );
        board.firmware_version = 3;
        assert_eq!(
            board.stage_update(
                UpdateManifest {
                    version: 2,
                    image_len: 64,
                },
                true,
            ),
            Err(BoardError::Rollback)
        );
    }
}
