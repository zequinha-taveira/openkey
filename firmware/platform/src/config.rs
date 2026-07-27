//! Configuration Manager - gerencia configuração persistente
//!
//! O firmware nunca deve assumir características da placa.
//! O OpenKey Configurator grava Board Profile, Device Profile e
//! Application Configuration na Flash persistente.

use crate::app_config::AppConfig;
use crate::board::BoardProfile;
use crate::device::DeviceProfile;
use crate::hal::{FlashStorageProvider, HalError};

/// Magic bytes para validação de configuração no flash
const CONFIG_MAGIC: &[u8; 4] = b"OKCF";

/// Versão do formato de configuração
const CONFIG_VERSION: u32 = 1;

/// Offset onde a configuração é armazenada no flash
const CONFIG_FLASH_OFFSET: u32 = 0x0000;

/// Estado de provisionamento do dispositivo
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProvisioningState {
    /// Não provisionado - nenhuma configuração válida no flash
    Unprovisioned,
    /// Parcialmente provisionado - alguns dados de configuração presentes
    Partial,
    /// Totalmente provisionado - todos os dados de configuração presentes e válidos
    Provisioned,
}

/// Configuration Manager - gerencia Board Profile, Device Profile e Application Configuration
///
/// O Configuration Manager não mantém uma referência ao flash. Em vez disso,
/// o flash é passado como parâmetro para os métodos `load` e `save`, evitando
/// empréstimos múltos e permitindo que o Platform Services gerencie o flash.
pub struct ConfigurationManager {
    board: Option<BoardProfile>,
    device: Option<DeviceProfile>,
    app: Option<AppConfig>,
    state: ProvisioningState,
}

impl ConfigurationManager {
    /// Cria um novo Configuration Manager sem configuração carregada
    pub const fn new() -> Self {
        Self {
            board: None,
            device: None,
            app: None,
            state: ProvisioningState::Unprovisioned,
        }
    }

    /// Carrega a configuração do flash persistente
    pub fn load(&mut self, flash: &mut dyn FlashStorageProvider) -> Result<(), HalError> {
        let mut magic = [0u8; 4];
        flash.read(CONFIG_FLASH_OFFSET, &mut magic)?;

        if &magic != CONFIG_MAGIC {
            self.state = ProvisioningState::Unprovisioned;
            return Err(HalError::InvalidParameter);
        }

        let mut version = [0u8; 4];
        flash.read(CONFIG_FLASH_OFFSET + 4, &mut version)?;

        let ver = u32::from_le_bytes(version);
        if ver != CONFIG_VERSION {
            self.state = ProvisioningState::Unprovisioned;
            return Err(HalError::InvalidParameter);
        }

        self.state = ProvisioningState::Provisioned;
        Ok(())
    }

    /// Salva a configuração no flash persistente
    pub fn save(
        &mut self,
        flash: &mut dyn FlashStorageProvider,
        board: &BoardProfile,
        device: &DeviceProfile,
        app: &AppConfig,
    ) -> Result<(), HalError> {
        flash.erase(CONFIG_FLASH_OFFSET, 4096)?;

        flash.write(CONFIG_FLASH_OFFSET, CONFIG_MAGIC)?;

        let version = CONFIG_VERSION.to_le_bytes();
        flash.write(CONFIG_FLASH_OFFSET + 4, &version)?;

        self.board = Some(board.clone());
        self.device = Some(device.clone());
        self.app = Some(app.clone());
        self.state = ProvisioningState::Provisioned;
        Ok(())
    }

    /// Retorna o Board Profile carregado
    pub fn board_profile(&self) -> Option<&BoardProfile> {
        self.board.as_ref()
    }

    /// Retorna o Device Profile carregado
    pub fn device_profile(&self) -> Option<&DeviceProfile> {
        self.device.as_ref()
    }

    /// Retorna a Application Configuration carregada
    pub fn app_config(&self) -> Option<&AppConfig> {
        self.app.as_ref()
    }

    /// Retorna o estado de provisionamento
    pub fn provisioning_state(&self) -> ProvisioningState {
        self.state
    }

    /// Verifica se o dispositivo está totalmente provisionado
    pub fn is_provisioned(&self) -> bool {
        self.state == ProvisioningState::Provisioned
    }
}

impl Default for ConfigurationManager {
    fn default() -> Self {
        Self::new()
    }
}
