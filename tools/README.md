# Ferramentas auxiliares (`tools/`)

Ferramentas internas de desenvolvimento, automação e geração para o projeto OpenKey.

## Estrutura

```text
tools/
├── openkey-cli/      # CLI Rust do workspace ativo
├── manufacturing/    # Ferramentas de produção e injeção de chaves
├── migration/        # Scripts de migração de dados e perfis antigos
├── scripts/          # Automação: build, lint e release
└── generators/       # Geradores de perfis de placas e documentação
```

O simulador e o firmware da implementação anterior foram preservados em `legacy/`. Um
simulador host será adicionado ao workspace ativo quando sua fronteira de transporte estiver
definida e coberta por testes.
