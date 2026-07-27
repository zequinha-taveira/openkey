# Criar Board Profile

## Estrutura

```json
{
  "manufacturer": "string",
  "model": "string",
  "revision": "string",
  "flash": {
    "total_size": 4194304,
    "page_size": 4096,
    "sector_size": 4096
  },
  "usb": {
    "vid": 6528,
    "pid": 10203,
    "bcd_version": 512,
    "max_packet_size": 64
  },
  "led": {
    "pin": {"port": 0, "pin": 25},
    "active_high": true
  },
  "button": {
    "pin": {"port": 0, "pin": 24},
    "active_low": true,
    "pull_up": true
  },
  "features": {
    "has_nfc": false,
    "has_ble": false,
    "has_secure_element": false,
    "has_tamper_detect": false
  }
}
```

## Campos

- `manufacturer` - Nome do fabricante
- `model` - Modelo da placa
- `revision` - Revisão do hardware
- `flash` - Configuração de memória Flash
- `usb` - Configuração USB (VID/PID)
- `led` - Configuração do LED (opcional)
- `button` - Configuração do botão (opcional)
- `features` - Recursos opcionais