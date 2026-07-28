//! Configuration Manager - configuração persistente, versionada e validada.

use crate::app_config::{
    AppConfig, CcidConfig, Ctap2Config, LoggingConfig, OpenPgpConfig, PivConfig, SecurityPolicies,
};
use crate::board::{BoardProfile, BoardProfileCatalog, BoardProfileId};
use crate::device::{
    CalibrationData, DeviceProfile, DeviceText, DeviceTextError, ManufacturingData, UsbIdentity,
    DEVICE_TEXT_CAPACITY,
};
use crate::hal::{FlashError, FlashStorageProvider, HalError, RngProvider};
use openkey_crypto::{
    decrypt_config, encrypt_config, CONFIG_AEAD_KEY_SIZE, CONFIG_AEAD_NONCE_SIZE,
    CONFIG_AEAD_TAG_SIZE,
};
use zeroize::Zeroizing;

const CONFIG_MAGIC: [u8; 4] = *b"OKCF";
const CONFIG_VERSION: u8 = 2;
const HEADER_SIZE: usize = 40;
const AAD_SIZE: usize = 23;
const MAX_PAYLOAD_SIZE: usize = 512;
const MIN_SLOT_SIZE: u32 = (HEADER_SIZE + MAX_PAYLOAD_SIZE) as u32;
const VALID_STATE: u8 = 0xA5;
const WRITING_STATE: u8 = 0;

/// Origem da chave AES-256 exclusiva do dispositivo.
pub trait ConfigKeyProvider {
    fn fill_key(&self, destination: &mut [u8; CONFIG_AEAD_KEY_SIZE]) -> Result<(), ConfigKeyError>;
}

/// Recursos criptográficos necessários durante uma gravação de configuração.
pub struct ConfigCryptoContext<'a> {
    key_provider: &'a dyn ConfigKeyProvider,
    rng: &'a mut dyn RngProvider,
}

impl<'a> ConfigCryptoContext<'a> {
    pub fn new(key_provider: &'a dyn ConfigKeyProvider, rng: &'a mut dyn RngProvider) -> Self {
        Self { key_provider, rng }
    }
}

/// Erro ao obter a chave de autenticação da configuração.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigKeyError {
    Unavailable,
    HardwareFailure,
}

/// Duas regiões de Flash reservadas exclusivamente para configuração.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigStorageLayout {
    pub primary_offset: u32,
    pub secondary_offset: u32,
    pub slot_size: u32,
}

/// Erros de validação ou acesso da configuração persistente.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigurationError {
    Flash(FlashError),
    Key(ConfigKeyError),
    Rng(HalError),
    InvalidLayout,
    BufferTooSmall,
    InvalidRecord,
    UnknownBoardProfile,
    InvalidText,
}

impl From<ConfigKeyError> for ConfigurationError {
    fn from(error: ConfigKeyError) -> Self {
        Self::Key(error)
    }
}

impl From<FlashError> for ConfigurationError {
    fn from(error: FlashError) -> Self {
        Self::Flash(error)
    }
}

/// Estado de provisionamento do dispositivo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProvisioningState {
    Unprovisioned,
    Partial,
    Provisioned,
}

/// Gerencia Board Profile, Device Profile e Application Configuration.
pub struct ConfigurationManager {
    board: Option<BoardProfile>,
    device: Option<DeviceProfile>,
    app: Option<AppConfig>,
    state: ProvisioningState,
    generation: u32,
}

impl ConfigurationManager {
    pub const fn new() -> Self {
        Self {
            board: None,
            device: None,
            app: None,
            state: ProvisioningState::Unprovisioned,
            generation: 0,
        }
    }

