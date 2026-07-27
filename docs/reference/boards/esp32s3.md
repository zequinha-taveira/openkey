# ESP32-S3

## Visão Geral

Microcontrolador da Espressif (ESP32-S3).

## Recursos

- Xtensa LX7
- USB OTG
- 8MB Flash
- 51 GPIOs
- TRNG de hardware

## Configuração

### USB
- VID: 0x303A (Espressif)
- PID: 0x8031 (FIDO2)

### Flash
- Total: 8MB
- Page Size: 4KB

### GPIOs Padrão
- LED: GPIO 2 (active_high)
- Botão: GPIO 0 (active_low, pull_up)

## Build

```bash
cargo build --package openkey-target-esp32s3 --target riscv32imac-unknown-none-elf
```

## Dependências

- `esp-idf-sys` - Bindings para ESP-IDF
- `tinyusb` - Stack USB HID