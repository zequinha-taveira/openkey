//! OpenKey Boot (`no_std`)
//!
//! Bootloader e sequência de inicialização segura:
//! verificação de assinatura, Self-Test (POST) e
//! transferência segura de controle ao firmware principal.
//!
//! ## Arquitetura
//!
//! ```text
//!  ┌─────────────────────────────────────────────┐
//!  │           Secure Boot                        │
//!  │  (verificação ECDSA P-256 do firmware)       │
//!  ├─────────────────────────────────────────────┤
//!  │           Dual-Bank Bootloader               │
//!  │  (Bank A ativo / Bank B staging)             │
//!  ├─────────────────────────────────────────────┤
//!  │           Self-Test (POST)                   │
//!  │  (TRNG health check, memory test)            │
//!  └─────────────────────────────────────────────┘
//! ```
//!
//! ## Fluxo de Boot
//!
//! 1. **POST** — Power-On Self Test (TRNG, memória)
//! 2. **Verify** — Verifica assinatura ECDSA P-256 do bank ativo
//! 3. **Jump** — Salta para firmware principal
//!
//! Em caso de falha na verificação:
//! - Se Bank B (staging) for válido → rollback para Bank B
//! - Se nenhum bank for válido → falha crítica (LED piscando)

#![no_std]

use openkey_crypto::keys::{verify_p256_signature, P256_PUBLIC_KEY_SIZE, P256_SIGNATURE_SIZE};
use openkey_platform::hal::{FlashError, FlashStorageProvider, HalError, OtpProvider, RngProvider};
use sha2::{Digest, Sha256};

/// Versão do módulo de boot
pub const BOOT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Tamanho do bloco de verificação (tamanho do bloco a ser assinado)
pub const VERIFY_BLOCK_SIZE: usize = 1024;

/// Tamanho do header de assinatura no início de cada bank
/// Layout: magic(4) + size(4) + hash(32) + signature(72) + reserved(8) = 120
pub const SIGNATURE_HEADER_SIZE: usize = 120;

/// Offset do magic number
pub const MAGIC_OFFSET: usize = 0;
/// Offset do tamanho da imagem
pub const IMAGE_SIZE_OFFSET: usize = 4;
/// Offset do hash da imagem
pub const IMAGE_HASH_OFFSET: usize = 8;
/// Offset da assinatura
pub const SIGNATURE_OFFSET: usize = 40;

/// Magic number para imagens válidas
pub const IMAGE_MAGIC: [u8; 4] = *b"OKFI";

/// Estado do boot
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootState {
    /// Boot bem-sucedido
    Ok,
    /// Falha na verificação — rollback para bank B
    Rollback,
    /// Falha crítica — nenhum bank válido
    CriticalFailure,
}

/// Resultado da verificação de boot
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootResult {
    pub state: BootState,
    pub bank: BankId,
}

/// Identificador de bank de flash
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BankId {
    BankA,
    BankB,
}

/// Layout de memória dual-bank
#[derive(Debug, Clone, Copy)]
pub struct DualBankLayout {
    pub bank_a_offset: u32,
    pub bank_b_offset: u32,
    pub bank_size: u32,
}

/// Erro de boot
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootError {
    /// Flash error
    Flash,
    /// Assinatura inválida
    InvalidSignature,
    /// OTP não programada
    OtpNotProgrammed,
    /// RNG não saudável
    RngNotHealthy,
    /// Layout inválido
    InvalidLayout,
    /// Imagem corrompida
    CorruptedImage,
}

impl From<HalError> for BootError {
    fn from(err: HalError) -> Self {
        match err {
            HalError::HardwareFailure => BootError::Flash,
            HalError::RngNotHealthy => BootError::RngNotHealthy,
            _ => BootError::Flash,
        }
    }
}

impl From<FlashError> for BootError {
    fn from(_err: FlashError) -> Self {
        BootError::Flash
    }
}

/// Trait para Secure Boot Provider
///
/// Define o contrato para verificação de integridade e autenticidade
/// do firmware usando assinatura ECDSA P-256.
pub trait SecureBootProvider {
    /// Verifica a assinatura de uma imagem de firmware
    ///
    /// # Arguments
    /// * `flash` — Provedor de flash para leitura da imagem
    /// * `offset` — Offset inicial da imagem no flash
    /// * `size` — Tamanho da imagem em bytes
    ///
    /// # Returns
    /// `Ok(())` se a assinatura for válida, `Err` caso contrário
    fn verify_image(
        &mut self,
        flash: &mut dyn FlashStorageProvider,
        offset: u32,
        size: u32,
    ) -> Result<(), BootError>;

