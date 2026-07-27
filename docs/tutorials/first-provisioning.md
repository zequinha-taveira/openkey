# Primeiro Provisionamento

## Pré-requisitos

- Dispositivo OpenKey conectado
- OpenKey Configurator instalado

## Passos

1. **Conectar o dispositivo**
   ```bash
   openkey-configurator detect
   ```

2. **Criar Board Profile**
   ```bash
   # Editar board.json
   {
     "manufacturer": "OpenKey",
     "model": "RP2350-REF",
     "revision": "1.0",
     "usb": {"vid": 0x16C0, "pid": 0x27DB}
   }
   ```

3. **Criar Device Profile**
   ```bash
   # Editar device.json
   {
     "serial_number": "OPENKEY-000001",
     "usb_identity": {"product_name": "OpenKey Security Key"}
   }
   ```

4. **Provisionar**
   ```bash
   openkey-configurator provision --board board.json --device device.json
   ```

5. **Verificar**
   ```bash
   openkey-cli info
   ```