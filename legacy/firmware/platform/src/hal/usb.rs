//! HAL USB traits
//!
//! Abstração de USB para transporte de dados HID e outros protocolos.

use super::error::HalError;

/// Tamanho máximo de um pacote USB HID padrão (64 bytes)
pub const USB_HID_PACKET_SIZE: usize = 64;

/// Provedor de transporte USB HID
pub trait UsbTransportProvider {
    /// Envia um pacote de dados via USB HID
    fn send_packet(&mut self, packet: &[u8]) -> Result<(), HalError>;
    /// Tenta receber um pacote de dados via USB HID
    fn receive_packet(&mut self, buf: &mut [u8]) -> Result<usize, HalError>;
    /// Verifica se o dispositivo USB está conectado
    fn is_connected(&self) -> bool;
}

/// Provedor de USB para configuração e controle
pub trait UsbDeviceProvider {
    /// Inicializa o dispositivo USB
    fn init(&mut self) -> Result<(), HalError>;
    /// Processa eventos USB (chamado no loop principal)
    fn poll(&mut self);
}
