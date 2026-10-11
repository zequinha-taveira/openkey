#![no_std]

/// Contrato de transporte independente da implementação USB.
pub trait UsbTransport {
    type Error;
    fn write(&mut self, packet: &[u8]) -> Result<(), Self::Error>;
}
