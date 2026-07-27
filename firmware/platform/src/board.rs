//! Board Profile - descrição de dados da placa
//!
//! Toda placa é descrita por dados. Nunca codifique GPIOs diretamente
//! no firmware quando puderem ser parametrizados.

/// Identificador estável de um perfil de placa.
///
/// Este valor é persistido na configuração do dispositivo e resolvido por um
/// catálogo compilado. O perfil completo nunca é desserializado da Flash.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoardProfileId(pub [u8; 16]);

/// Catálogo de perfis de placa disponíveis no firmware.
pub trait BoardProfileCatalog {
    /// Retorna o perfil associado ao identificador persistido.
    fn find(&self, id: BoardProfileId) -> Option<&'static BoardProfile>;
}

/// Identificador de pino GPIO
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GpioPin {
    pub port: u8,
    pub pin: u8,
}

/// Configuração de um LED na placa
#[derive(Debug, Clone, Copy)]
pub struct LedConfig {
    pub pin: GpioPin,
    pub active_high: bool,
}

/// Configuração de um botão na placa
#[derive(Debug, Clone, Copy)]
pub struct ButtonConfig {
    pub pin: GpioPin,
    pub active_low: bool,
    pub pull_up: bool,
}

/// Configuração de USB na placa
#[derive(Debug, Clone, Copy)]
pub struct UsbConfig {
    pub vid: u16,
    pub pid: u16,
    pub bcd_version: u16,
    pub max_packet_size: u8,
}

/// Configuração de flash na placa
#[derive(Debug, Clone, Copy)]
pub struct FlashConfig {
    pub total_size: u32,
    pub page_size: u32,
    pub sector_size: u32,
}

/// Recursos opcionais suportados pela placa
#[derive(Debug, Clone, Copy)]
pub struct OptionalFeatures {
    pub has_nfc: bool,
    pub has_ble: bool,
    pub has_secure_element: bool,
    pub has_tamper_detect: bool,
}

/// Board Profile - descreve uma placa de hardware por dados
#[derive(Debug, Clone)]
pub struct BoardProfile {
    pub id: BoardProfileId,
    pub manufacturer: &'static str,
    pub model: &'static str,
    pub revision: &'static str,
    pub flash: FlashConfig,
    pub usb: UsbConfig,
    pub led: Option<LedConfig>,
    pub button: Option<ButtonConfig>,
    pub features: OptionalFeatures,
}

impl BoardProfile {
    /// Cria um novo Board Profile
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        id: BoardProfileId,
        manufacturer: &'static str,
        model: &'static str,
        revision: &'static str,
        flash: FlashConfig,
        usb: UsbConfig,
        led: Option<LedConfig>,
        button: Option<ButtonConfig>,
        features: OptionalFeatures,
    ) -> Self {
        Self {
            id,
            manufacturer,
            model,
            revision,
            flash,
            usb,
            led,
            button,
            features,
        }
    }
}
