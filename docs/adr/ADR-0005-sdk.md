# ADR-0005: Arquitetura do Host SDK e Suporte a Bindings Multi-linguagem

- **Status**: Aceito
- **Data**: 2026-07-24
- **Autores**: Equipe de Desenvolvimento Host

## 📌 Contexto

Para permitir que a chave OpenKey seja facilmente integrada por desenvolvedores em aplicações desktop, web e scripts de administração, é necessária uma biblioteca client robusta com suporte a múltiplas linguagens de programação.

## 💡 Decisão

Construiremos o **Core SDK em Rust** (`host/sdk`), expondo interfaces FFI seguras C e bindings de alto nível para **Python** (`pyopenkey`) usando PyO3.

## 🚀 Consequências

### Positivas
- Código base único para manutenção da lógica de transporte e parsing de pacotes CTAP2.
- Excelente desempenho e portabilidade entre Windows, Linux e macOS.

### Compromissos (Trade-offs)
- Exige pipelines de build de wheels nativas em CI para diferentes arquiteturas de SO.
