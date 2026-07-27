# OpenKey Hardware Strategy

## Princípio

O OpenKey é um **projeto de software**.

Seu objetivo é fornecer um framework universal para chaves de segurança
FIDO2/WebAuthn. O projeto **não possui nem exige uma placa própria (PCB)**.

---

## Hardware de Referência

O OpenKey utiliza placas de desenvolvimento existentes como plataformas de execução.

### RP23xx (Referência Primária)

| Placa | Fabricante |
|-------|-----------|
| Raspberry Pi Pico 2 | Raspberry Pi Foundation |
| Raspberry Pi Pico 2 W | Raspberry Pi Foundation |
| Seeed XIAO RP2350 | Seeed Studio |
| Pimoroni Tiny2350 | Pimoroni |
| Adafruit Feather RP2350 | Adafruit |

### RP2040

| Placa | Fabricante |
|-------|-----------|
| Raspberry Pi Pico | Raspberry Pi Foundation |
| Raspberry Pi Pico W | Raspberry Pi Foundation |
| Tiny2040 | Pimoroni |
| XIAO RP2040 | Seeed Studio |
| Feather RP2040 | Adafruit |

### Futuro

- ESP32-S3 (Espressif)
- STM32 (STMicroelectronics)
- nRF52 / nRF54 (Nordic Semiconductor)

---

## O que NÃO faz parte do projeto

O OpenKey **não** mantém:

- PCB própria
- Esquemáticos eletrônicos
- BOM (Bill of Materials)
- Arquivos KiCad
- Gabinete / enclosure
- Circuito USB dedicado

Esses elementos pertencem ao fabricante da placa utilizada.

---

## Modelo de Suporte

O OpenKey opera com três camadas de abstração:

```text
┌─────────────────────────────┐
│   Firmware (por arquitetura)│  ← Um firmware por família de MCU
├─────────────────────────────┤
│   Board Profile (por placa) │  ← Dados de hardware em YAML
├─────────────────────────────┤
│   Device Profile (por unit) │  ← Identidade única do dispositivo
└─────────────────────────────┘
```

- **Um firmware por arquitetura** — ex: um único binário para todos os RP2350.
- **Um Board Profile por modelo de placa** — descreve GPIOs, LED, botão, Flash, USB.
- **Um Device Profile por dispositivo** — número de série, calibração, fabricação.

Assim, diferentes placas podem compartilhar **exatamente o mesmo firmware**.

---

## Benefícios

- Elimina custos de hardware próprio
- Reduz manutenção de PCB e eletrônica
- Facilita testes pela comunidade (qualquer pessoa com uma Pico 2 pode testar)
- Permite uso imediato de placas comerciais disponíveis
- Acelera o desenvolvimento do firmware
- Amplia a portabilidade do OpenKey

---

## Papel do Board Profile

Cada placa é descrita por um perfil YAML contendo apenas informações de hardware:

- GPIOs (LED, botão de presença)
- Memória Flash (tamanho, page size, sector size)
- Parâmetros USB (VID, PID, product string)
- Recursos opcionais (NFC, BLE, Secure Element, Tamper Detect)

O firmware permanece **inalterado** — a personalização é feita pelo perfil de dados.

Perfis disponíveis em: [`boards/profiles/`](../../boards/profiles/)

---

## Filosofia

> O OpenKey não vende hardware.
>
> O OpenKey fornece um **framework aberto** que pode ser executado em qualquer
> placa compatível com a arquitetura suportada.
>
> Fabricantes, empresas ou membros da comunidade podem desenvolver placas
> dedicadas no futuro **sem necessidade de modificar o núcleo do projeto**.
