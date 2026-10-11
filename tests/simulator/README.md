# Testes do simulador

O binário `openkey-simulator` fornece implementações determinísticas para:

- vault em memória;
- RNG controlável;
- relógio determinístico;
- presença do usuário;
- falha de energia antes do commit;
- corrupção controlada na leitura;
- timeout e reordenação de pacotes.

O fluxo de evolução é contrato → simulador → testes → target físico.

Execute:

```bash
cargo run -p openkey-simulator
cargo test -p openkey-simulator
```
