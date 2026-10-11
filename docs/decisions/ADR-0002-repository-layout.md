# ADR-0002: layout canônico do repositório

- **Status:** aceito
- **Decisão:** o workspace modular em `crates/` é o layout Rust canônico na raiz.
- **Contexto:** havia uma implementação anterior distribuída em `protocol/`, `firmware/` e
  `simulator/`, além de um scaffold paralelo aninhado.
- **Consequência:** a implementação anterior foi movida para `legacy/` sem exclusão; novos
  componentes devem ser adicionados ao workspace raiz e hardware específico deve ficar em
  `boards/`. A migração de funcionalidades será incremental e revisável.
