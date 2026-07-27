# Provisionar Dispositivo

## Requisitos

- Dispositivo OpenKey conectado
- `openkey-configurator` instalado

## Passos

1. **Detectar dispositivo**
   ```bash
   openkey-configurator detect
   ```

2. **Criar arquivos de configuração**
   ```bash
   openkey-configurator generate-board-config
   openkey-configurator generate-device-config --serial OPENKEY-000001
   ```

3. **Editar configurações**
   - `board.json` - Configuração do board
   - `device.json` - Dados do dispositivo

4. **Provisionar**
   ```bash
   openkey-configurator provision --board board.json --device device.json
   ```

5. **Verificar**
   ```bash
   openkey-cli info
   ```