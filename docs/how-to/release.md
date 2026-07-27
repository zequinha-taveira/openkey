# Release

## Versionamento

Usa Semantic Versioning (SemVer).

## Processo

1. **Atualizar CHANGELOG.md**
   ```markdown
   ## [X.Y.Z] - YYYY-MM-DD
   - Novas funcionalidades
   - Correções de bugs
   - Breaking changes
   ```

2. **Criar tag**
   ```bash
   git tag -a vX.Y.Z -m "Release vX.Y.Z"
   git push origin vX.Y.Z
   ```

3. **Build de release**
   ```bash
   cargo build --release
   ```

4. **Publicar no GitHub**
   - Criar Release
   - Anexar binários
   - Verificar assinatura