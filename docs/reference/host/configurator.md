# Configurator

## Função

Ferramenta para provisionamento de dispositivos OpenKey.

## Operações

- **Provisionar** - Definir Board Profile, Device Profile, Application Configuration
- **Flash Firmware** - Atualizar firmware
- **Recuperar** - Restaurar configurações padrão

## Fluxo de Provisionamento

1. Conectar dispositivo
2. Iniciar configuração
3. Definir Board Profile
4. Definir Device Profile
5. Configurar Application Configuration
6. Salvar na Flash

## Uso

```bash
openkey-configurator provision --board my-board.json --device my-device.json
```