//! Device Profile - representa uma unidade física
//!
//! Representa uma unidade física individual. Nunca misture Device Profile com Board Profile.

use core::str;

/// Capacidade máxima de campos textuais persistidos do dispositivo.
pub const DEVICE_TEXT_CAPACITY: usize = 64;

/// Texto UTF-8 de tamanho fixo, adequado para persistência em `no_std`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceText {
    len: u8,
    bytes: [u8; DEVICE_TEXT_CAPACITY],
}

/// Erro de conversão de texto do dispositivo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceTextError {
    TooLong,
    InvalidUtf8,
}

impl DeviceText {
    pub const fn empty() -> Self {
        Self {
            len: 0,
            bytes: [0; DEVICE_TEXT_CAPACITY],
        }
    }

    pub const fn from_static(value: &str) -> Self {
        assert!(
            value.len() <= DEVICE_TEXT_CAPACITY,
            "device text exceeds capacity"
        );
        let source = value.as_bytes();
        let mut bytes = [0; DEVICE_TEXT_CAPACITY];
        let mut index = 0;
        while index < source.len() {
            bytes[index] = source[index];
            index += 1;
        }
        Self {
            len: value.len() as u8,
            bytes,
        }
    }

    pub fn try_from_str(value: &str) -> Result<Self, DeviceTextError> {
        if value.len() > DEVICE_TEXT_CAPACITY {
            return Err(DeviceTextError::TooLong);
        }
        let mut text = Self::empty();
        text.bytes[..value.len()].copy_from_slice(value.as_bytes());
        text.len = value.len() as u8;
        Ok(text)
    }

    pub fn from_bytes(len: u8, bytes: [u8; DEVICE_TEXT_CAPACITY]) -> Result<Self, DeviceTextError> {
        if usize::from(len) > DEVICE_TEXT_CAPACITY {
            return Err(DeviceTextError::TooLong);
        }
        str::from_utf8(&bytes[..usize::from(len)]).map_err(|_| DeviceTextError::InvalidUtf8)?;
        Ok(Self { len, bytes })
    }

    pub fn as_str(&self) -> Result<&str, DeviceTextError> {
        str::from_utf8(&self.bytes[..usize::from(self.len)])
            .map_err(|_| DeviceTextError::InvalidUtf8)
    }

    pub const fn len(&self) -> u8 {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub const fn bytes(&self) -> &[u8; DEVICE_TEXT_CAPACITY] {
        &self.bytes
    }
}

/// Identidade USB de um dispositivo
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UsbIdentity {
    pub vid: u16,
    pub pid: u16,
    pub serial_number: DeviceText,
    pub product_name: DeviceText,
    pub manufacturer_name: DeviceText,
}

/// Dados de calibração do dispositivo
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalibrationData {
    pub rng_offset: u32,
    pub rng_scale: u32,
    pub temp_offset: i16,
    pub temp_scale: u16,
}

/// Dados de fabricação do dispositivo
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManufacturingData {
    pub production_date: u32,
    pub production_location: DeviceText,
    pub batch_number: u32,
    pub test_result: bool,
}

/// Device Profile - representa uma unidade física
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceProfile {
    pub serial_number: DeviceText,
    pub usb_identity: UsbIdentity,
    pub calibration: Option<CalibrationData>,
    pub manufacturing: Option<ManufacturingData>,
}

impl DeviceProfile {
    /// Cria um novo Device Profile
    pub const fn new(
        serial_number: DeviceText,
        usb_identity: UsbIdentity,
        calibration: Option<CalibrationData>,
        manufacturing: Option<ManufacturingData>,
    ) -> Self {
        Self {
            serial_number,
            usb_identity,
            calibration,
            manufacturing,
        }
    }
}