    /// Carrega o registro válido mais recente dos dois slots.
    pub fn load(
        &mut self,
        flash: &mut dyn FlashStorageProvider,
        key_provider: &dyn ConfigKeyProvider,
        catalog: &dyn BoardProfileCatalog,
        layout: ConfigStorageLayout,
    ) -> Result<(), ConfigurationError> {
        validate_layout(flash, layout)?;
        self.clear();
        let mut key = Zeroizing::new([0u8; CONFIG_AEAD_KEY_SIZE]);
        key_provider.fill_key(&mut key)?;

        let primary = read_slot(flash, layout.primary_offset, &key, catalog)?;
        let secondary = read_slot(flash, layout.secondary_offset, &key, catalog)?;
        let selected = match (primary, secondary) {
            (Some(left), Some(right)) => {
                if left.generation >= right.generation {
                    left
                } else {
                    right
                }
            }
            (Some(record), None) | (None, Some(record)) => record,
            (None, None) => return Err(ConfigurationError::InvalidRecord),
        };

        self.board = Some(selected.board.clone());
        self.device = Some(selected.device);
        self.app = Some(selected.app);
        self.generation = selected.generation;
        self.state = ProvisioningState::Provisioned;
        Ok(())
    }

    /// Persiste uma nova configuração no slot inativo e preserva a anterior.
    pub fn save(
        &mut self,
        flash: &mut dyn FlashStorageProvider,
        crypto: &mut ConfigCryptoContext<'_>,
        layout: ConfigStorageLayout,
        board: &BoardProfile,
        device: &DeviceProfile,
        app: &AppConfig,
    ) -> Result<(), ConfigurationError> {
        validate_layout(flash, layout)?;
        if !crypto.rng.is_healthy() {
            return Err(ConfigurationError::Rng(HalError::RngNotHealthy));
        }
        let mut key = Zeroizing::new([0u8; CONFIG_AEAD_KEY_SIZE]);
        crypto.key_provider.fill_key(&mut key)?;
        let mut nonce = [0u8; CONFIG_AEAD_NONCE_SIZE];
        crypto
            .rng
            .fill_bytes(&mut nonce)
            .map_err(ConfigurationError::Rng)?;
        let generation = self
            .generation
            .checked_add(1)
            .ok_or(ConfigurationError::InvalidRecord)?;
        // Select the target slot by comparing actual flash contents, not by
        // relying on the cached generation. The slot with the lower generation
        // (or the primary slot when both are empty) is the inactive slot.
        let target = self.select_inactive_slot(flash, &key, layout)?;
        let mut payload = [0u8; MAX_PAYLOAD_SIZE];
        let payload_len = encode_payload(&mut payload, board.id, device, app)?;
        let mut header = [0u8; HEADER_SIZE];
        header[..4].copy_from_slice(&CONFIG_MAGIC);
        header[4] = CONFIG_VERSION;
        header[5] = WRITING_STATE;
        header[6..10].copy_from_slice(&generation.to_le_bytes());
        header[10..12].copy_from_slice(&(payload_len as u16).to_le_bytes());
        header[12..24].copy_from_slice(&nonce);
        let aad = associated_data(&header);
        let tag = encrypt_config(&key, &nonce, &aad, &mut payload[..payload_len])
            .map_err(|_| ConfigurationError::InvalidRecord)?;
        header[24..40].copy_from_slice(&tag);
        // Write the VALID_STATE byte directly in the header buffer so the
        // entire header (including the state byte) is written in a single
        // flash.write() call. This avoids writing to the same offset twice,
        // which is not supported by all flash hardware.
        header[5] = VALID_STATE;

        flash.erase(target, layout.slot_size)?;
        let header_end = target
            .checked_add(HEADER_SIZE as u32)
            .ok_or(ConfigurationError::InvalidLayout)?;
        flash.write(target, &header)?;
        flash.write(header_end, &payload[..payload_len])?;

        self.board = Some(board.clone());
        self.device = Some(device.clone());
        self.app = Some(app.clone());
        self.generation = generation;
        self.state = ProvisioningState::Provisioned;
        Ok(())
    }

    pub fn board_profile(&self) -> Option<&BoardProfile> {
        self.board.as_ref()
    }
    pub fn device_profile(&self) -> Option<&DeviceProfile> {
        self.device.as_ref()
    }
    pub fn app_config(&self) -> Option<&AppConfig> {
        self.app.as_ref()
    }
    pub const fn provisioning_state(&self) -> ProvisioningState {
        self.state
    }
    pub fn is_provisioned(&self) -> bool {
        self.state == ProvisioningState::Provisioned
    }

