# Camada de Plataforma

## Componentes

### HAL (Hardware Abstraction Layer)
Traits para GPIO, USB, Flash, SPI, I²C, UART, Timer, RNG, Watchdog.

### Board Profile
Descrição de dados da placa de hardware.

### Device Profile
Dados do dispositivo físico único.

### Configuration Manager
Gerenciamento de configuração persistente.

## Fluxo de Dados

```text
Application Configuration
        │
Configuration Manager
        │
Board Profile + Device Profile
        │
Platform Services
        │
OpenKey Core
```