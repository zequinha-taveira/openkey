//! OpenKey Crypto (`no_std`)
//!
//! Abstrações criptográficas: ECC, SHA, AES, RNG.
//!
//! A autenticação da configuração persistente é encapsulada aqui para que a
//! camada de plataforma não manipule detalhes de AES-GCM.

#![no_std]

use aes_gcm::{
    aead::{AeadInOut, KeyInit},
    Aes256Gcm, Nonce, Tag,
};

/// Tamanho de uma chave AES-256 em bytes.
pub const CONFIG_AEAD_KEY_SIZE: usize = 32;
/// Tamanho do nonce AES-GCM em bytes.
pub const CONFIG_AEAD_NONCE_SIZE: usize = 12;
/// Tamanho da tag de autenticação AES-GCM em bytes.
pub const CONFIG_AEAD_TAG_SIZE: usize = 16;

/// Erro durante autenticação ou cifragem da configuração.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AeadError {
    AuthenticationFailed,
}

/// Cifra um payload de configuração in-place com AES-256-GCM.
pub fn encrypt_config(
    key: &[u8; CONFIG_AEAD_KEY_SIZE],
    nonce: &[u8; CONFIG_AEAD_NONCE_SIZE],
    aad: &[u8],
    payload: &mut [u8],
) -> Result<[u8; CONFIG_AEAD_TAG_SIZE], AeadError> {
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| AeadError::AuthenticationFailed)?;
    let nonce = Nonce::try_from(nonce.as_slice()).map_err(|_| AeadError::AuthenticationFailed)?;
    let tag = cipher
        .encrypt_inout_detached(&nonce, aad, payload.into())
        .map_err(|_| AeadError::AuthenticationFailed)?;
    let mut output = [0u8; CONFIG_AEAD_TAG_SIZE];
    output.copy_from_slice(&tag);
    Ok(output)
}

/// Autentica e decifra um payload de configuração in-place com AES-256-GCM.
pub fn decrypt_config(
    key: &[u8; CONFIG_AEAD_KEY_SIZE],
    nonce: &[u8; CONFIG_AEAD_NONCE_SIZE],
    aad: &[u8],
    payload: &mut [u8],
    tag: &[u8; CONFIG_AEAD_TAG_SIZE],
) -> Result<(), AeadError> {
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| AeadError::AuthenticationFailed)?;
    let nonce = Nonce::try_from(nonce.as_slice()).map_err(|_| AeadError::AuthenticationFailed)?;
    let tag = Tag::try_from(tag.as_slice()).map_err(|_| AeadError::AuthenticationFailed)?;
    cipher
        .decrypt_inout_detached(&nonce, aad, payload.into(), &tag)
        .map_err(|_| AeadError::AuthenticationFailed)
}

/// Versão do módulo criptográfico
pub const CRYPTO_VERSION: &str = env!("CARGO_PKG_VERSION");
