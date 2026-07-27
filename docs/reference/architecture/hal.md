# HAL (Hardware Abstraction Layer)

## Traits Implementados

### GPIO
- `set_direction()` - Configura direção do pino
- `read_pin()` - Lê nível lógico
- `write_pin()` - Escreve nível lógico
- `toggle_pin()` - Alterna nível

### USB
- `send_packet()` - Envia pacote HID
- `receive_packet()` - Recebe pacote HID
- `is_connected()` - Verifica conexão

### Flash
- `read()` - Lê dados da Flash
- `write()` - Escreve dados na Flash
- `erase()` - Apaga setor da Flash
- `total_size()` - Retorna tamanho total

### SPI
- `configure()` - Configura frequência e modo
- `transfer()` - Transferência full-duplex
- `write()` - Escrita-only

### I²C
- `configure()` - Configura frequência
- `read()` - Lê de dispositivo
- `write()` - Escreve para dispositivo
- `write_read()` - Escrita seguida de leitura

### UART
- `init()` - Inicializa com baud rate
- `write()` - Envia dados
- `read()` - Recebe dados
- `available()` - Verifica dados disponíveis

### Timer
- `millis()` - Tempo em milissegundos
- `micros()` - Tempo em microssegundos
- `nanos()` - Tempo em nanossegundos
- `delay_ms()` - Delay em milissegundos
- `delay_us()` - Delay em microssegundos

### RNG
- `fill_bytes()` - Preenche com bytes aleatórios
- `next_u32()` - Gera número aleatório 32-bit
- `is_healthy()` - Verifica saúde do RNG

### Watchdog
- `init()` - Inicializa com timeout
- `feed()` - Alimenta o watchdog
- `disable()` - Desativa o watchdog

## Erros

Use o tipo `HalError` para tratamento de erros.