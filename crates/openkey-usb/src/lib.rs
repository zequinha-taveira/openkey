#![no_std]

//! Contratos de transporte; implementações HID/USB ficam nos adaptadores de placa.

pub trait Transport {
    type Error;
    fn receive(&mut self, frame: &mut [u8]) -> Result<usize, Self::Error>;
    fn send(&mut self, frame: &[u8]) -> Result<(), Self::Error>;
}
