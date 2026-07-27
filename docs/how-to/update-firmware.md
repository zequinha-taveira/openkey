# Atualizar Firmware

## Passos

1. **Verificar versão atual**
   ```bash
   openkey-cli info
   ```

2. **Obter novo firmware**
   ```bash
   # Via release
   wget https://github.com/openkey/firmware/releases/latest/firmware.bin
   ```

3. **Verificar assinatura**
   ```bash
   openkey-cli verify-firmware firmware.bin
   ```

4. **Atualizar**
   ```bash
   openkey-cli update firmware.bin
   ```

## Notas

- O firmware deve ser assinado
- Backup de configuração é automático
- Device pode precisar de reset após atualização