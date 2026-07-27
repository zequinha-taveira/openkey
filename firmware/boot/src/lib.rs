//! OpenKey Boot (`no_std`)
//!
//! Bootloader e sequência de inicialização segura:
//! verificação de assinatura, Self-Test (POST) e
//! transferência segura de controle ao firmware principal.

#![no_std]

/// Versão do módulo de boot
pub const BOOT_VERSION: &str = env!("CARGO_PKG_VERSION");
