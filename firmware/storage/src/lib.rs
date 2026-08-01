//! OpenKey Storage (`no_std`)
//!
//! Gerenciamento de armazenamento persistente com wear-leveling circular
//! e criptografia AES-256-GCM para dados sensíveis.
//!
//! Arquitetura:
//! ```text
//!  ┌─────────────────────────────────────────────┐
//!  │           Storage Manager                    │
//!  │  (wear-leveling, page management)            │
//!  ├─────────────────────────────────────────────┤
//!  │           Crypto Layer                       │
//!  │  (AES-256-GCM encrypt/decrypt)               │
//!  ├─────────────────────────────────────────────┤
//!  │           Flash HAL                          │
//!  │  (FlashStorageProvider trait)                │
//!  └─────────────────────────────────────────────┘
//! ```

#![no_std]

use core::marker::PhantomData;
use openkey_core::hal::{FlashError, FlashStorageProvider, FLASH_PAGE_SIZE};

/// Tamanho máximo de uma página de dados (4 KiB)
pub const PAGE_SIZE: usize = FLASH_PAGE_SIZE as usize;

/// Tamanho do cabeçalho de página (metadados)
pub const PAGE_HEADER_SIZE: usize = 16;

/// Tamanho útil de dados por página
pub const PAGE_DATA_SIZE: usize = PAGE_SIZE - PAGE_HEADER_SIZE;

/// Tamanho máximo de dados criptografados por página
/// Garante espaço para header + nonce + tag
pub const MAX_ENCRYPTED_DATA_SIZE: usize = PAGE_DATA_SIZE - NONCE_SIZE - TAG_SIZE;

/// Tamanho do nonce AES-GCM
pub const NONCE_SIZE: usize = 12;

/// Tamanho da tag de autenticação AES-GCM
pub const TAG_SIZE: usize = 16;

/// Tamanho da chave AES-256
pub const KEY_SIZE: usize = 32;

/// Número máximo de páginas de wear-leveling
pub const MAX_PAGES: usize = 128;

/// Estado de uma página de flash
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageState {
    /// Página vazia (0xFF)
    Empty,
    /// Página ativa com dados válidos
    Active,
    /// Página obsoleta (dados substituídos)
    Obsolete,
    /// Página corrompida (falha de integridade)
    Corrupted,
    /// Página em processo de escrita (power-loss recovery)
    /// Indica que a escrita foi interrompida antes de conclusão
    Writing,
}

/// Erro de armazenamento
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageError {
    /// Erro de flash
    Flash(FlashError),
    /// Página não encontrada
    PageNotFound,
    /// Página cheia
    PageFull,
    /// Falha de autenticação
    AuthenticationFailed,
    /// Buffer insuficiente
    BufferTooSmall,
    /// Chave não disponível
    KeyUnavailable,
    /// Armazenamento cheio
    StorageFull,
    /// Dados corrompidos
    CorruptedData,
    /// Operação não suportada
    Unsupported,
}

impl From<FlashError> for StorageError {
    fn from(err: FlashError) -> Self {
        Self::Flash(err)
    }
}

/// Metadados de uma página de flash
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageHeader {
    /// Estado da página
    pub state: PageState,
    /// Número de série da página (monotônico)
    pub sequence: u32,
    /// Tamanho dos dados válidos
    pub data_len: u16,
    /// Checksum CRC-16 dos dados
    pub crc: u16,
    /// Tipo de dado armazenado
    pub data_type: u8,
    /// Flags de proteção
    pub flags: u8,
}

impl PageHeader {
    /// Cria um novo cabeçalho de página
    pub const fn new(data_type: u8, data_len: u16, sequence: u32) -> Self {
        Self {
            state: PageState::Active,
            sequence,
            data_len,
            crc: 0,
            data_type,
            flags: 0,
        }
    }

    /// Cria um cabeçalho para página vazia
    pub const fn empty() -> Self {
        Self {
            state: PageState::Empty,
            sequence: 0,
            data_len: 0,
            crc: 0,
            data_type: 0,
            flags: 0,
        }
    }

    /// Serializa o cabeçalho para bytes
    pub fn serialize(&self) -> [u8; PAGE_HEADER_SIZE] {
        let mut buf = [0u8; PAGE_HEADER_SIZE];
        buf[0] = self.state as u8;
        buf[1..5].copy_from_slice(&self.sequence.to_le_bytes());
        buf[5..7].copy_from_slice(&self.data_len.to_le_bytes());
        buf[7..9].copy_from_slice(&self.crc.to_le_bytes());
        buf[9] = self.data_type;
        buf[10] = self.flags;
        // Reserved
        buf[11..PAGE_HEADER_SIZE].fill(0xFF);
        buf
    }

