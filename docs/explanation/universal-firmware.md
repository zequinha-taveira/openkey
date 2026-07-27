# Firmware Universal

## Conceito

Um único firmware por arquitetura de MCU, reutilizando código através de abstrações de hardware.

## Arquitetura

```
OpenKey Core (protocolo, segurança)
        │
Platform Services (orquestração)
        │
Configuration Manager (persistência)
        │
Board Profile + Device Profile (dados)
        │
HAL (GPIO, USB, Flash, SPI, I²C, UART, Timer, RNG, Watchdog)
```

## Vantagens

- **Reutilização máxima** - Um firmware serve múltiplos boards
- **Manutenibilidade** - Mudanças em um lugar
- **Portabilidade** - Adição de novos MCUs é simples
- **Auditabilidade** - Código centralizado e revisado