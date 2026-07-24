# ADR-0001: Seleção da Linguagem Rust para Firmware

- **Status**: Aceito
- **Data**: 2026-07-24
- **Autores**: Equipe de Arquitetura OpenKey

## 📌 Contexto

Chaves de segurança de hardware como o OpenKey exigem o mais alto nível de garantia de segurança contra ataques de estouro de buffer (*buffer overflows*), *use-after-free*, corrupção de memória e corridas de dados (*data races*). Historicamente, firmwares em C/C++ são vulneráveis a uma ampla classe de falhas de memória.

## 💡 Decisão

Decidimos utilizar a linguagem **Rust** (`no_std`) como linguagem primária para a implementação de todo o firmware embarcado e simulador do OpenKey.

## 🚀 Consequências

### Positivas
- Garantia de segurança de memória em tempo de compilação sem overhead de Garbage Collector.
- Ecossistema maduro de crates embarcadas para microcontroladores ARM Cortex-M (`cortex-m-rt`, `embedded-hal`).
- Integração nativa de testes unitários e ferramentas de verificação estática (`clippy`).

### Compromissos (Trade-offs)
- Curva de aprendizado inicial para desenvolvedores acostumados a C tradicional.
- Necessidade de política rigorosa para controlar o uso da palavra-chave `unsafe` (ver [ADR-0004](ADR-0004-unsafe.md)).
