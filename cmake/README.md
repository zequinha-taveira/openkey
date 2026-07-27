# CMake Support (`cmake/`)

Módulos e scripts CMake para integração do OpenKey com projetos C/C++:

- Localização e link das bibliotecas de plataforma OpenKey.
- Suporte a toolchains cross-compilação (ARM Cortex-M, RISC-V).
- Integração com `probe-rs` e `OpenOCD` para gravação via CMake.

> **Nota**: O firmware primário é compilado com Cargo/Rust. Este diretório
> destina-se à integração com SDKs de fabricantes (Pico SDK, ESP-IDF, Zephyr)
> que utilizam CMake como sistema de build.