    /// Verifica se uma imagem é válida (magic number + tamanho)
    fn is_valid_image(
        &mut self,
        flash: &mut dyn FlashStorageProvider,
        offset: u32,
    ) -> Result<bool, BootError>;

    /// Retorna o tamanho da imagem no offset especificado
    fn image_size(
        &mut self,
        flash: &mut dyn FlashStorageProvider,
        offset: u32,
    ) -> Result<u32, BootError>;
}

/// Provedor de chave de atestação para Secure Boot
///
/// Em produção, a chave pública de verificação é armazenada em OTP.
/// Em simulação, pode ser fornecida via software.
pub trait BootKeyProvider {
    /// Retorna a chave pública de verificação (P-256, 64 bytes)
    fn public_key(&self) -> Result<[u8; P256_PUBLIC_KEY_SIZE], BootError>;
}

/// Secure Boot Manager
///
/// Orquestra a verificação de assinatura do firmware e o
/// mecanismo de rollback dual-bank.
pub struct SecureBootManager<'a> {
    key_provider: &'a dyn BootKeyProvider,
    layout: DualBankLayout,
}

impl<'a> SecureBootManager<'a> {
    /// Cria um novo Secure Boot Manager
    pub fn new(key_provider: &'a dyn BootKeyProvider, layout: DualBankLayout) -> Self {
        Self {
            key_provider,
            layout,
        }
    }

    /// Executa o processo de boot seguro
    ///
    /// 1. Verifica Bank A (ativo)
    /// 2. Se falhar, verifica Bank B (staging)
    /// 3. Retorna o resultado do boot
    pub fn boot(
        &mut self,
        flash: &mut dyn FlashStorageProvider,
        rng: &mut dyn RngProvider,
    ) -> Result<BootResult, BootError> {
        // POST: Verifica saúde do RNG
        if !rng.is_healthy() {
            return Err(BootError::RngNotHealthy);
        }

        // Verifica Bank A (ativo)
        let bank_a_valid = self.verify_bank(flash, BankId::BankA)?;

        if bank_a_valid {
            return Ok(BootResult {
                state: BootState::Ok,
                bank: BankId::BankA,
            });
        }

        // Bank A falhou — tenta rollback para Bank B
        let bank_b_valid = self.verify_bank(flash, BankId::BankB)?;

        if bank_b_valid {
            return Ok(BootResult {
                state: BootState::Rollback,
                bank: BankId::BankB,
            });
        }

        // Nenhum bank válido
        Ok(BootResult {
            state: BootState::CriticalFailure,
            bank: BankId::BankA,
        })
    }

    /// Verifica a integridade e autenticidade de um bank
    fn verify_bank(
        &mut self,
        flash: &mut dyn FlashStorageProvider,
        bank: BankId,
    ) -> Result<bool, BootError> {
        let offset = match bank {
            BankId::BankA => self.layout.bank_a_offset,
            BankId::BankB => self.layout.bank_b_offset,
        };

        // Verifica magic number
        if !self.is_valid_image(flash, offset)? {
            return Ok(false);
        }

        // Obtém tamanho da imagem
        let size = self.image_size(flash, offset)?;
        if size == 0 || size > self.layout.bank_size {
            return Ok(false);
        }

        // Verifica assinatura
        self.verify_image(flash, offset, size)
            .map(|_| true)
            .or(Ok(false))
    }

    /// Calcula o hash SHA-256 de uma região do flash
    fn hash_image(
        &mut self,
        flash: &mut dyn FlashStorageProvider,
        offset: u32,
        size: u32,
    ) -> Result<[u8; 32], BootError> {
        let mut hasher = Sha256::new();
        let mut buf = [0u8; VERIFY_BLOCK_SIZE];
        let mut remaining = size;
        let mut current_offset = offset;

        while remaining > 0 {
            let to_read = remaining.min(VERIFY_BLOCK_SIZE as u32) as usize;
            flash.read(current_offset, &mut buf[..to_read])?;
            hasher.update(&buf[..to_read]);
            remaining -= to_read as u32;
            current_offset += to_read as u32;
        }

        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        Ok(hash)
    }
}

