# Gerenciador de Configuração

## Função

Gerencia a configuração persistente durante o provisionamento.

## Componentes

### Board Profile
Descrição de dados da placa.

### Device Profile
Dados do dispositivo físico.

### Application Configuration
Configuração da aplicação (CTAP2, CCID, OpenPGP, PIV, Logging, Policies).

## Fluxo

```text
Provisionador
        │
Configuration Manager
        │
Flash Storage
```

## Métodos

- `load()` - Carrega configuração do Flash
- `save()` - Salva configuração no Flash
- `board_profile()` - Retorna Board Profile
- `device_profile()` - Retorna Device Profile
- `app_config()` - Retorna Application Configuration