# Depuração e HIL (`docs/development/debugging.md`)

## 🛠️ Ferramentas de Depuração do Firmware

- **Probe-rs / RTT**: Log em tempo real sem impacto no timing USB HID via Real-Time Transfer.
  ```bash
  probe-rs run --chip STM32WB55CGUx target/thumbv7em-none-eabihf/release/openkey-firmware
  ```
- **GDB / OpenOCD**: Suporte a breakpoints de hardware em registradores de periféricos e criptografia.
