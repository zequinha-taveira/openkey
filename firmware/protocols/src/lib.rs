//! OpenKey Protocols (`no_std`)
//!
//! Implementação dos protocolos CTAP2, CBOR, HID e WebAuthn.

#![no_std]

/// Versão do módulo de protocolos
pub const PROTOCOLS_VERSION: &str = env!("CARGO_PKG_VERSION");