impl<'a> SecureBootProvider for SecureBootManager<'a> {
    fn verify_image(
        &mut self,
        flash: &mut dyn FlashStorageProvider,
        offset: u32,
        size: u32,
    ) -> Result<(), BootError> {
        // Lê o header da assinatura
        let mut header = [0u8; SIGNATURE_HEADER_SIZE];
        flash.read(offset, &mut header)?;

        // Verifica magic number
        if header[MAGIC_OFFSET..MAGIC_OFFSET + 4] != IMAGE_MAGIC {
            return Err(BootError::CorruptedImage);
        }

        // Valida o tamanho informado contra o limite do bank
        if size == 0 || size > self.layout.bank_size {
            return Err(BootError::CorruptedImage);
        }

        // Lê o tamanho da imagem do header
        let header_size = u32::from_le_bytes([
            header[IMAGE_SIZE_OFFSET],
            header[IMAGE_SIZE_OFFSET + 1],
            header[IMAGE_SIZE_OFFSET + 2],
            header[IMAGE_SIZE_OFFSET + 3],
        ]);

        // Lê o hash armazenado
        let mut stored_hash = [0u8; 32];
        stored_hash.copy_from_slice(&header[IMAGE_HASH_OFFSET..IMAGE_HASH_OFFSET + 32]);

        // Lê a assinatura
        let mut signature = [0u8; P256_SIGNATURE_SIZE];
        signature
            .copy_from_slice(&header[SIGNATURE_OFFSET..SIGNATURE_OFFSET + P256_SIGNATURE_SIZE]);

        // Calcula o hash da imagem (excluindo o header de assinatura)
        // O tamanho do header não é confiável: limita-se ao `size` já
        // validado contra `bank_size` para evitar leitura excessiva
        let image_data_offset = offset + SIGNATURE_HEADER_SIZE as u32;
        let image_data_size = header_size
            .min(size)
            .saturating_sub(SIGNATURE_HEADER_SIZE as u32);
        let computed_hash = self.hash_image(flash, image_data_offset, image_data_size)?;

        // Verifica se o hash calculado corresponde ao hash armazenado
        if computed_hash != stored_hash {
            return Err(BootError::CorruptedImage);
        }

        // Obtém a chave pública de verificação
        let public_key = self.key_provider.public_key()?;

        // Verifica a assinatura ECDSA P-256
        // A assinatura é feita sobre o hash da imagem
        verify_p256_signature(&public_key, &computed_hash, &signature)
            .map_err(|_| BootError::InvalidSignature)?;

        Ok(())
    }

    fn is_valid_image(
        &mut self,
        flash: &mut dyn FlashStorageProvider,
        offset: u32,
    ) -> Result<bool, BootError> {
        let mut magic = [0u8; 4];
        flash.read(offset + MAGIC_OFFSET as u32, &mut magic)?;
        Ok(magic == IMAGE_MAGIC)
    }

    fn image_size(
        &mut self,
        flash: &mut dyn FlashStorageProvider,
        offset: u32,
    ) -> Result<u32, BootError> {
        let mut size_bytes = [0u8; 4];
        flash.read(offset + IMAGE_SIZE_OFFSET as u32, &mut size_bytes)?;
        let size = u32::from_le_bytes(size_bytes);
        if size == 0 || size > self.layout.bank_size {
            // Tamanho inválido (0 ou maior que o bank)
            return Err(BootError::CorruptedImage);
        }
        Ok(size)
    }
}

