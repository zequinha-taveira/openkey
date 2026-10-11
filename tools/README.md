# Ferramentas Auxiliares (`tools/`)

Ferramentas internas de desenvolvimento, automação e geração para o projeto OpenKey.

## Estrutura

```text
tools/
├── manufacturing/   # Ferramentas de produção e injeção de chaves de fábrica
├── migration/       # Scripts de migração de dados e perfis de versões antigas
├── scripts/         # Scripts de automação: build, lint, release, Docker
└── generators/      # Geradores de código: Board Profiles, documentação, certificados

> O simulador agora vive em `simulator/` (raiz do monorepo) — ver ADR-0010.
```

## Uso

Cada subdiretório possui seu próprio `README.md` com instruções de uso.
