# ADR-0009: Política de Versionamento Semântico e Release em Monorepo

- **Status**: Aceito
- **Data**: 2026-07-24
- **Autores**: Equipe de Mantenedores OpenKey

## 📌 Contexto

Como o repositório OpenKey é estruturado como um monorepo contendo firmware embarcado, SDKs para host, ferramentas CLI/GUI e simuladores, é necessário alinhar a política de versionamento entre os pacotes sem quebrar a interoperabilidade.

## 💡 Decisão

Adotaremos **Semantic Versioning 2.0.0 (SemVer)** com tags globais no repositório (`vX.Y.Z`). O firmware e o SDK manterão alinhamento de versão Major/Minor.

## 🚀 Consequências

### Positivas
- Clareza para usuários e integradores sobre compatibilidade entre versões de firmware e bibliotecas client.
- Automação simplificada de lançamentos via `cargo-release` e GitHub Actions.

### Compromissos (Trade-offs)
- Exige atualização síncrona do arquivo `CHANGELOG.md` no monorepo para todos os sub-pacotes modificados em um ciclo de release.
