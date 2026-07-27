# Flash Firmware

## Ferramentas

- `openkey-cli`
- `picotool` (RP2350)
- `esptool` (ESP32)

## RP2350

1. **Build o firmware**
   ```bash
   cargo build --package openkey-target-rp2350 --release
   ```

2. **Flash com picotool**
   ```bash
   picotool load -x target/thumbv6m-none-eabi/release/openkey-target-rp2350
   ```

## ESP32

1. **Build o firmware**
   ```bash
   cargo build --package openkey-target-esp32s3 --release
   ```

2. **Flash com esptool**
   ```bash
   esptool.py --chip esp32s3 write_flash 0x0 firmware.bin
   ```