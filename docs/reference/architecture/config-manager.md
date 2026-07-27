# Gerenciador de Configuração

## Função

Gerencia Board Profile, Device Profile e Application Configuration persistentes.
O formato é binário, versionado, determinístico e não usa alocação dinâmica.

## Persistência autenticada

A Flash contém dois slots definidos por `ConfigStorageLayout`. Cada registro v2
contém magic, versão, estado, geração, tamanho do payload, nonce AES-GCM de 96
bits e tag de 128 bits, seguido do payload cifrado.

O payload é cifrado e autenticado com AES-256-GCM. Magic, versão, geração,
tamanho e nonce formam AAD; o campo de estado fica fora da AAD para poder ser
marcado como válido somente no término da escrita. Registros incompletos,
corrompidos, com tag inválida, texto inválido ou versão diferente de v2 nunca
deixam o dispositivo no estado `Provisioned`.

`ConfigKeyProvider` fornece a chave AES-256 apenas para a operação e o material
efêmero é zeroizado. `RngProvider` saudável gera um nonce novo a cada gravação.
Registros v1 com checksum não são aceitos: a migração exige reprovisionamento.

## Interfaces

- `load(flash, key_provider, catalog, layout)` autentica e carrega a configuração.
- `save(flash, crypto, layout, board, device, app)` cifra e grava no slot inativo;
  `ConfigCryptoContext` agrupa o provider de chave e o RNG.
- `board_profile()`, `device_profile()` e `app_config()` expõem somente dados autenticados.
