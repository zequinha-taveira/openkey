# Guia de Estilo de Código (`docs/development/coding-style.md`)

## 📐 Diretrizes de Código Rust

- Formatação estrita com `rustfmt`.
- Lints estritos com `clippy`: `#![deny(clippy::all)]`.
- Documentação pública obrigatória: `#![warn(missing_docs)]` nos crates do SDK e utilitários.
- Tipos de erro descritivos e sem `panic!` em runtime de firmware.
