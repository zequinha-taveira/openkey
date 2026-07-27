# Testes Unitários (`tests/unit/`)

Testes unitários isolados por crate, sem dependências externas de hardware ou rede.

Cada crate do workspace contém seus próprios testes unitários em `src/` via `#[cfg(test)]`.
Este diretório abriga testes adicionais que cruzam crates mas não requerem integração completa.
