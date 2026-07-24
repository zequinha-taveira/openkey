# Arquitetura do Firmware (`docs/architecture/firmware.md`)

## 🧱 Arquitetura de Firmware (`no_std`)

O firmware do OpenKey é compilado para alvos ARM Cortex-M (ex: `thumbv7em-none-eabihf`) utilizando Rust `no_std` para garantir previsibilidade de tempo de execução, ausência de vazamentos de memória e runtime determinístico.

## 🔄 Fluxo de Execução

1. **Boot Sequencer**:
   - Inicializa clock do sistema, TRNG e Periféricos de E/S.
   - Valida integridade do mapa da Flash e assinaturas do bootloader (`docs/architecture/storage.md`).
2. **Loop de Eventos Assíncronos / RTIC**:
   - Interrupções de USB HID e NFC alimentam filas de pacotes de entrada.
   - O desfragmentador de pacotes HID reagrupa relatórios em mensagens CTAP2 completas.
3. **Despachante CTAP2**:
   - O parser CBOR lê o comando CTAP2 (`authenticatorMakeCredential`, `authenticatorGetAssertion`, etc.).
   - Processa a checagem de presença de usuário (User Presence / User Verification) via botão ou sensor biométrico.
4. **Resposta e Criptografia**:
   - Gera par de chaves ECDSA P-256 / Ed25519.
   - Retorna estrutura de resposta codificada em CBOR dividida em frames USB HID de 64 bytes.
