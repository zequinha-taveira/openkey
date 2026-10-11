//! OpenKey Protocols (`no_std`)
//!
//! Implementação dos protocolos CTAP2, CBOR, HID e WebAuthn.

#![no_std]

pub mod cbor;
pub mod cose;
pub mod ctap2;
pub mod ctap_hid;
pub mod webauthn;

/// Versão do módulo de protocolos
pub const PROTOCOLS_VERSION: &str = env!("CARGO_PKG_VERSION");
