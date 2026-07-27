//! OpenKey Platform (`no_std`)
//!
//! Platform Abstraction Layer, Board/Device Profiles, Configuration Manager
//! e Platform Services para o OpenKey Framework.
//!
//! Arquitetura:
//! ```text
//! OpenKey Core
//!         │
//! Platform Services
//!         │
//! Configuration Manager
//!         │
//! Board Profile
//!         │
//! Device Profile
//!         │
//! HAL
//!         │
//! Startup
//! ```

#![no_std]

pub mod app_config;
pub mod board;
pub mod config;
pub mod device;
pub mod hal;
pub mod services;

pub use app_config::{
    AppConfig, CcidConfig, Ctap2Config, LoggingConfig, OpenPgpConfig, PivConfig, SecurityPolicies,
};
pub use board::{
    BoardProfile, ButtonConfig, FlashConfig, GpioPin, LedConfig, OptionalFeatures, UsbConfig,
};
pub use config::{ConfigurationManager, ProvisioningState};
pub use device::{CalibrationData, DeviceProfile, ManufacturingData, UsbIdentity};
pub use hal::HalError;
pub use services::{HardwareProviders, PlatformServices};

/// Versão da plataforma
pub const PLATFORM_VERSION: &str = env!("CARGO_PKG_VERSION");
