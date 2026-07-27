# Build Firmware RP23xx

## Requisitos

- ARM toolchain
- Pico SDK (opcional para build real)

## Comando

```bash
cargo build --package openkey-target-rp2350 --target thumbv6m-none-eabi
```

## Opções

```bash
# Release
cargo build --release

# Com otimização de tamanho
cargo build --release --profile max-size
```

## Saída

- `target/thumbv6m-none-eabi/release/openkey-target-rp2350` - Binário firmware