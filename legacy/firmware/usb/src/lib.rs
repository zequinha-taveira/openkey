//! OpenKey USB (`no_std`)
//!
//! Camada de transporte USB HID: inicialização, polling e framing de pacotes CTAP2.
//! Implementa a interface USB HID conforme FIDO2 / CTAP2 Spec, Section 11.

#![no_std]

/// Versão do módulo USB
pub const USB_VERSION: &str = env!("CARGO_PKG_VERSION");
