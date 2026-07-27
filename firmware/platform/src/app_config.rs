//! Application Configuration - configuração independente de hardware
//!
//! Toda configuração da aplicação deve ser independente do hardware.

/// Configuração do protocolo CTAP2
#[derive(Debug, Clone, Copy)]
pub struct Ctap2Config {
    pub enable_fido2_0: bool,
    pub enable_fido2_1: bool,
    pub enable_resident_keys: bool,
    pub enable_user_verification: bool,
    pub enable_credential_management: bool,
    pub enable_authenticator_reset: bool,
    pub enable_hmac_secret: bool,
    pub enable_large_blob: bool,
    pub max_credential_count: u16,
    pub max_blob_size: u16,
}

/// Configuração do protocolo CCID
#[derive(Debug, Clone, Copy)]
pub struct CcidConfig {
    pub enable_ccid: bool,
    pub max_message_length: u32,
    pub max_busy_slots: u8,
}

/// Configuração do protocolo OpenPGP
#[derive(Debug, Clone, Copy)]
pub struct OpenPgpConfig {
    pub enable_openpgp: bool,
    pub max_key_size: u16,
    pub enable_rsa: bool,
    pub enable_ecdsa: bool,
}

/// Configuração do protocolo PIV
#[derive(Debug, Clone, Copy)]
pub struct PivConfig {
    pub enable_piv: bool,
    pub max_key_size: u16,
    pub enable_rsa: bool,
    pub enable_ecdsa: bool,
}

/// Configuração de logging
#[derive(Debug, Clone, Copy)]
pub struct LoggingConfig {
    pub enable_logging: bool,
    pub log_level: u8,
    pub log_to_usb: bool,
    pub log_to_flash: bool,
    pub max_log_entries: u16,
}

/// Políticas de segurança
#[derive(Debug, Clone, Copy)]
pub struct SecurityPolicies {
    pub require_user_presence: bool,
    pub require_user_verification: bool,
    pub pin_min_length: u8,
    pub pin_max_retries: u8,
    pub pin_lockout_threshold: u8,
    pub pin_lockout_duration_ms: u32,
    pub firmware_update_requires_signature: bool,
    pub disable_factory_reset: bool,
}

/// Application Configuration - configuração independente de hardware
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub ctap2: Ctap2Config,
    pub ccid: CcidConfig,
    pub openpgp: OpenPgpConfig,
    pub piv: PivConfig,
    pub logging: LoggingConfig,
    pub policies: SecurityPolicies,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            ctap2: Ctap2Config {
                enable_fido2_0: true,
                enable_fido2_1: true,
                enable_resident_keys: true,
                enable_user_verification: false,
                enable_credential_management: true,
                enable_authenticator_reset: true,
                enable_hmac_secret: true,
                enable_large_blob: false,
                max_credential_count: 128,
                max_blob_size: 0,
            },
            ccid: CcidConfig {
                enable_ccid: false,
                max_message_length: 0,
                max_busy_slots: 0,
            },
            openpgp: OpenPgpConfig {
                enable_openpgp: false,
                max_key_size: 0,
                enable_rsa: false,
                enable_ecdsa: false,
            },
            piv: PivConfig {
                enable_piv: false,
                max_key_size: 0,
                enable_rsa: false,
                enable_ecdsa: false,
            },
            logging: LoggingConfig {
                enable_logging: false,
                log_level: 0,
                log_to_usb: false,
                log_to_flash: false,
                max_log_entries: 0,
            },
            policies: SecurityPolicies {
                require_user_presence: true,
                require_user_verification: false,
                pin_min_length: 4,
                pin_max_retries: 3,
                pin_lockout_threshold: 8,
                pin_lockout_duration_ms: 1000,
                firmware_update_requires_signature: true,
                disable_factory_reset: false,
            },
        }
    }
}
