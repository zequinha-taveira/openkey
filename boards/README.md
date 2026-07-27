# OpenKey — Boards (`boards/`)

Este diretório contém **apenas perfis de hardware** no formato YAML.
Nenhum código de lógica de negócio ou crate Rust deve residir aqui.

## Estrutura

```text
boards/
├── profiles/        # Perfis YAML por família de MCU
│   ├── rp23xx/      # Raspberry Pi RP2350 / RP2040
│   ├── esp32s3/     # Espressif ESP32-S3
│   ├── stm32/       # STMicroelectronics STM32
│   └── nrf/         # Nordic Semiconductor nRF52/nRF54
├── templates/       # Templates de perfil para novos boards
└── examples/        # Exemplos de perfis comentados
```

## Formato de Perfil

Cada arquivo `.yaml` descreve os pinos, periféricos e recursos de uma placa
específica. O firmware lê esses perfis em tempo de build via `build.rs` ou
carrega a configuração armazenada na flash em tempo de execução.

Consulte [`docs/reference/board-profile-schema.md`](../docs/reference/) para
o schema YAML completo.
