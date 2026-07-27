# ADR-0011: Persistência A/B de Configuração de Dispositivo

- **Status**: Aceito
- **Data**: 2026-07-27

## Decisão

Board Profiles são resolvidos por `BoardProfileId` em catálogo compilado. Device
Profile e AppConfig usam dois slots em Flash definidos por `ConfigStorageLayout`.
O registro v2 cifra o payload e autentica payload e cabeçalho imutável com
AES-256-GCM. Registros v1 baseados apenas em checksum são rejeitados.

## Consequências

- Uma gravação interrompida preserva o slot válido anterior.
- `ConfigKeyProvider` e TRNG saudável são obrigatórios; falhas não provisionam
  o dispositivo.
- Textos persistidos permanecem limitados e validados em `no_std`.
- A migração de v1 requer reprovisionamento.
