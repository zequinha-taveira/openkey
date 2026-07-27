# ADR-0011: Persistência A/B de Configuração de Dispositivo

- **Status**: Aceito
- **Data**: 2026-07-27

## Contexto

O gerenciador anterior apenas verificava magic e versão na Flash, mas marcava o
dispositivo como provisionado sem reconstruir Board Profile, Device Profile e
AppConfig. Além disso, um offset fixo conflita com o layout de boot dual-bank.

## Decisão

Board Profiles permanecem dados reutilizáveis e são resolvidos por
`BoardProfileId` por um `BoardProfileCatalog` compilado. Os dados de unidade e
aplicação são codificados explicitamente em um payload `no_std` de tamanho
fixo. O armazenamento usa dois slots definidos por `ConfigStorageLayout`; o
registro válido de maior geração é selecionado no boot.

Um checksum detecta corrupção acidental nesta fase. Ele não é um mecanismo de
autenticidade: a proteção criptográfica deverá ser fornecida pelo subsistema de
storage/crypto definido no ADR-0002.

## Consequências

- Perda de energia durante uma gravação não invalida o registro anterior.
- Endereços de configuração deixam de ser globais e passam a pertencer ao
  layout da plataforma.
- Textos persistidos têm limite explícito de 64 bytes e são validados como UTF-8.
- A API pública de `DeviceProfile` passa a usar `DeviceText`, compatível com
  reconstrução segura em `no_std`.
