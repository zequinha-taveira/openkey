# Arquitetura do OpenKey

## Vision Geral

O OpenKey é um framework universal de código aberto para chaves de segurança FIDO2/WebAuthn.

## Arquitetura em Camadas

```text
Startup / firmware por MCU
        │
HAL por MCU
        │
Board Profile + Device Profile
        │
Configuration Manager
        │
Platform Services
        │
OpenKey Core
```

## Componentes

### OpenKey Core
Núcleo de segurança e protocolo CTAP2, desenvolvido em Rust `no_std`.

### Platform Services
Orquestra Board Profile, Device Profile, Configuration Manager e HAL.

### HAL (Hardware Abstraction Layer)
Abstrações de hardware: GPIO, USB, Flash, SPI, I²C, UART, Timer, RNG, Watchdog.

### Board Profile
Descrição de dados da placa (fabricante, modelo, GPIOs, LED, botão, USB).

### Device Profile
Dados do dispositivo físico (número de série, identidade USB, calibração).

### Configuration Manager
Gerencia configuração persistente durante o provisionamento.

O Board Profile é resolvido por identificador a partir de um catálogo; os dados
específicos da unidade e da aplicação são carregados de dois slots de Flash
validados.
