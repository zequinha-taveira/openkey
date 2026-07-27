# Device Profile

## Estrutura de Dados

```rust
pub struct DeviceProfile {
    pub serial_number: &'static str,
    pub usb_identity: UsbIdentity,
    pub calibration: Option<CalibrationData>,
    pub manufacturing: Option<ManufacturingData>,
}
```

## Componentes

### UsbIdentity
- `vid` - Vendor ID
- `pid` - Product ID
- `serial_number` - Número de série USB
- `product_name` - Nome do produto
- `manufacturer_name` - Nome do fabricante

### CalibrationData
- `rng_offset` - Offset do RNG
- `rng_scale` - Fator de escala do RNG
- `temp_offset` - Offset de temperatura
- `temp_scale` - Fator de escala de temperatura

### ManufacturingData
- `production_date` - Data de produção
- `production_location` - Localização
- `batch_number` - Número do lote
- `test_result` - Resultado do teste

## Uso

O Device Profile identifica uma unidade física única e é persistido junto à
configuração. Para funcionar em firmware `no_std`, campos de texto usam
`DeviceText`: UTF-8 validado, comprimento explícito e capacidade máxima de 64
bytes. Dados que excedam esse limite são rejeitados no provisionamento.
