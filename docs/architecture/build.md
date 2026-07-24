# Sistema de Compilação Reproduzível (`docs/architecture/build.md`)

## 🏗️ Reproducibilidade de Binários

Para garantir que o binário gerado para gravação no firmware corresponda exatamente ao código-fonte auditado no repositório, o OpenKey impõe um ambiente de build reproduzível via container Docker isolado.

```bash
# Executar build reproduzível via Docker
docker run --rm -v $(pwd):/workspace openkey-builder cargo build --release --target thumbv7em-none-eabihf
```

Flags de compilação como `remap-path-prefix` são ativadas em `Cargo.toml` para remover caminhos absolutos do ambiente local do desenvolvedor dos binários compilados.