    fn clear(&mut self) {
        self.board = None;
        self.device = None;
        self.app = None;
        self.generation = 0;
        self.state = ProvisioningState::Unprovisioned;
    }

    /// Reads the generation from a slot header without decrypting the payload.
    /// Returns `None` if the slot is empty or invalid.
    fn read_slot_generation(
        flash: &mut dyn FlashStorageProvider,
        offset: u32,
    ) -> Result<Option<u32>, ConfigurationError> {
        let mut header = [0u8; HEADER_SIZE];
        flash.read(offset, &mut header)?;
        if header[..4] != CONFIG_MAGIC || header[4] != CONFIG_VERSION || header[5] != VALID_STATE {
            return Ok(None);
        }
        let generation = u32::from_le_bytes([header[6], header[7], header[8], header[9]]);
        Ok(Some(generation))
    }

    /// Selects the inactive slot by comparing the actual generation values
    /// stored in flash. The slot with the lower generation is considered
    /// inactive. If both slots are empty, the primary slot is selected.
    fn select_inactive_slot(
        &self,
        flash: &mut dyn FlashStorageProvider,
        _key: &[u8; CONFIG_AEAD_KEY_SIZE],
        layout: ConfigStorageLayout,
    ) -> Result<u32, ConfigurationError> {
        let primary_gen = Self::read_slot_generation(flash, layout.primary_offset)?;
        let secondary_gen = Self::read_slot_generation(flash, layout.secondary_offset)?;
        match (primary_gen, secondary_gen) {
            (None, None) => Ok(layout.primary_offset),
            (Some(_), None) => Ok(layout.secondary_offset),
            (None, Some(_)) => Ok(layout.primary_offset),
            (Some(p), Some(s)) => {
                if p <= s {
                    Ok(layout.secondary_offset)
                } else {
                    Ok(layout.primary_offset)
                }
            }
        }
    }
}

impl Default for ConfigurationManager {
    fn default() -> Self {
        Self::new()
    }
}

struct LoadedRecord {
    generation: u32,
    board: BoardProfile,
    device: DeviceProfile,
    app: AppConfig,
}

fn validate_layout(
    flash: &dyn FlashStorageProvider,
    layout: ConfigStorageLayout,
) -> Result<(), ConfigurationError> {
    if layout.slot_size < MIN_SLOT_SIZE || layout.primary_offset == layout.secondary_offset {
        return Err(ConfigurationError::InvalidLayout);
    }
    let primary_end = layout
        .primary_offset
        .checked_add(layout.slot_size)
        .ok_or(ConfigurationError::InvalidLayout)?;
    let secondary_end = layout
        .secondary_offset
        .checked_add(layout.slot_size)
        .ok_or(ConfigurationError::InvalidLayout)?;
    if primary_end > flash.total_size() || secondary_end > flash.total_size() {
        return Err(ConfigurationError::InvalidLayout);
    }
    let overlaps = layout.primary_offset < secondary_end && layout.secondary_offset < primary_end;
    if overlaps {
        return Err(ConfigurationError::InvalidLayout);
    }
    Ok(())
}

fn read_slot(
    flash: &mut dyn FlashStorageProvider,
    offset: u32,
    key: &[u8; CONFIG_AEAD_KEY_SIZE],
    catalog: &dyn BoardProfileCatalog,
) -> Result<Option<LoadedRecord>, ConfigurationError> {
    let mut header = [0u8; HEADER_SIZE];
    flash.read(offset, &mut header)?;
    if header[..4] != CONFIG_MAGIC || header[4] != CONFIG_VERSION || header[5] != VALID_STATE {
        return Ok(None);
    }
    let payload_len = usize::from(u16::from_le_bytes([header[10], header[11]]));
    if payload_len > MAX_PAYLOAD_SIZE {
        return Ok(None);
    }
    let mut payload = Zeroizing::new([0u8; MAX_PAYLOAD_SIZE]);
    flash.read(offset + HEADER_SIZE as u32, &mut payload[..payload_len])?;
    let mut nonce = [0u8; CONFIG_AEAD_NONCE_SIZE];
    nonce.copy_from_slice(&header[12..24]);
    let mut tag = [0u8; CONFIG_AEAD_TAG_SIZE];
    tag.copy_from_slice(&header[24..40]);
    let aad = associated_data(&header);
    if decrypt_config(key, &nonce, &aad, &mut payload[..payload_len], &tag).is_err() {
        return Ok(None);
    }
    let generation = u32::from_le_bytes([header[6], header[7], header[8], header[9]]);
    let (id, device, app) = decode_payload(&payload[..payload_len])?;
    let board = catalog
        .find(id)
        .ok_or(ConfigurationError::UnknownBoardProfile)?;
    Ok(Some(LoadedRecord {
        generation,
        board: board.clone(),
        device,
        app,
    }))
}

