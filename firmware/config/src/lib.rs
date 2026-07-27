//! OpenKey Config (`no_std`)
//!
//! Gerenciamento de configuração persistente do firmware:
//! carregamento, validação e atualização segura de Board Profile,
//! Device Profile e Application Configuration na Flash.

#![no_std]

/// Versão do módulo de configuração do firmware
pub const CONFIG_VERSION: &str = env!("CARGO_PKG_VERSION");
