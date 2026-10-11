//! OpenKey RP2350 Target Firmware (`no_std`)
//!
//! Firmware de referência para o microcontrolador RP2350 (ARM Cortex-M33).
//!
//! Arquitetura:
//! ```text
//! OpenKey Core
//!         │
//! Platform Services
//!         │
//! Configuration Manager
//!         │
//! Board Profile (RP2350)
//!         │
//! Device Profile (RP2350)
//!         │
//! HAL (RP2350)
//!         │
//! Startup
//! ```

#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
use openkey_core::core_info;
use openkey_platform::board::{
    BoardProfile, BoardProfileId, ButtonConfig, FlashConfig, LedConfig, OptionalFeatures, UsbConfig,
};
use openkey_platform::device::{DeviceProfile, DeviceText, UsbIdentity};
#[cfg(not(test))]
use openkey_platform::hal::{
    FlashError, FlashStorageProvider, GpioDirection, GpioLevel, GpioProvider, HalError,
    RngProvider, TimerProvider, UsbTransportProvider, WatchdogProvider,
};
#[cfg(not(test))]
use openkey_platform::{HardwareProviders, PlatformServices};

/// Board Profile para a placa RP2350 de referência
pub const BOARD_PROFILE: BoardProfile = BoardProfile {
    id: BoardProfileId(*b"openkey-rp23xx01"),
    manufacturer: "OpenKey",
    model: "RP2350-REF",
    revision: "1.0",
    flash: FlashConfig {
        total_size: 4 * 1024 * 1024,
        page_size: 4096,
        sector_size: 4096,
    },
    usb: UsbConfig {
        vid: 0x16C0,
        pid: 0x27DB,
        bcd_version: 0x0200,
        max_packet_size: 64,
    },
    led: Some(LedConfig {
        pin: openkey_platform::board::GpioPin { port: 0, pin: 25 },
        active_high: true,
    }),
    button: Some(ButtonConfig {
        pin: openkey_platform::board::GpioPin { port: 0, pin: 24 },
        active_low: true,
        pull_up: true,
    }),
    features: OptionalFeatures {
        has_nfc: false,
        has_ble: false,
        has_secure_element: false,
        has_tamper_detect: false,
    },
};

/// Device Profile para a unidade RP2350 de referência
pub const DEVICE_PROFILE: DeviceProfile = DeviceProfile {
    serial_number: DeviceText::from_static("RP2350-00000001"),
    usb_identity: UsbIdentity {
        vid: 0x16C0,
        pid: 0x27DB,
        serial_number: DeviceText::from_static("RP2350-00000001"),
        product_name: DeviceText::from_static("OpenKey Security Key"),
        manufacturer_name: DeviceText::from_static("OpenKey"),
    },
    calibration: None,
    manufacturing: None,
};

/// HAL de Flash para RP2350 (stub - implementação real usa XIP)
#[cfg(not(test))]
struct Rp2350Flash;

#[cfg(not(test))]
impl FlashStorageProvider for Rp2350Flash {
    fn read(&mut self, _offset: u32, _buf: &mut [u8]) -> Result<(), FlashError> {
        Err(FlashError::HardwareFailure)
    }

    fn write(&mut self, _offset: u32, _data: &[u8]) -> Result<(), FlashError> {
        Err(FlashError::HardwareFailure)
    }

    fn erase(&mut self, _offset: u32, _len: u32) -> Result<(), FlashError> {
        Err(FlashError::HardwareFailure)
    }

    fn total_size(&self) -> u32 {
        4 * 1024 * 1024
    }
}

/// HAL de RNG para RP2350 (stub - implementação real usa TRNG de hardware)
#[cfg(not(test))]
struct Rp2350Rng;

#[cfg(not(test))]
impl RngProvider for Rp2350Rng {
    fn fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), HalError> {
        Err(HalError::HardwareFailure)
    }

    fn is_healthy(&self) -> bool {
        false
    }
}

/// HAL de USB para RP2350 (stub - implementação real usa TinyUSB)
#[cfg(not(test))]
struct Rp2350Usb;

#[cfg(not(test))]
impl UsbTransportProvider for Rp2350Usb {
    fn send_packet(&mut self, _packet: &[u8]) -> Result<(), HalError> {
        Err(HalError::HardwareFailure)
    }

    fn receive_packet(&mut self, _buf: &mut [u8]) -> Result<usize, HalError> {
        Ok(0)
    }

    fn is_connected(&self) -> bool {
        false
    }
}

/// HAL de GPIO para RP2350 (stub)
#[cfg(not(test))]
struct Rp2350Gpio;

#[cfg(not(test))]
impl GpioProvider for Rp2350Gpio {
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

/// HAL de Timer para RP2350 (stub)
#[cfg(not(test))]
struct Rp2350Timer;

#[cfg(not(test))]
impl TimerProvider for Rp2350Timer {
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

/// HAL de Watchdog para RP2350 (stub)
#[cfg(not(test))]
struct Rp2350Watchdog;

#[cfg(not(test))]
impl WatchdogProvider for Rp2350Watchdog {
    fn init(&mut self, _timeout_ms: u32) -> Result<(), HalError> {
        Ok(())
    }

    fn feed(&mut self) {}

    fn disable(&mut self) {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

/// Ponto de entrada do firmware RP2350
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut flash = Rp2350Flash;
    let mut rng = Rp2350Rng;
    let mut usb = Rp2350Usb;
    let mut gpio = Rp2350Gpio;
    let mut timer = Rp2350Timer;
    let mut watchdog = Rp2350Watchdog;

    let hw = HardwareProviders {
        rng: &mut rng,
        flash: &mut flash,
        usb: &mut usb,
        gpio: &mut gpio,
        timer: &mut timer,
        watchdog: &mut watchdog,
    };

    let mut platform = PlatformServices::new(hw);

    let _ = core_info();

    loop {
        platform.feed_watchdog();
    }
}