fn associated_data(header: &[u8; HEADER_SIZE]) -> [u8; AAD_SIZE] {
    let mut aad = [0u8; AAD_SIZE];
    aad[..5].copy_from_slice(&header[..5]);
    aad[5..].copy_from_slice(&header[6..24]);
    aad
}

struct Writer<'a> {
    bytes: &'a mut [u8],
    pos: usize,
}
impl<'a> Writer<'a> {
    fn new(bytes: &'a mut [u8]) -> Self {
        Self { bytes, pos: 0 }
    }
    fn put(&mut self, value: &[u8]) -> Result<(), ConfigurationError> {
        let end = self
            .pos
            .checked_add(value.len())
            .ok_or(ConfigurationError::BufferTooSmall)?;
        if end > self.bytes.len() {
            return Err(ConfigurationError::BufferTooSmall);
        }
        self.bytes[self.pos..end].copy_from_slice(value);
        self.pos = end;
        Ok(())
    }
    fn u8(&mut self, value: u8) -> Result<(), ConfigurationError> {
        self.put(&[value])
    }
    fn bool(&mut self, value: bool) -> Result<(), ConfigurationError> {
        self.u8(u8::from(value))
    }
    fn u16(&mut self, value: u16) -> Result<(), ConfigurationError> {
        self.put(&value.to_le_bytes())
    }
    fn u32(&mut self, value: u32) -> Result<(), ConfigurationError> {
        self.put(&value.to_le_bytes())
    }
    fn i16(&mut self, value: i16) -> Result<(), ConfigurationError> {
        self.put(&value.to_le_bytes())
    }
    fn text(&mut self, value: DeviceText) -> Result<(), ConfigurationError> {
        self.u8(value.len())?;
        self.put(value.bytes())
    }
}

struct Reader<'a> {
    bytes: &'a [u8],
    pos: usize,
}
impl<'a> Reader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }
    fn take(&mut self, len: usize) -> Result<&'a [u8], ConfigurationError> {
        let end = self
            .pos
            .checked_add(len)
            .ok_or(ConfigurationError::InvalidRecord)?;
        if end > self.bytes.len() {
            return Err(ConfigurationError::InvalidRecord);
        }
        let result = &self.bytes[self.pos..end];
        self.pos = end;
        Ok(result)
    }
    fn u8(&mut self) -> Result<u8, ConfigurationError> {
        Ok(self.take(1)?[0])
    }
    fn bool(&mut self) -> Result<bool, ConfigurationError> {
        match self.u8()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(ConfigurationError::InvalidRecord),
        }
    }
    fn u16(&mut self) -> Result<u16, ConfigurationError> {
        let b = self.take(2)?;
        Ok(u16::from_le_bytes([b[0], b[1]]))
    }
    fn u32(&mut self) -> Result<u32, ConfigurationError> {
        let b = self.take(4)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }
    fn i16(&mut self) -> Result<i16, ConfigurationError> {
        let b = self.take(2)?;
        Ok(i16::from_le_bytes([b[0], b[1]]))
    }
    fn text(&mut self) -> Result<DeviceText, ConfigurationError> {
        let len = self.u8()?;
        let mut bytes = [0u8; DEVICE_TEXT_CAPACITY];
        bytes.copy_from_slice(self.take(DEVICE_TEXT_CAPACITY)?);
        DeviceText::from_bytes(len, bytes).map_err(map_text_error)
    }
    fn finished(&self) -> bool {
        self.pos == self.bytes.len()
    }
}

