//! OpenKey Crypto (`no_std`)
//!
//! Abstrações criptográficas: ECC, SHA, AES, RNG.

#![no_std]

/// Versão do módulo criptográfico
pub const CRYPTO_VERSION: &str = env!("CARGO_PKG_VERSION");
