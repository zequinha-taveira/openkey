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
    pub fn load_config(
        &mut self,
        key_provider: &dyn ConfigKeyProvider,
        catalog: &dyn BoardProfileCatalog,
        layout: ConfigStorageLayout,
    ) -> Result<(), ConfigurationError>;
    pub fn feed_watchdog(&mut self);
    pub fn is_provisioned(&self) -> bool;
}
```

`ConfigKeyProvider` deve fornecer uma chave AES-256 exclusiva do dispositivo.
Se estiver indisponível, o carregamento falha fechado.

## HAL Traits

Veja `docs/reference/architecture/hal.md`.
