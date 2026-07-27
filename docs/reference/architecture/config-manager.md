# Gerenciador de Configuração

## Função

Gerencia a configuração persistente durante o provisionamento. O registro é
binário, versionado, determinístico e não usa alocação dinâmica.

## Componentes

### Board Profile
Descrição de dados da placa.

### Device Profile
Dados do dispositivo físico.

### Application Configuration
Configuração da aplicação (CTAP2, CCID, OpenPGP, PIV, Logging, Policies).

## Persistência e recuperação

A Flash contém dois slots exclusivos de configuração definidos pelo board/MCU
através de `ConfigStorageLayout`. Cada slot contém `magic`, versão, estado,
geração, tamanho do payload e checksum, seguido de `BoardProfileId`,
`DeviceProfile` e `AppConfig`.

O gravador apaga e prepara o slot inativo, escreve o payload e marca o slot
como válido apenas ao final. Na inicialização, o gerenciador valida ambos os
slots e escolhe o registro válido de maior geração. Um registro incompleto,
com versão desconhecida, tamanho inválido, checksum incorreto ou dados UTF-8
inválidos nunca deixa o dispositivo no estado `Provisioned`.

O checksum atual detecta corrupção acidental; ele não autentica a configuração.
A autenticação criptográfica será integrada ao subsistema definido no ADR-0002.

## Fluxo

```text
Provisionador
        │
Configuration Manager
        │
Flash Storage
```

## Interfaces

- `load(flash, catalog, layout)` - Carrega e valida a configuração do Flash
- `save(flash, layout, board, device, app)` - Salva no slot inativo
- `board_profile()` - Retorna Board Profile
- `device_profile()` - Retorna Device Profile
- `app_config()` - Retorna Application Configuration
