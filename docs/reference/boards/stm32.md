# STM32

## Visão Geral

Microcontroladores STMicroelectronics (STM32).

## Famílias Suportadas

- STM32F4 (Cortex-M4)
- STM32L4 (Cortex-M4)
- STM32U5 (Cortex-M33)

## Recursos Comuns

- FPU
- USB FS/HS
- 1-2MB Flash
- 64-512 GPIOs
- TRNG de hardware

## Build

```bash
# Cortex-M4
cargo build --package openkey-target-stm32f4 --target thumbv7em-none-eabihf

# Cortex-M33
cargo build --package openkey-target-stm32u5 --target thumbv8m.main-none-eabihf
```

## HAL

Usa `stm32f4xx-hal`, `stm32l4xx-hal`, ou `stm32u5xx-hal`.