# Adicionar Novo Board

## Passos

1. **Criar crate de board**
   ```
   boards/<nome>/
   ├── Cargo.toml
   └── src/
       └── main.rs
   ```

2. **Implementar HAL traits**
   - `RngProvider`
   - `FlashStorageProvider`
   - `UsbTransportProvider`
   - `GpioProvider`
   - `TimerProvider`
   - `WatchdogProvider`

3. **Definir Board Profile**
   ```rust
   pub const BOARD_PROFILE: BoardProfile = BoardProfile {
       manufacturer: "MeuFabricante",
       model: "MeuBoard",
       revision: "1.0",
       // ... outros campos
   };
   ```

4. **Adicionar ao workspace**
   ```toml
   members = [
       # ... outros membros
       "boards/meu-board",
   ]
   ```

5. **Build**
   ```bash
   cargo build --package openkey-target-meu-board
   ```