    /// Deserializa o cabeçalho a partir de bytes
    pub fn deserialize(buf: &[u8]) -> Result<Self, StorageError> {
        if buf.len() < PAGE_HEADER_SIZE {
            return Err(StorageError::BufferTooSmall);
        }
        Ok(Self {
            state: match buf[0] {
                0 => PageState::Empty,
                1 => PageState::Active,
                2 => PageState::Obsolete,
                3 => PageState::Corrupted,
                4 => PageState::Writing,
                _ => PageState::Corrupted,
            },
            sequence: u32::from_le_bytes([buf[1], buf[2], buf[3], buf[4]]),
            data_len: u16::from_le_bytes([buf[5], buf[6]]),
            crc: u16::from_le_bytes([buf[7], buf[8]]),
            data_type: buf[9],
            flags: buf[10],
        })
    }
}

/// Provedor de chave para criptografia de armazenamento
pub trait StorageKeyProvider {
    /// Preenche o buffer com a chave de armazenamento
    fn fill_key(&self, destination: &mut [u8; KEY_SIZE]) -> Result<(), StorageError>;
}

/// Provedor de RNG para nonces
pub trait StorageRngProvider {
    /// Preenche o buffer com bytes aleatórios
    fn fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), StorageError>;
}

/// Slot de armazenamento para wear-leveling circular
#[derive(Debug, Clone, Copy)]
pub struct StorageSlot {
    /// Offset base no flash
    pub offset: u32,
    /// Tamanho total do slot em páginas
    pub page_count: usize,
    /// Tipo de dado
    pub data_type: u8,
}

/// Gerenciador de armazenamento com wear-leveling circular
pub struct StorageManager<'a, F, K, R>
where
    F: FlashStorageProvider,
    K: StorageKeyProvider,
    R: StorageRngProvider,
{
    flash: &'a mut F,
    key_provider: &'a K,
    rng: &'a mut R,
    /// Slots configurados
    slots: [Option<StorageSlot>; 8],
    /// Contador de sequência global
    sequence: u32,
    _phantom: PhantomData<()>,
}