/// Executa o Self-Test (POST) no boot
pub fn run_self_test(rng: &mut dyn RngProvider, otp: &dyn OtpProvider) -> Result<(), BootError> {
    // Verifica saúde do RNG
    if !rng.is_healthy() {
        return Err(BootError::RngNotHealthy);
    }

    // Verifica se a OTP está acessível
    if otp.total_size() == 0 {
        return Err(BootError::OtpNotProgrammed);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use openkey_platform::hal::OtpError;

    /// Mock key provider para testes
    struct TestKeyProvider;

    impl BootKeyProvider for TestKeyProvider {
        fn public_key(&self) -> Result<[u8; P256_PUBLIC_KEY_SIZE], BootError> {
            // Chave pública P-256 de teste (não válida para produção)
            let mut key = [0u8; P256_PUBLIC_KEY_SIZE];
            key[0] = 0x04; // Prefixo de ponto não comprimido
            Ok(key)
        }
    }

    /// Mock flash para testes
    struct MockFlash {
        data: [u8; 8192],
    }

    impl MockFlash {
        const fn new() -> Self {
            Self { data: [0xFF; 8192] }
        }
    }

    impl FlashStorageProvider for MockFlash {
        fn read(&mut self, offset: u32, buf: &mut [u8]) -> Result<(), FlashError> {
            let start = offset as usize;
            let end = start + buf.len();
            if end > self.data.len() {
                return Err(FlashError::OutOfBounds);
            }
            buf.copy_from_slice(&self.data[start..end]);
            Ok(())
        }

        fn write(&mut self, offset: u32, data: &[u8]) -> Result<(), FlashError> {
            let start = offset as usize;
            let end = start + data.len();
            if end > self.data.len() {
                return Err(FlashError::OutOfBounds);
            }
            self.data[start..end].copy_from_slice(data);
            Ok(())
        }

        fn erase(&mut self, offset: u32, len: u32) -> Result<(), FlashError> {
            let start = offset as usize;
            let end = start + len as usize;
            if end > self.data.len() {
                return Err(FlashError::OutOfBounds);
            }
            self.data[start..end].fill(0xFF);
            Ok(())
        }

        fn total_size(&self) -> u32 {
            self.data.len() as u32
        }
    }

    /// Mock RNG para testes
    struct MockRng;

    impl RngProvider for MockRng {
        fn fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), HalError> {
            dest.fill(0x42);
            Ok(())
        }

        fn is_healthy(&self) -> bool {
            true
        }
    }

    /// Mock OTP para testes
    struct MockOtp;

    impl OtpProvider for MockOtp {
        fn read(&self, _offset: u32, buf: &mut [u8]) -> Result<(), OtpError> {
            buf.fill(0);
            Ok(())
        }

        fn is_programmed(&self, _offset: u32, _len: usize) -> Result<bool, OtpError> {
            Ok(true)
        }

        fn total_size(&self) -> u32 {
            1024
        }
    }

    #[test]
    fn test_is_valid_image() {
        let mut flash = MockFlash::new();
        let key_provider = TestKeyProvider;
        let layout = DualBankLayout {
            bank_a_offset: 0,
            bank_b_offset: 4096,
            bank_size: 4096,
        };
        let mut boot = SecureBootManager::new(&key_provider, layout);

        // Imagem inválida (flash vazio)
        assert!(!boot.is_valid_image(&mut flash, 0).unwrap());

        // Escreve magic number
        flash.data[0..4].copy_from_slice(&IMAGE_MAGIC);
        assert!(boot.is_valid_image(&mut flash, 0).unwrap());
    }

    #[test]
    fn test_image_size() {
        let mut flash = MockFlash::new();
        let key_provider = TestKeyProvider;
        let layout = DualBankLayout {
            bank_a_offset: 0,
            bank_b_offset: 4096,
            bank_size: 4096,
        };
        let mut boot = SecureBootManager::new(&key_provider, layout);

        // Escreve tamanho da imagem (1024 bytes)
        flash.data[4..8].copy_from_slice(&1024u32.to_le_bytes());
        assert_eq!(boot.image_size(&mut flash, 0).unwrap(), 1024);
    }

    #[test]
    fn test_image_size_rejects_size_above_bank_size() {
        let mut flash = MockFlash::new();
        let key_provider = TestKeyProvider;
        let layout = DualBankLayout {
            bank_a_offset: 0,
            bank_b_offset: 4096,
            bank_size: 4096,
        };
        let mut boot = SecureBootManager::new(&key_provider, layout);

        // Tamanho acima do limite do bank (anteriormente aceito até 1 MiB)
        flash.data[4..8].copy_from_slice(&8192u32.to_le_bytes());
        assert_eq!(
            boot.image_size(&mut flash, 0),
            Err(BootError::CorruptedImage)
        );
    }

    #[test]
    fn test_verify_image_rejects_size_above_bank_size() {
        let mut flash = MockFlash::new();
        let key_provider = TestKeyProvider;
        let layout = DualBankLayout {
            bank_a_offset: 0,
            bank_b_offset: 4096,
            bank_size: 4096,
        };
        let mut boot = SecureBootManager::new(&key_provider, layout);

        flash.data[0..4].copy_from_slice(&IMAGE_MAGIC);
        // Header corrompido com tamanho máximo e `size` informado acima do bank
        flash.data[4..8].copy_from_slice(&u32::MAX.to_le_bytes());
        assert_eq!(
            boot.verify_image(&mut flash, 0, 8192),
            Err(BootError::CorruptedImage)
        );
    }

    #[test]
    fn test_run_self_test() {
        let mut rng = MockRng;
        let otp = MockOtp;

        // Self-test deve passar com RNG saudável e OTP acessível
        assert!(run_self_test(&mut rng, &otp).is_ok());
    }

    #[test]
    fn test_dual_bank_layout() {
        let layout = DualBankLayout {
            bank_a_offset: 0,
            bank_b_offset: 0x80000,
            bank_size: 0x80000,
        };
        assert_eq!(layout.bank_a_offset, 0);
        assert_eq!(layout.bank_b_offset, 0x80000);
        assert_eq!(layout.bank_size, 0x80000);
    }
}
