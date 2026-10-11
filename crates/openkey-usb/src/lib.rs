#![no_std]

/// Transporte bidirecional independente da implementação física.
pub trait Transport {
    type Error;

    fn receive(&mut self, packet: &mut [u8]) -> Result<usize, Self::Error>;
    fn send(&mut self, packet: &[u8]) -> Result<(), Self::Error>;
}

/// Alias semântico para transportes que usam enquadramento USB HID.
pub trait UsbTransport: Transport {}