impl<'a, F, K, R> StorageManager<'a, F, K, R>
where
    F: FlashStorageProvider,
    K: StorageKeyProvider,
    R: StorageRngProvider,
{
    /// Cria um novo gerenciador de armazenamento
    pub fn new(flash: &'a mut F, key_provider: &'a K, rng: &'a mut R) -> Self {
        Self {
            flash,
            key_provider,
            rng,
            slots: [None, None, None, None, None, None, None, None],
            sequence: 0,
            _phantom: PhantomData,
        }
    }

    /// Configura um slot de armazenamento
    pub fn configure_slot(
        &mut self,
        index: usize,
        offset: u32,
        page_count: usize,
        data_type: u8,
    ) -> Result<(), StorageError> {
        if index >= self.slots.len() {
            return Err(StorageError::Unsupported);
        }
        if !offset.is_multiple_of(PAGE_SIZE as u32) {
            return Err(StorageError::Unsupported);
        }
        let slot = StorageSlot {
            offset,
            page_count,
            data_type,
        };
        // Retoma o contador de sequência do maior valor já gravado na flash,
        // garantindo que novas escritas sejam monotônicas após reboot.
        self.sequence = self.sequence.max(self.scan_max_sequence(&slot));
        self.slots[index] = Some(slot);
        Ok(())
    }

    /// Calcula CRC-16 CCITT (polinômio 0x1021)
    fn crc16(data: &[u8]) -> u16 {
        let mut crc: u16 = 0xFFFF;
        for &byte in data {
            crc ^= (byte as u16) << 8;
            for _ in 0..8 {
                if (crc & 0x8000) != 0 {
                    crc = (crc << 1) ^ 0x1021;
                } else {
                    crc <<= 1;
                }
            }
        }
        crc
    }

    /// Escreve dados criptografados em um slot
    pub fn write_encrypted(&mut self, slot_index: usize, data: &[u8]) -> Result<(), StorageError> {
        let slot = self.slots[slot_index].ok_or(StorageError::PageNotFound)?;

        if data.len() > MAX_ENCRYPTED_DATA_SIZE {
            return Err(StorageError::BufferTooSmall);
        }

        // Obter chave
        let mut key = [0u8; KEY_SIZE];
        self.key_provider
            .fill_key(&mut key)
            .map_err(|_| StorageError::KeyUnavailable)?;

        // Gerar nonce
        let mut nonce = [0u8; NONCE_SIZE];
        self.rng.fill_bytes(&mut nonce)?;

        // Preparar buffer de página
        let mut page_buf = [0u8; PAGE_SIZE];

        // Copiar dados para o buffer
        page_buf[PAGE_HEADER_SIZE..PAGE_HEADER_SIZE + data.len()].copy_from_slice(data);

        // Calcular CRC do dado original (plaintext) antes da criptografia
        let crc = Self::crc16(data);

        // Construir cabeçalho com CRC do plaintext
        self.sequence = self.sequence.wrapping_add(1);
        let mut header = PageHeader {
            state: PageState::Active,
            sequence: self.sequence,
            data_len: data.len() as u16,
            crc,
            data_type: slot.data_type,
            flags: 0,
        };

        // Serializar header no início do buffer ANTES da criptografia
        // para que o AAD (Additional Authenticated Data) seja consistente
        // entre escrita e leitura.
        let header_bytes = header.serialize();
        page_buf[..PAGE_HEADER_SIZE].copy_from_slice(&header_bytes);

        // Criptografar dados in-place com AES-256-GCM
        // O AAD é o header serializado (PAGE_HEADER_SIZE bytes no início do buffer)
        let (aad, payload) = page_buf.split_at_mut(PAGE_HEADER_SIZE);
        let payload = &mut payload[..data.len()];

        let tag = openkey_crypto::encrypt_config(&key, &nonce, aad, payload)
            .map_err(|_| StorageError::AuthenticationFailed)?;

        // Escrever nonce após o header (em área reservada)
        let nonce_offset = PAGE_HEADER_SIZE + data.len();
        if nonce_offset + NONCE_SIZE <= PAGE_SIZE {
            page_buf[nonce_offset..nonce_offset + NONCE_SIZE].copy_from_slice(&nonce);
        }

        // Escrever tag após nonce
        let tag_offset = nonce_offset + NONCE_SIZE;
        if tag_offset + TAG_SIZE <= PAGE_SIZE {
            page_buf[tag_offset..tag_offset + TAG_SIZE].copy_from_slice(&tag);
        }

        // Encontrar página vazia ou mais antiga no slot
        let page_idx = self.find_writable_page(&slot)?;
        let page_offset = slot.offset + (page_idx as u32) * PAGE_SIZE as u32;

        // Apagar página antes de escrever
        self.flash.erase(page_offset, PAGE_SIZE as u32)?;

        // Escrever header com estado Writing (power-loss recovery)
        let mut write_header = header;
        write_header.state = PageState::Writing;
        let write_header_bytes = write_header.serialize();
        self.flash.write(page_offset, &write_header_bytes)?;

        // Escrever dados (header já serializado no page_buf)
        self.flash.write(page_offset, &page_buf)?;

        // Atualizar estado para Active (commit final)
        header.state = PageState::Active;
        let final_header_bytes = header.serialize();
        self.flash.write(page_offset, &final_header_bytes)?;

        Ok(())
    }

    /// Recupera de falha de energia durante escrita
    ///
    /// Varre todas as páginas dos slots configurados e:
    /// - Marca páginas em estado `Writing` como `Corrupted`
    /// - Marca páginas `Active` cuja assinatura falha como `Corrupted`
    /// - Limpa páginas óbvias de escrita interrompida
    pub fn recover_power_loss(&mut self) -> Result<(), StorageError> {
        // Coleta informações dos slots antes de mutar
        let slots: [(u32, usize, u8); 8] = core::array::from_fn(|i| {
            self.slots[i]
                .map(|s| (s.offset, s.page_count, s.data_type))
                .unwrap_or((0, 0, 0))
        });

        let mut max_seq = 0u32;

        for (offset, page_count, _data_type) in slots.iter() {
            for i in 0..*page_count {
                let page_offset = offset + (i as u32) * PAGE_SIZE as u32;
                let mut header_buf = [0u8; PAGE_HEADER_SIZE];
                if self.flash.read(page_offset, &mut header_buf).is_err() {
                    continue;
                }

                let mut header = match PageHeader::deserialize(&header_buf) {
                    Ok(h) => h,
                    Err(_) => continue,
                };

                if matches!(
                    header.state,
                    PageState::Active | PageState::Writing | PageState::Obsolete
                ) && header.sequence > max_seq
                {
                    max_seq = header.sequence;
                }

                match header.state {
                    PageState::Writing => {
                        // Escrita interrompida — marcar como corrompida
                        header.state = PageState::Corrupted;
                        let header_bytes = header.serialize();
                        let _ = self.flash.write(page_offset, &header_bytes);
                    }
                    PageState::Active => {
                        // Verifica integridade da página ativa
                        let is_valid = self.validate_page_integrity(page_offset, &header)?;
                        if !is_valid {
                            header.state = PageState::Corrupted;
                            let header_bytes = header.serialize();
                            let _ = self.flash.write(page_offset, &header_bytes);
                        }
                    }
                    _ => {}
                }
            }
        }

        // Retoma o contador de sequência do maior valor encontrado na flash
        self.sequence = self.sequence.max(max_seq);

        Ok(())
    }

    /// Valida a integridade de uma página ativa (CRC + AEAD)
    fn validate_page_integrity(
        &mut self,
        page_offset: u32,
        header: &PageHeader,
    ) -> Result<bool, StorageError> {
        let data_len = header.data_len as usize;
        if data_len > MAX_ENCRYPTED_DATA_SIZE {
            return Ok(false);
        }

        let mut page_buf = [0u8; PAGE_SIZE];
        if self.flash.read(page_offset, &mut page_buf).is_err() {
            return Ok(false);
        }

        // Obter chave
        let mut key = [0u8; KEY_SIZE];
        if self.key_provider.fill_key(&mut key).is_err() {
            return Ok(false);
        }

        // Extrair nonce
        let nonce_offset = PAGE_HEADER_SIZE + data_len;
        let mut nonce = [0u8; NONCE_SIZE];
        // Com MAX_ENCRYPTED_DATA_SIZE, nonce sempre cabe
        nonce.copy_from_slice(&page_buf[nonce_offset..nonce_offset + NONCE_SIZE]);

        // Extrair tag
        let tag_offset = nonce_offset + NONCE_SIZE;
        let mut tag = [0u8; TAG_SIZE];
        if tag_offset + TAG_SIZE <= PAGE_SIZE {
            tag.copy_from_slice(&page_buf[tag_offset..tag_offset + TAG_SIZE]);
        }

        // Tentar decriptar
        let (aad, payload_and_rest) = page_buf.split_at_mut(PAGE_HEADER_SIZE);
        let payload = &mut payload_and_rest[..data_len];

        if openkey_crypto::decrypt_config(&key, &nonce, aad, payload, &tag).is_err() {
            return Ok(false);
        }

        // Verificar CRC
        let crc = Self::crc16(payload);
        if crc != header.crc {
            return Ok(false);
        }

        Ok(true)
    }

    /// Lê e decripta dados de um slot
    pub fn read_encrypted(
        &mut self,
        slot_index: usize,
        output: &mut [u8],
    ) -> Result<usize, StorageError> {
        let slot = self.slots[slot_index].ok_or(StorageError::PageNotFound)?;

        // Encontrar página ativa mais recente
        let (page_idx, _header) = self.find_latest_active_page(&slot)?;
        let page_offset = slot.offset + (page_idx as u32) * PAGE_SIZE as u32;

        // Ler página
        let mut page_buf = [0u8; PAGE_SIZE];
        self.flash.read(page_offset, &mut page_buf)?;

        // Deserializar header
        let header = PageHeader::deserialize(&page_buf)?;
        if header.state != PageState::Active {
            return Err(StorageError::CorruptedData);
        }

        let data_len = header.data_len as usize;
        // data_len > MAX_ENCRYPTED_DATA_SIZE indica página corrompida
        // (o nonce/tag não cabem), tratada como erro de integridade
        if data_len > MAX_ENCRYPTED_DATA_SIZE {
            return Err(StorageError::CorruptedData);
        }
        if data_len > output.len() {
            return Err(StorageError::BufferTooSmall);
        }

        // Obter chave
        let mut key = [0u8; KEY_SIZE];
        self.key_provider
            .fill_key(&mut key)
            .map_err(|_| StorageError::KeyUnavailable)?;

        // Extrair nonce
        let nonce_offset = PAGE_HEADER_SIZE + data_len;
        let mut nonce = [0u8; NONCE_SIZE];
        // Com MAX_ENCRYPTED_DATA_SIZE, nonce sempre cabe
        nonce.copy_from_slice(&page_buf[nonce_offset..nonce_offset + NONCE_SIZE]);

        // Extrair tag
        let tag_offset = nonce_offset + NONCE_SIZE;
        let mut tag = [0u8; TAG_SIZE];
        if tag_offset + TAG_SIZE <= PAGE_SIZE {
            tag.copy_from_slice(&page_buf[tag_offset..tag_offset + TAG_SIZE]);
        }

        // Decriptografar dados in-place
        let (aad, payload_and_rest) = page_buf.split_at_mut(PAGE_HEADER_SIZE);
        let payload = &mut payload_and_rest[..data_len];

        openkey_crypto::decrypt_config(&key, &nonce, aad, payload, &tag)
            .map_err(|_| StorageError::AuthenticationFailed)?;

        // Verificar CRC do dado decriptografado (plaintext)
        let crc = Self::crc16(payload);
        if crc != header.crc {
            return Err(StorageError::CorruptedData);
        }

        // Copiar dados decriptografados para saída
        output[..data_len].copy_from_slice(payload);

        Ok(data_len)
    }

    /// Encontra a próxima página gravável no slot (wear-leveling circular)
    fn find_writable_page(&mut self, slot: &StorageSlot) -> Result<usize, StorageError> {
        let mut oldest_idx = 0;
        let mut oldest_seq = u32::MAX;
        let mut empty_idx = None;

        for i in 0..slot.page_count {
            let page_offset = slot.offset + (i as u32) * PAGE_SIZE as u32;
            let mut header_buf = [0u8; PAGE_HEADER_SIZE];
            match self.flash.read(page_offset, &mut header_buf) {
                Ok(()) => {}
                Err(_) => continue,
            }

            let header = match PageHeader::deserialize(&header_buf) {
                Ok(h) => h,
                Err(_) => continue,
            };

            match header.state {
                PageState::Empty => {
                    if empty_idx.is_none() {
                        empty_idx = Some(i);
                    }
                }
                PageState::Active => {
                    if header.sequence < oldest_seq {
                        oldest_seq = header.sequence;
                        oldest_idx = i;
                    }
                }
                PageState::Writing | PageState::Obsolete | PageState::Corrupted => {
                    if empty_idx.is_none() {
                        empty_idx = Some(i);
                    }
                }
            }
        }

        // Se encontrou página vazia, use-a
        if let Some(idx) = empty_idx {
            return Ok(idx);
        }

        // Slot cheio: reutilizar a página mais antiga (menor sequence),
        // preservando os dados mais recentes até o commit final
        Ok(oldest_idx)
    }

    /// Escaneia as páginas de um slot e retorna a maior sequência encontrada
    fn scan_max_sequence(&mut self, slot: &StorageSlot) -> u32 {
        let mut max_seq = 0u32;
        for i in 0..slot.page_count {
            let page_offset = slot.offset + (i as u32) * PAGE_SIZE as u32;
            let mut header_buf = [0u8; PAGE_HEADER_SIZE];
            if self.flash.read(page_offset, &mut header_buf).is_err() {
                continue;
            }

            let header = match PageHeader::deserialize(&header_buf) {
                Ok(h) => h,
                Err(_) => continue,
            };

            if matches!(
                header.state,
                PageState::Active | PageState::Writing | PageState::Obsolete
            ) && header.sequence > max_seq
            {
                max_seq = header.sequence;
            }
        }
        max_seq
    }

    /// Encontra a página ativa mais recente no slot
    fn find_latest_active_page(
        &mut self,
        slot: &StorageSlot,
    ) -> Result<(usize, PageHeader), StorageError> {
        let mut latest_idx = 0;
        let mut latest_header = PageHeader::empty();
        let mut found = false;

        for i in 0..slot.page_count {
            let page_offset = slot.offset + (i as u32) * PAGE_SIZE as u32;
            let mut header_buf = [0u8; PAGE_HEADER_SIZE];
            match self.flash.read(page_offset, &mut header_buf) {
                Ok(()) => {}
                Err(_) => continue,
            }

            let header = match PageHeader::deserialize(&header_buf) {
                Ok(h) => h,
                Err(_) => continue,
            };

            if header.state == PageState::Active && header.sequence >= latest_header.sequence {
                latest_header = header;
                latest_idx = i;
                found = true;
            }
        }

        if !found {
            return Err(StorageError::PageNotFound);
        }

        Ok((latest_idx, latest_header))
    }

    /// Marca todas as páginas de um slot como obsoletas (para wear-leveling)
    pub fn invalidate_slot(&mut self, slot_index: usize) -> Result<(), StorageError> {
        let slot = self.slots[slot_index].ok_or(StorageError::PageNotFound)?;

        for i in 0..slot.page_count {
            let page_offset = slot.offset + (i as u32) * PAGE_SIZE as u32;
            let mut header_buf = [0u8; PAGE_HEADER_SIZE];
            if self.flash.read(page_offset, &mut header_buf).is_err() {
                continue;
            }

            let mut header = match PageHeader::deserialize(&header_buf) {
                Ok(h) => h,
                Err(_) => continue,
            };

            if header.state == PageState::Active {
                header.state = PageState::Obsolete;
                let header_bytes = header.serialize();
                self.flash.write(page_offset, &header_bytes)?;
            }
        }

        Ok(())
    }

    /// Retorna o uso do slot (páginas ativas / total)
    pub fn slot_usage(&mut self, slot_index: usize) -> Result<(usize, usize), StorageError> {
        let slot = self.slots[slot_index].ok_or(StorageError::PageNotFound)?;

        let mut active = 0;
        for i in 0..slot.page_count {
            let page_offset = slot.offset + (i as u32) * PAGE_SIZE as u32;
            let mut header_buf = [0u8; PAGE_HEADER_SIZE];
            if self.flash.read(page_offset, &mut header_buf).is_err() {
                continue;
            }

            if let Ok(header) = PageHeader::deserialize(&header_buf) {
                if header.state == PageState::Active {
                    active += 1;
                }
            }
        }

        Ok((active, slot.page_count))
    }
}

