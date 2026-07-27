# Board Profile

## Estrutura de Dados

```rust
pub struct BoardProfile {
    pub manufacturer: &'static str,
    pub model: &'static str,
    pub revision: &'static str,
    pub flash: FlashConfig,
    pub usb: UsbConfig,
    pub led: Option<LedConfig>,
    pub button: Option<ButtonConfig>,
    pub features: OptionalFeatures,
}
```

## Componentes

### FlashConfig
- `total_size` - Tamanho total em bytes
- `page_size` - Tamanho de página
- `sector_size` - Tamanho de setor

### UsbConfig
- `vid` - Vendor ID
- `pid` - Product ID
- `bcd_version` - Versão USB
- `max_packet_size` - Tamanho máximo de pacote

### LedConfig
- `pin` - Pino GPIO
- `active_high` - Nível ativo

### ButtonConfig
- `pin` - Pino GPIO
- `active_low` - Nível ativo baixo
- `pull_up` - Pull-up habilitado

### OptionalFeatures
- `has_nfc` - Suporte NFC
- `has_ble` - Suporte BLE
- `has_secure_element` - Elemento seguro
- `has_tamper_detect` - Detecção de manipulação

## Uso

O Board Profile é definido durante o provisionamento e armazenado na Flash.