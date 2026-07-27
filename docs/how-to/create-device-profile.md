# Criar Device Profile

## Estrutura

```json
{
  "serial_number": "OPENKEY-000001",
  "usb_identity": {
    "vid": 6528,
    "pid": 10203,
    "serial_number": "OPENKEY-000001",
    "product_name": "OpenKey Security Key",
    "manufacturer_name": "OpenKey"
  },
  "calibration": {
    "rng_offset": 0,
    "rng_scale": 1000,
    "temp_offset": 0,
    "temp_scale": 1000
  },
  "manufacturing": {
    "production_date": 20240101,
    "production_location": "CN",
    "batch_number": 1,
    "test_result": true
  }
}
```

## Campos

- `serial_number` - Número de série único
- `usb_identity` - Identidade USB
- `calibration` - Dados de calibração (opcional)
- `manufacturing` - Dados de fabricação (opcional)