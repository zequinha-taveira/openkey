# RP23xx

## Visão Geral

Microcontrolador da Raspberry Pi (RP23xx).

## Recursos

- ARM Cortex-M33 (RP2350)
- USB 2.0 Full Speed
- 2MB Flash
- 26 GPIOs
- TRNG de hardware

## Configuração

### USB
- VID: 0x16C0
- PID: 0x27DB

### Flash
- Total: 4MB
- Page Size: 4KB
- Sector Size: 4KB

### GPIOs Padrão
- LED: GPIO 25 (active_high)
- Botão: GPIO 24 (active_low, pull_up)

## Build

```bash
cargo build --package openkey-target-rp2350 --target thumbv6m-none-eabi
```