/// Versão do módulo de armazenamento
pub const STORAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    /// Simulador de flash para testes
    struct MockFlash {
        storage: [u8; 16 * PAGE_SIZE],
    }

    impl MockFlash {
        const fn new() -> Self {
            Self {
                storage: [0xFF; 16 * PAGE_SIZE],
            }
        }
    }

    impl FlashStorageProvider for MockFlash {
        fn read(&mut self, offset: u32, buf: &mut [u8]) -> Result<(), FlashError> {
            let start = offset as usize;
            let end = start + buf.len();
            if end > self.storage.len() {
                return Err(FlashError::OutOfBounds);
            }
            buf.copy_from_slice(&self.storage[start..end]);
            Ok(())
        }

        fn write(&mut self, offset: u32, data: &[u8]) -> Result<(), FlashError> {
            let start = offset as usize;
            let end = start + data.len();
            if end > self.storage.len() {
                return Err(FlashError::OutOfBounds);
            }
            self.storage[start..end].copy_from_slice(data);
            Ok(())
        }

        fn erase(&mut self, offset: u32, len: u32) -> Result<(), FlashError> {
            let start = offset as usize;
            let end = start + len as usize;
            if end > self.storage.len() {
                return Err(FlashError::OutOfBounds);
            }
            for byte in &mut self.storage[start..end] {
                *byte = 0xFF;
            }
            Ok(())
        }

        fn total_size(&self) -> u32 {
            self.storage.len() as u32
        }
    }

    /// Provedor de chave mock para testes
    struct MockKeyProvider;

    impl StorageKeyProvider for MockKeyProvider {
        fn fill_key(&self, dest: &mut [u8; KEY_SIZE]) -> Result<(), StorageError> {
            dest.fill(0x42);
            Ok(())
        }
    }

    /// Provedor de RNG mock para testes
    struct MockRng;

    impl StorageRngProvider for MockRng {
        fn fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), StorageError> {
            for (i, b) in dest.iter_mut().enumerate() {
                *b = (i % 256) as u8;
            }
            Ok(())
        }
    }

    #[test]
    fn test_crc16() {
        let data = b"Hello, OpenKey!";
        let crc = StorageManager::<MockFlash, MockKeyProvider, MockRng>::crc16(data);
        assert_ne!(crc, 0);
    }

    #[test]
    fn test_page_header_serialize_deserialize() {
        let header = PageHeader::new(0x01, 128, 42);
        let bytes = header.serialize();
        let deserialized = PageHeader::deserialize(&bytes).unwrap();
        assert_eq!(header.state, deserialized.state);
        assert_eq!(header.sequence, deserialized.sequence);
        assert_eq!(header.data_len, deserialized.data_len);
        assert_eq!(header.data_type, deserialized.data_type);
    }

    #[test]
    fn test_storage_write_read() {
        let mut flash = MockFlash::new();
        let key_provider = MockKeyProvider;
        let mut rng = MockRng;

        let mut storage = StorageManager::new(&mut flash, &key_provider, &mut rng);
        storage.configure_slot(0, 0, 4, 0x01).unwrap();

        let test_data = b"Test credential data";
        storage.write_encrypted(0, test_data).unwrap();

        let mut output = [0u8; 64];
        let read_len = storage.read_encrypted(0, &mut output).unwrap();
        assert_eq!(read_len, test_data.len());
        assert_eq!(&output[..read_len], test_data);
    }

    #[test]
    fn test_storage_update() {
        let mut flash = MockFlash::new();
        let key_provider = MockKeyProvider;
        let mut rng = MockRng;

        let mut storage = StorageManager::new(&mut flash, &key_provider, &mut rng);
        storage.configure_slot(0, 0, 4, 0x01).unwrap();

        // Primeira escrita
        let data1 = b"First version";
        storage.write_encrypted(0, data1).unwrap();

        // Segunda escrita (atualização)
        let data2 = b"Second version - updated";
        storage.write_encrypted(0, data2).unwrap();

        // Ler deve retornar a versão mais recente
        let mut output = [0u8; 64];
        let read_len = storage.read_encrypted(0, &mut output).unwrap();
        assert_eq!(read_len, data2.len());
        assert_eq!(&output[..read_len], data2);
    }

    #[test]
    fn test_storage_slot_usage() {
        let mut flash = MockFlash::new();
        let key_provider = MockKeyProvider;
        let mut rng = MockRng;

        let mut storage = StorageManager::new(&mut flash, &key_provider, &mut rng);
        storage.configure_slot(0, 0, 4, 0x01).unwrap();

        let (active, total) = storage.slot_usage(0).unwrap();
        assert_eq!(active, 0);
        assert_eq!(total, 4);

        storage.write_encrypted(0, b"test").unwrap();

        let (active, total) = storage.slot_usage(0).unwrap();
        assert_eq!(active, 1);
        assert_eq!(total, 4);
    }

    #[test]
    fn test_power_loss_recovery_marks_writing_as_corrupted() {
        let mut flash = MockFlash::new();
        let key_provider = MockKeyProvider;
        let mut rng = MockRng;

        // Simula uma escrita interrompida: escreve um header com estado Writing
        let mut header = PageHeader::new(0x01, 10, 1);
        header.state = PageState::Writing;
        let header_bytes = header.serialize();
        flash.write(0, &header_bytes).unwrap();

        // Recuperação deve marcar a página como corrompida
        {
            let mut storage = StorageManager::new(&mut flash, &key_provider, &mut rng);
            storage.configure_slot(0, 0, 4, 0x01).unwrap();
            storage.recover_power_loss().unwrap();
        }

        // Verifica que a página foi marcada como Corrupted
        let mut recovered_header_buf = [0u8; PAGE_HEADER_SIZE];
        flash.read(0, &mut recovered_header_buf).unwrap();
        let recovered_header = PageHeader::deserialize(&recovered_header_buf).unwrap();
        assert_eq!(recovered_header.state, PageState::Corrupted);
    }

    #[test]
    fn test_power_loss_recovery_preserves_valid_pages() {
        let mut flash = MockFlash::new();
        let key_provider = MockKeyProvider;
        let mut rng = MockRng;

        // Escreve dados válidos
        let test_data = b"Valid data for power loss test";
        {
            let mut storage = StorageManager::new(&mut flash, &key_provider, &mut rng);
            storage.configure_slot(0, 0, 4, 0x01).unwrap();
            storage.write_encrypted(0, test_data).unwrap();
            // Recuperação não deve corromper a página válida
            storage.recover_power_loss().unwrap();
        }

        // Lê os dados — deve ser recuperado corretamente
        let mut output = [0u8; 64];
        let mut storage = StorageManager::new(&mut flash, &key_provider, &mut rng);
        storage.configure_slot(0, 0, 4, 0x01).unwrap();
        let read_len = storage.read_encrypted(0, &mut output).unwrap();
        assert_eq!(read_len, test_data.len());
        assert_eq!(&output[..read_len], test_data);
    }

    #[test]
    fn test_integrity_validation_detects_corruption() {
        let mut flash = MockFlash::new();
        let key_provider = MockKeyProvider;
        let mut rng = MockRng;

        // Escreve dados válidos
        let test_data = b"Integrity test data";
        {
            let mut storage = StorageManager::new(&mut flash, &key_provider, &mut rng);
            storage.configure_slot(0, 0, 4, 0x01).unwrap();
            storage.write_encrypted(0, test_data).unwrap();
        }

        // Corrompe um byte de dados (fora do escopo do StorageManager)
        let page_offset = 0u32;
        let corrupt_offset = PAGE_HEADER_SIZE as u32 + 5; // dentro do payload
        let mut byte_buf = [0u8; 1];
        flash
            .read(page_offset + corrupt_offset, &mut byte_buf)
            .unwrap();
        flash
            .write(page_offset + corrupt_offset, &[byte_buf[0] ^ 0xFF])
            .unwrap();

        // Recuperação deve detectar a corrupção
        {
            let mut storage = StorageManager::new(&mut flash, &key_provider, &mut rng);
            storage.configure_slot(0, 0, 4, 0x01).unwrap();
            storage.recover_power_loss().unwrap();
        }

        // A página deve ser marcada como corrompida
        let mut header_buf = [0u8; PAGE_HEADER_SIZE];
        flash.read(page_offset, &mut header_buf).unwrap();
        let header = PageHeader::deserialize(&header_buf).unwrap();
        assert_eq!(header.state, PageState::Corrupted);

        // Leitura deve falhar
        let mut storage = StorageManager::new(&mut flash, &key_provider, &mut rng);
        storage.configure_slot(0, 0, 4, 0x01).unwrap();
        let mut output = [0u8; 64];
        assert!(storage.read_encrypted(0, &mut output).is_err());
    }

    #[test]
    fn test_wear_leveling_rewrites_oldest_page() {
        let mut flash = MockFlash::new();
        let key_provider = MockKeyProvider;
        let mut rng = MockRng;

        {
            let mut storage = StorageManager::new(&mut flash, &key_provider, &mut rng);
            storage.configure_slot(0, 0, 4, 0x01).unwrap();

            // Preenche o slot com 4 escritas (uma por página, seq 1..=4)
            for i in 0..4 {
                storage.write_encrypted(0, &[i as u8 + 1; 8]).unwrap();
            }

            // 5ª escrita com slot cheio: deve reescrever a página mais antiga (page 0)
            storage.write_encrypted(0, b"final").unwrap();
        }

        // A página 0 (antiga) deve agora conter a sequência mais recente (5)
        let mut header_buf = [0u8; PAGE_HEADER_SIZE];
        flash.read(0, &mut header_buf).unwrap();
        let header = PageHeader::deserialize(&header_buf).unwrap();
        assert_eq!(header.state, PageState::Active);
        assert_eq!(header.sequence, 5);

        // A página mais recente anterior (page 3, seq 4) deve estar preservada
        let mut header_buf = [0u8; PAGE_HEADER_SIZE];
        flash.read(3 * PAGE_SIZE as u32, &mut header_buf).unwrap();
        let header = PageHeader::deserialize(&header_buf).unwrap();
        assert_eq!(header.state, PageState::Active);
        assert_eq!(header.sequence, 4);

        // A leitura deve retornar o dado mais recente
        let mut storage = StorageManager::new(&mut flash, &key_provider, &mut rng);
        storage.configure_slot(0, 0, 4, 0x01).unwrap();
        let mut output = [0u8; 64];
        let read_len = storage.read_encrypted(0, &mut output).unwrap();
        assert_eq!(read_len, b"final".len());
        assert_eq!(&output[..read_len], b"final");
    }

    #[test]
    fn test_sequence_continues_after_reboot() {
        let mut flash = MockFlash::new();
        let key_provider = MockKeyProvider;
        let mut rng = MockRng;

        // Primeira sessão: escreve 3 versões (seq 1, 2, 3)
        {
            let mut storage = StorageManager::new(&mut flash, &key_provider, &mut rng);
            storage.configure_slot(0, 0, 4, 0x01).unwrap();
            storage.write_encrypted(0, b"v1").unwrap();
            storage.write_encrypted(0, b"v2").unwrap();
            storage.write_encrypted(0, b"v3").unwrap();
        }

        // "Reboot": nova instância do gerenciador sobre a mesma flash
        {
            let mut storage = StorageManager::new(&mut flash, &key_provider, &mut rng);
            storage.configure_slot(0, 0, 4, 0x01).unwrap();
            // A sequência deve continuar do máximo encontrado na flash
            storage.write_encrypted(0, b"v4").unwrap();

            // A leitura deve retornar v4 (com o bug, v4 recebia seq 1 e era perdida)
            let mut output = [0u8; 64];
            let read_len = storage.read_encrypted(0, &mut output).unwrap();
            assert_eq!(read_len, b"v4".len());
            assert_eq!(&output[..read_len], b"v4");
        }
    }

    #[test]
    fn test_oversized_data_len_no_panic_and_marked_corrupted() {
        let mut flash = MockFlash::new();
        let key_provider = MockKeyProvider;
        let mut rng = MockRng;

        // data_len no intervalo perigoso: > MAX_ENCRYPTED_DATA_SIZE e <= PAGE_DATA_SIZE,
        // o que estourava o array de página ao extrair o nonce
        let bad_len = (MAX_ENCRYPTED_DATA_SIZE + 20) as u16;
        assert!(bad_len as usize <= PAGE_DATA_SIZE);
        let header_bytes = PageHeader::new(0x01, bad_len, 1).serialize();
        flash.write(0, &header_bytes).unwrap();

        // Leitura não deve entrar em panic; trata como corrompida
        {
            let mut storage = StorageManager::new(&mut flash, &key_provider, &mut rng);
            storage.configure_slot(0, 0, 4, 0x01).unwrap();
            let mut output = [0u8; 64];
            assert!(storage.read_encrypted(0, &mut output).is_err());
        }

        // Recuperação não deve entrar em panic; marca a página como corrompida
        {
            let mut storage = StorageManager::new(&mut flash, &key_provider, &mut rng);
            storage.configure_slot(0, 0, 4, 0x01).unwrap();
            storage.recover_power_loss().unwrap();
            let mut header_buf = [0u8; PAGE_HEADER_SIZE];
            flash.read(0, &mut header_buf).unwrap();
            let header = PageHeader::deserialize(&header_buf).unwrap();
            assert_eq!(header.state, PageState::Corrupted);
        }
    }

    #[test]
    fn test_writing_state_in_serialization() {
        let header = PageHeader {
            state: PageState::Writing,
            sequence: 42,
            data_len: 128,
            crc: 0xABCD,
            data_type: 0x01,
            flags: 0,
        };
        let bytes = header.serialize();
        let deserialized = PageHeader::deserialize(&bytes).unwrap();
        assert_eq!(header.state, deserialized.state);
        assert_eq!(deserialized.state, PageState::Writing);
    }
}
