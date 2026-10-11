# Testes do simulador

O binário `openkey-simulator` fornece um transporte em memória e uma máquina de estados mínima. O fluxo de evolução é contrato → simulador → testes → target físico.

Execute:

```bash
cargo run -p openkey-simulator
cargo test -p openkey-simulator
```
