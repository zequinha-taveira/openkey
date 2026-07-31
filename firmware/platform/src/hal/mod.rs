//! HAL Module - Hardware Abstraction Layer traits
//!
//! O HAL pertence ao microcontrolador. Implementa apenas abstrações de baixo nível.
//! Nunca coloque lógica específica da placa no HAL.

pub mod error;
pub mod flash;
pub mod gpio;
pub mod i2c;
pub mod otp;
pub mod rng;
pub mod spi;
pub mod timer;
pub mod uart;
pub mod usb;
pub mod watchdog;

pub use error::HalError;
pub use flash::{FlashError, FlashStorageProvider, FLASH_PAGE_SIZE};
pub use gpio::{GpioDirection, GpioLevel, GpioProvider};
pub use i2c::I2cProvider;
pub use otp::{OtpError, OtpProvider};
pub use rng::{HealthTestResult, RngHealthCheck, RngProvider};
pub use spi::{SpiBitOrder, SpiMode, SpiProvider};
pub use timer::TimerProvider;
pub use uart::UartProvider;
pub use usb::{UsbDeviceProvider, UsbTransportProvider, USB_HID_PACKET_SIZE};
pub use watchdog::WatchdogProvider;