fn map_text_error(_: DeviceTextError) -> ConfigurationError {
    ConfigurationError::InvalidText
}

fn encode_payload(
    out: &mut [u8],
    id: BoardProfileId,
    device: &DeviceProfile,
    app: &AppConfig,
) -> Result<usize, ConfigurationError> {
    let mut w = Writer::new(out);
    w.put(&id.0)?;
    w.text(device.serial_number)?;
    w.u16(device.usb_identity.vid)?;
    w.u16(device.usb_identity.pid)?;
    w.text(device.usb_identity.serial_number)?;
    w.text(device.usb_identity.product_name)?;
    w.text(device.usb_identity.manufacturer_name)?;
    match device.calibration {
        Some(value) => {
            w.bool(true)?;
            w.u32(value.rng_offset)?;
            w.u32(value.rng_scale)?;
            w.i16(value.temp_offset)?;
            w.u16(value.temp_scale)?;
        }
        None => w.bool(false)?,
    }
    match &device.manufacturing {
        Some(value) => {
            w.bool(true)?;
            w.u32(value.production_date)?;
            w.text(value.production_location)?;
            w.u32(value.batch_number)?;
            w.bool(value.test_result)?;
        }
        None => w.bool(false)?,
    }
    encode_app(&mut w, app)?;
    Ok(w.pos)
}

fn encode_app(w: &mut Writer<'_>, app: &AppConfig) -> Result<(), ConfigurationError> {
    let c = app.ctap2;
    for value in [
        c.enable_fido2_0,
        c.enable_fido2_1,
        c.enable_resident_keys,
        c.enable_user_verification,
        c.enable_credential_management,
        c.enable_authenticator_reset,
        c.enable_hmac_secret,
        c.enable_large_blob,
    ] {
        w.bool(value)?;
    }
    w.u16(c.max_credential_count)?;
    w.u16(c.max_blob_size)?;
    let c = app.ccid;
    w.bool(c.enable_ccid)?;
    w.u32(c.max_message_length)?;
    w.u8(c.max_busy_slots)?;
    let c = app.openpgp;
    w.bool(c.enable_openpgp)?;
    w.u16(c.max_key_size)?;
    w.bool(c.enable_rsa)?;
    w.bool(c.enable_ecdsa)?;
    let c = app.piv;
    w.bool(c.enable_piv)?;
    w.u16(c.max_key_size)?;
    w.bool(c.enable_rsa)?;
    w.bool(c.enable_ecdsa)?;
    let c = app.logging;
    w.bool(c.enable_logging)?;
    w.u8(c.log_level)?;
    w.bool(c.log_to_usb)?;
    w.bool(c.log_to_flash)?;
    w.u16(c.max_log_entries)?;
    let p = app.policies;
    w.bool(p.require_user_presence)?;
    w.bool(p.require_user_verification)?;
    w.u8(p.pin_min_length)?;
    w.u8(p.pin_max_retries)?;
    w.u8(p.pin_lockout_threshold)?;
    w.u32(p.pin_lockout_duration_ms)?;
    w.bool(p.firmware_update_requires_signature)?;
    w.bool(p.disable_factory_reset)
}

fn decode_payload(
    input: &[u8],
) -> Result<(BoardProfileId, DeviceProfile, AppConfig), ConfigurationError> {
    let mut r = Reader::new(input);
    let mut id = [0u8; 16];
    id.copy_from_slice(r.take(16)?);
    let serial_number = r.text()?;
    let vid = r.u16()?;
    let pid = r.u16()?;
    let usb_identity = UsbIdentity {
        vid,
        pid,
        serial_number: r.text()?,
        product_name: r.text()?,
        manufacturer_name: r.text()?,
    };
    let calibration = if r.bool()? {
        Some(CalibrationData {
            rng_offset: r.u32()?,
            rng_scale: r.u32()?,
            temp_offset: r.i16()?,
            temp_scale: r.u16()?,
        })
    } else {
        None
    };
    let manufacturing = if r.bool()? {
        Some(ManufacturingData {
            production_date: r.u32()?,
            production_location: r.text()?,
            batch_number: r.u32()?,
            test_result: r.bool()?,
        })
    } else {
        None
    };
    let app = decode_app(&mut r)?;
    if !r.finished() {
        return Err(ConfigurationError::InvalidRecord);
    }
    Ok((
        BoardProfileId(id),
        DeviceProfile::new(serial_number, usb_identity, calibration, manufacturing),
        app,
    ))
}

