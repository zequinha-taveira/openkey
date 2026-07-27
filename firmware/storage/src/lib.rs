//! OpenKey Storage (`no_std`)
//!
//! Gerenciamento de armazenamento persistente e wear-leveling.

#![no_std]

/// Versão do módulo de armazenamento
pub const STORAGE_VERSION: &str = env!("CARGO_PKG_VERSION");
