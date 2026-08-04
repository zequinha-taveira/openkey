//! OpenKey Host Simulator
//!
//! Simulador de software que implementa os HAL traits do OpenKey Platform
//! para execução em desktop (Linux, macOS, Windows).

use openkey_core::core_info;
use openkey_platform::hal::{
    FlashError, FlashStorageProvider, GpioDirection, GpioLevel, GpioProvider, HalError,
    RngProvider, TimerProvider, UsbTransportProvider, WatchdogProvider,
};

/// Simulador dummy de aleatoriedade em memória para host
struct DummyRng;

impl RngProvider for DummyRng {
    fn fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), HalError> {
        for (i, b) in dest.iter_mut().enumerate() {
            *b = (i % 256) as u8;
        }
        Ok(())
    }

    fn is_healthy(&self) -> bool {
        true
    }
}

/// Simulador de armazenamento em memória para host
struct DummyFlash {
    storage: [u8; 4096],
}

impl DummyFlash {
    const fn new() -> Self {
        Self {
            storage: [0u8; 4096],
        }
    }
}

impl FlashStorageProvider for DummyFlash {
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

/// Simulador de GPIO para host
struct DummyGpio;

impl GpioProvider for DummyGpio {
    fn set_direction(&mut self, _pin: u8, _direction: GpioDirection) -> Result<(), HalError> {
        Ok(())
    }

    fn read_pin(&mut self, _pin: u8) -> Result<GpioLevel, HalError> {
        Ok(GpioLevel::Low)
    }

    fn write_pin(&mut self, _pin: u8, _level: GpioLevel) -> Result<(), HalError> {
        Ok(())
    }

    fn toggle_pin(&mut self, _pin: u8) -> Result<(), HalError> {
        Ok(())
    }
}

/// Simulador de USB para host
struct DummyUsb;

impl UsbTransportProvider for DummyUsb {
    fn send_packet(&mut self, _packet: &[u8]) -> Result<(), HalError> {
        Ok(())
    }

    fn receive_packet(&mut self, _buf: &mut [u8]) -> Result<usize, HalError> {
        Ok(0)
    }

    fn is_connected(&self) -> bool {
        true
    }
}

/// Simulador de timer para host
struct DummyTimer;

impl TimerProvider for DummyTimer {
    fn millis(&self) -> u64 {
        0
    }

    fn micros(&self) -> u64 {
        0
    }

    fn nanos(&self) -> u128 {
        0
    }

    fn delay_ms(&mut self, _ms: u32) {}

    fn delay_us(&mut self, _us: u32) {}
}

/// Simulador de watchdog para host
struct DummyWatchdog;

impl WatchdogProvider for DummyWatchdog {
    fn init(&mut self, _timeout_ms: u32) -> Result<(), HalError> {
        Ok(())
    }

    fn feed(&mut self) {}

    fn disable(&mut self) {}
}

fn main() {
    println!("Iniciando OpenKey Software Simulator...");
    println!("Info: {}", core_info());

    let mut rng = DummyRng;
    let mut buf = [0u8; 8];
    if rng.fill_bytes(&mut buf).is_ok() {
        println!("TRNG Emulado: {:?}", buf);
    }
    println!("RNG Health: {}", rng.is_healthy());

    let mut flash = DummyFlash::new();
    let test_data = b"OpenKey Test";
    flash.write(0, test_data).unwrap();
    let mut read_buf = [0u8; 12];
    flash.read(0, &mut read_buf).unwrap();
    println!("Flash Test: {:?}", read_buf);

    let mut gpio = DummyGpio;
    gpio.set_direction(0, GpioDirection::Output).unwrap();
    gpio.write_pin(0, GpioLevel::High).unwrap();
    println!("GPIO Test: {:?}", gpio.read_pin(0).unwrap());

    let mut usb = DummyUsb;
    usb.send_packet(b"test").unwrap();
    println!("USB Connected: {}", usb.is_connected());

    let timer = DummyTimer;
    println!("Timer millis: {}", timer.millis());

    let mut watchdog = DummyWatchdog;
    watchdog.init(1000).unwrap();
    watchdog.feed();
    println!("Watchdog initialized and fed");

    println!("Simulador inicializado com sucesso!");
}