fn decode_app(r: &mut Reader<'_>) -> Result<AppConfig, ConfigurationError> {
    let ctap2 = Ctap2Config {
        enable_fido2_0: r.bool()?,
        enable_fido2_1: r.bool()?,
        enable_resident_keys: r.bool()?,
        enable_user_verification: r.bool()?,
        enable_credential_management: r.bool()?,
        enable_authenticator_reset: r.bool()?,
        enable_hmac_secret: r.bool()?,
        enable_large_blob: r.bool()?,
        max_credential_count: r.u16()?,
        max_blob_size: r.u16()?,
    };
    let ccid = CcidConfig {
        enable_ccid: r.bool()?,
        max_message_length: r.u32()?,
        max_busy_slots: r.u8()?,
    };
    let openpgp = OpenPgpConfig {
        enable_openpgp: r.bool()?,
        max_key_size: r.u16()?,
        enable_rsa: r.bool()?,
        enable_ecdsa: r.bool()?,
    };
    let piv = PivConfig {
        enable_piv: r.bool()?,
        max_key_size: r.u16()?,
        enable_rsa: r.bool()?,
        enable_ecdsa: r.bool()?,
    };
    let logging = LoggingConfig {
        enable_logging: r.bool()?,
        log_level: r.u8()?,
        log_to_usb: r.bool()?,
        log_to_flash: r.bool()?,
        max_log_entries: r.u16()?,
    };
    let policies = SecurityPolicies {
        require_user_presence: r.bool()?,
        require_user_verification: r.bool()?,
        pin_min_length: r.u8()?,
        pin_max_retries: r.u8()?,
        pin_lockout_threshold: r.u8()?,
        pin_lockout_duration_ms: r.u32()?,
        firmware_update_requires_signature: r.bool()?,
        disable_factory_reset: r.bool()?,
    };
    Ok(AppConfig {
        ctap2,
        ccid,
        openpgp,
        piv,
        logging,
        policies,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::{FlashConfig, OptionalFeatures, UsbConfig};

    const BOARD: BoardProfile = BoardProfile {
        id: BoardProfileId(*b"openkey-rp23xx01"),
        manufacturer: "OpenKey",
        model: "test",
        revision: "1",
        flash: FlashConfig {
            total_size: 8192,
            page_size: 4096,
            sector_size: 4096,
        },
        usb: UsbConfig {
            vid: 1,
            pid: 2,
            bcd_version: 1,
            max_packet_size: 64,
        },
        led: None,
        button: None,
        features: OptionalFeatures {
            has_nfc: false,
            has_ble: false,
            has_secure_element: false,
            has_tamper_detect: false,
        },
    };
    struct Catalog;
    impl BoardProfileCatalog for Catalog {
        fn find(&self, id: BoardProfileId) -> Option<&'static BoardProfile> {
            if id == BOARD.id {
                Some(&BOARD)
            } else {
                None
            }
        }
    }

    struct TestKey;
    impl ConfigKeyProvider for TestKey {
        fn fill_key(
            &self,
            destination: &mut [u8; CONFIG_AEAD_KEY_SIZE],
        ) -> Result<(), ConfigKeyError> {
            destination.fill(0x42);
            Ok(())
        }
    }

    struct WrongKey;
    impl ConfigKeyProvider for WrongKey {
        fn fill_key(
            &self,
            destination: &mut [u8; CONFIG_AEAD_KEY_SIZE],
        ) -> Result<(), ConfigKeyError> {
            destination.fill(0x43);
            Ok(())
        }
    }

    struct TestRng {
        next: u8,
        healthy: bool,
    }
    impl TestRng {
        fn healthy() -> Self {
            Self {
                next: 0,
                healthy: true,
            }
        }
    }
    impl RngProvider for TestRng {
        fn fill_bytes(&mut self, destination: &mut [u8]) -> Result<(), HalError> {
            destination.fill(self.next);
            self.next = self.next.wrapping_add(1);
            Ok(())
        }
        fn is_healthy(&self) -> bool {
            self.healthy
        }
    }
    #[derive(Clone, Copy)]
    enum Failure {
        None,
        Read,
        Write,
        Erase,
    }

    struct Flash {
        bytes: [u8; 8192],
        failure: Failure,
    }
    impl Flash {
        fn new() -> Self {
            Self {
                bytes: [0xff; 8192],
                failure: Failure::None,
            }
        }
    }
    impl FlashStorageProvider for Flash {
        fn read(&mut self, offset: u32, out: &mut [u8]) -> Result<(), FlashError> {
            if matches!(self.failure, Failure::Read) {
                return Err(FlashError::HardwareFailure);
            }
            let end = offset as usize + out.len();
            if end > self.bytes.len() {
                return Err(FlashError::OutOfBounds);
            }
            out.copy_from_slice(&self.bytes[offset as usize..end]);
            Ok(())
        }
        fn write(&mut self, offset: u32, data: &[u8]) -> Result<(), FlashError> {
            if matches!(self.failure, Failure::Write) {
                return Err(FlashError::WriteError);
            }
            let end = offset as usize + data.len();
            if end > self.bytes.len() {
                return Err(FlashError::OutOfBounds);
            }
            self.bytes[offset as usize..end].copy_from_slice(data);
            Ok(())
        }
        fn erase(&mut self, offset: u32, len: u32) -> Result<(), FlashError> {
            if matches!(self.failure, Failure::Erase) {
                return Err(FlashError::EraseError);
            }
            let end = offset as usize + len as usize;
            if end > self.bytes.len() {
                return Err(FlashError::OutOfBounds);
            }
            self.bytes[offset as usize..end].fill(0xff);
            Ok(())
        }
        fn total_size(&self) -> u32 {
            self.bytes.len() as u32
        }
    }
    fn layout() -> ConfigStorageLayout {
        ConfigStorageLayout {
            primary_offset: 0,
            secondary_offset: 4096,
            slot_size: 4096,
        }
    }
    fn text(value: &str) -> DeviceText {
        DeviceText::try_from_str(value).unwrap()
    }
    fn device() -> DeviceProfile {
        DeviceProfile::new(
            text("device-1"),
            UsbIdentity {
                vid: 1,
                pid: 2,
                serial_number: text("usb-1"),
                product_name: text("OpenKey"),
                manufacturer_name: text("OpenKey"),
            },
            Some(CalibrationData {
                rng_offset: 1,
                rng_scale: 2,
                temp_offset: -3,
                temp_scale: 4,
            }),
            Some(ManufacturingData {
                production_date: 5,
                production_location: text("BR"),
                batch_number: 6,
                test_result: true,
            }),
        )
    }
    #[test]
    fn round_trip_restores_all_configuration() {
        let mut flash = Flash::new();
        let mut writer = ConfigurationManager::new();
        let mut rng = TestRng::healthy();
        let expected = device();
        let app = AppConfig::default();
        writer
            .save(
                &mut flash,
                &mut ConfigCryptoContext::new(&TestKey, &mut rng),
                layout(),
                &BOARD,
                &expected,
                &app,
            )
            .unwrap();
        let mut reader = ConfigurationManager::new();
        reader
            .load(&mut flash, &TestKey, &Catalog, layout())
            .unwrap();
        assert_eq!(reader.device_profile(), Some(&expected));
        assert_eq!(reader.app_config(), Some(&app));
        assert_eq!(
            reader.board_profile().map(|profile| profile.id),
            Some(BOARD.id)
        );
    }
    #[test]
    fn corrupted_new_slot_falls_back_to_previous_record() {
        let mut flash = Flash::new();
        let mut config = ConfigurationManager::new();
        let mut rng = TestRng::healthy();
        config
            .save(
                &mut flash,
                &mut ConfigCryptoContext::new(&TestKey, &mut rng),
                layout(),
                &BOARD,
                &device(),
                &AppConfig::default(),
            )
            .unwrap();
        let mut updated = AppConfig::default();
        updated.logging.enable_logging = true;
        config
            .save(
                &mut flash,
                &mut ConfigCryptoContext::new(&TestKey, &mut rng),
                layout(),
                &BOARD,
                &device(),
                &updated,
            )
            .unwrap();
        flash.bytes[4096 + HEADER_SIZE] ^= 1;
        let mut reader = ConfigurationManager::new();
        reader
            .load(&mut flash, &TestKey, &Catalog, layout())
            .unwrap();
        assert!(!reader.app_config().unwrap().logging.enable_logging);
    }
    #[test]
    fn invalid_layout_is_rejected() {
        let mut flash = Flash::new();
        let mut rng = TestRng::healthy();
        let error = ConfigurationManager::new()
            .save(
                &mut flash,
                &mut ConfigCryptoContext::new(&TestKey, &mut rng),
                ConfigStorageLayout {
                    primary_offset: 0,
                    secondary_offset: 0,
                    slot_size: 4096,
                },
                &BOARD,
                &device(),
                &AppConfig::default(),
            )
            .unwrap_err();
        assert_eq!(error, ConfigurationError::InvalidLayout);
    }

    #[test]
    fn flash_failures_do_not_provision_the_device() {
        for failure in [Failure::Write, Failure::Erase] {
            let mut flash = Flash::new();
            flash.failure = failure;
            let mut config = ConfigurationManager::new();
            let mut rng = TestRng::healthy();
            assert!(config
                .save(
                    &mut flash,
                    &mut ConfigCryptoContext::new(&TestKey, &mut rng),
                    layout(),
                    &BOARD,
                    &device(),
                    &AppConfig::default()
                )
                .is_err());
            assert!(!config.is_provisioned());
        }
        let mut flash = Flash::new();
        flash.failure = Failure::Read;
        let mut config = ConfigurationManager::new();
        assert!(config
            .load(&mut flash, &TestKey, &Catalog, layout())
            .is_err());
        assert!(!config.is_provisioned());
    }

    #[test]
    fn authentication_failures_do_not_provision_the_device() {
        for offset in [HEADER_SIZE, 6, 24] {
            let mut flash = Flash::new();
            let mut writer = ConfigurationManager::new();
            let mut rng = TestRng::healthy();
            writer
                .save(
                    &mut flash,
                    &mut ConfigCryptoContext::new(&TestKey, &mut rng),
                    layout(),
                    &BOARD,
                    &device(),
                    &AppConfig::default(),
                )
                .unwrap();
            flash.bytes[offset] ^= 1;
            let mut reader = ConfigurationManager::new();
            assert!(reader
                .load(&mut flash, &TestKey, &Catalog, layout())
                .is_err());
            assert!(!reader.is_provisioned());
        }

        let mut flash = Flash::new();
        let mut writer = ConfigurationManager::new();
        let mut rng = TestRng::healthy();
        writer
            .save(
                &mut flash,
                &mut ConfigCryptoContext::new(&TestKey, &mut rng),
                layout(),
                &BOARD,
                &device(),
                &AppConfig::default(),
            )
            .unwrap();
        let mut reader = ConfigurationManager::new();
        assert!(reader
            .load(&mut flash, &WrongKey, &Catalog, layout())
            .is_err());
        assert!(!reader.is_provisioned());
    }

    #[test]
    fn unhealthy_rng_prevents_persistence() {
        let mut flash = Flash::new();
        let mut rng = TestRng {
            next: 0,
            healthy: false,
        };
        let mut config = ConfigurationManager::new();
        assert_eq!(
            config.save(
                &mut flash,
                &mut ConfigCryptoContext::new(&TestKey, &mut rng),
                layout(),
                &BOARD,
                &device(),
                &AppConfig::default(),
            ),
            Err(ConfigurationError::Rng(HalError::RngNotHealthy))
        );
        assert!(!config.is_provisioned());
    }
}
