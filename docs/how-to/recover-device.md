# Recuperar Dispositivo

## Quando Usar

- Firmware corrompido
- Configuração perdida
- PIN esquecido

## Passos

1. **Reset de fábrica**
   ```bash
   # Dentro de 10 segundos após conectar
   openkey-cli reset --force
   ```

2. **Reprovisionar**
   ```bash
   openkey-configurator provision --board board.json --device device.json
   ```

## Alternativa: Bootloader

Se o firmware estiver corrompido:
1. Conectar em modo bootloader (GPIO específico)
2. Flash manual via ferramenta do fabricante