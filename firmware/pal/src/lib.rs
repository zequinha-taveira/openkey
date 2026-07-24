//! OpenKey Platform Abstraction Layer (PAL) Traits (`no_std`)
//!
//! Abstração de hardware e plataforma para o OpenKey Framework.

#![no_std]

/// Provedor de números aleatórios de entropia (TRNG / RNG de hardware)
pub trait RngProvider {
    /// Preenche o buffer fornecido com bytes aleatórios seguros
    fn fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), ()>;
}

/// Provedor de armazenamento não-volátil (Flash / NVRAM)
pub trait FlashStorageProvider {
    /// Lê dados da memória persistente no offset especificado
    fn read(&mut self, offset: u32, buf: &mut [u8]) -> Result<(), ()>;
    /// Escreve dados na memória persistente no offset especificado
    fn write(&mut self, offset: u32, data: &[u8]) -> Result<(), ()>;
}

/// Provedor de transporte de comunicação (USB HID, BLE, NFC)
pub trait UsbTransportProvider {
    /// Envia um pacote de dados via transporte de plataforma
    fn send_packet(&mut self, packet: &[u8]) -> Result<(), ()>;
    /// Tenta receber um pacote de dados do transporte de plataforma
    fn receive_packet(&mut self, buf: &mut [u8]) -> Result<usize, ()>;
}

/// Provedor de verificação de presença de usuário (Touch sensor, GPIO botão)
pub trait GpioUserPresenceProvider {
    /// Verifica se a presença física do usuário foi confirmada
    fn is_user_present(&mut self) -> bool;
}
