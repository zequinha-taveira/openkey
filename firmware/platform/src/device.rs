//! Device Profile - representa uma unidade física
//!
//! Representa uma unidade física individual. Nunca misture Device Profile com Board Profile.

/// Identidade USB de um dispositivo
#[derive(Debug, Clone, Copy)]
pub struct UsbIdentity {
    pub vid: u16,
    pub pid: u16,
    pub serial_number: &'static str,
    pub product_name: &'static str,
    pub manufacturer_name: &'static str,
}

/// Dados de calibração do dispositivo
#[derive(Debug, Clone, Copy)]
pub struct CalibrationData {
    pub rng_offset: u32,
    pub rng_scale: u32,
    pub temp_offset: i16,
    pub temp_scale: u16,
}

/// Dados de fabricação do dispositivo
#[derive(Debug, Clone)]
pub struct ManufacturingData {
    pub production_date: u32,
    pub production_location: &'static str,
    pub batch_number: u32,
    pub test_result: bool,
}

/// Device Profile - representa uma unidade física
#[derive(Debug, Clone)]
pub struct DeviceProfile {
    pub serial_number: &'static str,
    pub usb_identity: UsbIdentity,
    pub calibration: Option<CalibrationData>,
    pub manufacturing: Option<ManufacturingData>,
}

impl DeviceProfile {
    /// Cria um novo Device Profile
    pub const fn new(
        serial_number: &'static str,
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
