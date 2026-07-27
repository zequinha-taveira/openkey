# API do Firmware

## OpenKey Core

```rust
pub fn core_info() -> &'static str;
```

Retorna informações de versão do núcleo.

## Platform Services

```rust
pub struct HardwareProviders<'a> {
    pub rng: &'a mut dyn RngProvider,
    pub flash: &'a mut dyn FlashStorageProvider,
    pub usb: &'a mut dyn UsbTransportProvider,
    pub gpio: &'a mut dyn GpioProvider,
    pub timer: &'a mut dyn TimerProvider,
    pub watchdog: &'a mut dyn WatchdogProvider,
}

pub struct PlatformServices<'a> {
    pub fn new(hw: HardwareProviders<'a>) -> Self;
    pub fn load_config(&mut self) -> Result<(), HalError>;
    pub fn feed_watchdog(&mut self);
    pub fn is_provisioned(&self) -> bool;
}
```

## HAL Traits

Veja `docs/reference/architecture/hal.md`.