# ADR-0006: Pipeline de Compilação Reproduzível via Containers Docker

- **Status**: Aceito
- **Data**: 2026-07-24
- **Autores**: Equipe de DevOps e Segurança

## 📌 Contexto

Diferenças de versão de compiladores locais, caminhos de sistema de arquivos e variáveis de ambiente podem gerar binários de firmware ligeiramente diferentes, prejudicando auditorias públicas de segurança.

## 💡 Decisão

Definiremos o **Docker** como ambiente oficial e obrigatório para compilação das releases do firmware do OpenKey, padronizando a versão exata do toolchain LLVM/Rust.

## 🚀 Consequências

### Positivas
- Garantia de que qualquer auditor independente consiga recompilar o firmware exatamente com o mesmo hash SHA-256.
- Eliminação da dependência de configurações da máquina pessoal do desenvolvedor.

### Compromissos (Trade-offs)
- Exige instalação prévia do Docker no ambiente de publicação da release.
