# nRF

## Visão Geral

Microcontroladores Nordic Semiconductor (nRF52/53).

## Famílias Suportadas

- nRF52840 (Cortex-M4)
- nRF5340 (Cortex-M33 dual-core)

## Recursos

- ARM Cortex-M4/M33
- USB (nRF52840)
- 1-2MB Flash
- 64 GPIOs
- TRNG de hardware
- BLE integrado

## Build

```bash
cargo build --package openkey-target-nrf52840 --target thumbv7em-none-eabihf
```

## Dependências

- `nrf52840-hal` ou `nrf5340-hal`
- `nrf-softdevice` (opcional, BLE)