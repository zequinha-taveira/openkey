//! OpenKey Core Security Engine (`no_std`)
//!
//! Esta crate implementa a lógica agnóstica de protocolo e segurança do OpenKey.
//! O Core depende da Platform crate para acesso a HAL, Board Profile,
//! Device Profile e Configuration Manager, mantendo-se 100% livre de
//! acoplamento a registradores de hardware.

#![no_std]

pub mod error;

pub use openkey_platform::{
    app_config, board, config, device, hal, services, AppConfig, BoardProfile,
    ConfigurationManager, DeviceProfile, HardwareProviders, PlatformServices, ProvisioningState,
};

/// Versão do núcleo de protocolo OpenKey
pub const OPENKEY_CORE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Retorna o status de inicialização do núceo
pub fn core_info() -> &'static str {
    "OpenKey Core v0.1.0 (no_std)"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_info() {
        assert_eq!(core_info(), "OpenKey Core v0.1.0 (no_std)");
    }

    #[test]
    fn test_platform_version() {
        assert!(!openkey_platform::PLATFORM_VERSION.is_empty());
    }
}
