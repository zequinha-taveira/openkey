//! Platform Services - orquestra Board Profile, Device Profile e HAL
//!
//! Fornece serviços unificados ao OpenKey Core, combinando HAL,
//! Configuration Manager, Board Profile e Device Profile.

use crate::app_config::AppConfig;
use crate::board::{BoardProfile, BoardProfileCatalog};
use crate::config::{ConfigStorageLayout, ConfigurationError, ConfigurationManager};
use crate::device::DeviceProfile;
use crate::hal::{
    FlashStorageProvider, GpioProvider, RngProvider, TimerProvider, UsbTransportProvider,
    WatchdogProvider,
};

/// Plataforma de hardware com todos os provedores HAL
pub struct HardwareProviders<'a> {
    pub rng: &'a mut dyn RngProvider,
    pub flash: &'a mut dyn FlashStorageProvider,
    pub usb: &'a mut dyn UsbTransportProvider,
    pub gpio: &'a mut dyn GpioProvider,
    pub timer: &'a mut dyn TimerProvider,
    pub watchdog: &'a mut dyn WatchdogProvider,
}

/// Platform Services - orquestra todos os componentes da plataforma
pub struct PlatformServices<'a> {
    hw: HardwareProviders<'a>,
    config_mgr: ConfigurationManager,
}

impl<'a> PlatformServices<'a> {
    /// Cria novos Platform Services com os provedores de hardware especificados
    pub fn new(hw: HardwareProviders<'a>) -> Self {
        Self {
            hw,
            config_mgr: ConfigurationManager::new(),
        }
    }

    /// Carrega a configuração do flash
    pub fn load_config(
        &mut self,
        catalog: &dyn BoardProfileCatalog,
        layout: ConfigStorageLayout,
    ) -> Result<(), ConfigurationError> {
        self.config_mgr.load(self.hw.flash, catalog, layout)
    }

    /// Retorna referência ao Configuration Manager
    pub fn config(&self) -> &ConfigurationManager {
        &self.config_mgr
    }

    /// Retorna referência mutável ao Configuration Manager
    pub fn config_mut(&mut self) -> &mut ConfigurationManager {
        &mut self.config_mgr
    }

    /// Retorna referência aos provedores de hardware
    pub fn hw(&self) -> &HardwareProviders<'a> {
        &self.hw
    }

    /// Retorna referência mutável aos provedores de hardware
    pub fn hw_mut(&mut self) -> &mut HardwareProviders<'a> {
        &mut self.hw
    }

    /// Retorna o Board Profile carregado
    pub fn board_profile(&self) -> Option<&BoardProfile> {
        self.config_mgr.board_profile()
    }

    /// Retorna o Device Profile carregado
    pub fn device_profile(&self) -> Option<&DeviceProfile> {
        self.config_mgr.device_profile()
    }

    /// Retorna a Application Configuration carregada
    pub fn app_config(&self) -> Option<&AppConfig> {
        self.config_mgr.app_config()
    }

    /// Alimenta o watchdog
    pub fn feed_watchdog(&mut self) {
        self.hw.watchdog.feed();
    }

    /// Verifica se o dispositivo está provisionado
    pub fn is_provisioned(&self) -> bool {
        self.config_mgr.is_provisioned()
    }
}
