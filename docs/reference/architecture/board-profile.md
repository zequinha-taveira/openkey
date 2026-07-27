# Board Profile

## Estrutura de Dados

```rust
pub struct BoardProfile {
    pub id: BoardProfileId,
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

O Board Profile é uma descrição reutilizável, mantida como dado de fabricação
(YAML) e disponibilizada ao firmware como catálogo compilado. A Flash armazena
somente seu `BoardProfileId`; durante o boot, `BoardProfileCatalog` resolve o
identificador para o perfil conhecido. O perfil completo não é desserializado
da Flash.
