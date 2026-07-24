# Processo de Release (RELEASING.md)

Este documento descreve os passos e requisitos para publicação de novas versões do firmware, SDKs e ferramentas do OpenKey.

## 📌 Política de Versionamento

Seguimos a especificação [SemVer 2.0.0](https://semver.org/):
- **MAJOR**: Alterações incompatíveis de protocolo CTAP/API ou mudanças na estrutura de dados da Flash.
- **MINOR**: Novas funcionalidades compatíveis com versões anteriores (ex: adição de suporte a novos comandos CTAP2.1).
- **PATCH**: Correções de bugs e patches de segurança mantendo compatibilidade.

Para obter detalhes de versão entre sub-pacotes do monorepo, consulte [`docs/adr/ADR-0009-versioning.md`](docs/adr/ADR-0009-versioning.md).

## 📝 Checklist de Pre-Release

1. **Testes de Integração e HIL**: Verifique aprovação no simulador e em bancada de testes de hardware.
2. **Fuzzing**: Execute 24h de fuzzing em `fuzz/` sem crashes reportados.
3. **Auditoria de `unsafe`**: Revise todos os blocos `unsafe` modificados conforme [`docs/security/unsafe-policy.md`](docs/security/unsafe-policy.md).
4. **Assinatura de Firmware**: O binário do firmware deve ser assinado digitalmente com a chave privada de release do projeto.
5. **Atualização de Changelog**: Atualize o arquivo `CHANGELOG.md` com a versão final e data.

## 🚀 Passos de Publicação

```bash
# 1. Bump de versão no monorepo
cargo release release --execute

# 2. Gerar binários reproduzíveis do firmware
docker build -t openkey-builder -f scripts/docker/Dockerfile .
docker run --rm -v $(pwd):/workspace openkey-builder cargo build --release --target thumbv7em-none-eabihf

# 3. Assinar artefatos e criar Tag no GitHub
git tag -s v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
```

Mais detalhes sobre o pipeline de compilação em [`docs/development/release.md`](docs/development/release.md).
