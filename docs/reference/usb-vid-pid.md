# USB VID/PID Strategy

## Visão Geral

O OpenKey suporta dois cenários de USB VID/PID:

### 1. Placas de Desenvolvimento (comerciais)

Quando o firmware roda em placas existentes (Pico 2, XIAO, Tiny2350, etc.),
o VID/PID do **fabricante da placa** é utilizado. Não é necessária nenhuma
configuração adicional — os Board Profiles já contêm os valores corretos.

| Placa | VID | PID | Fabricante |
|-------|-----|-----|------------|
| Raspberry Pi Pico 2 | `0x2E8A` | `0x000F` | Raspberry Pi |
| Pimoroni Tiny2350 | `0x303A` | `0x8232` | Pimoroni |
| Seeed XIAO RP2350 | `0x2886` | `0x0045` | Seeed Studio |

### 2. Hardware Customizado (PCBs próprias)

Para PCBs que integram o firmware OpenKey, o projeto utiliza o
[pid.codes](https://pid.codes) — um registro comunitário gratuito de
VID/PID para projetos open-source.

| Campo | Valor |
|-------|-------|
| **VID** | `0x1209` (comunidade pid.codes) |
| **PID** | `0x4F4B` (hex de "OK" — OpenKey) |
| **Registro** | [pid.codes/1209/4F4B](https://pid.codes/1209/4F4B) |
| **Submissão** | [`tools/pidcodes/`](../../tools/pidcodes/) |

## Como registrar um novo PID no pid.codes

1. Fork o repo [`pidcodes/pidcodes.github.com`](https://github.com/pidcodes/pidcodes.github.com)
2. Crie `org/<sua-organizacao>/index.md` (perfil)
3. Crie `VID/1209/<hex>/index.md` (registro do PID)
4. Abra um Pull Request para revisão

## Referências

- [pid.codes](https://pid.codes) — registro comunitário
- [USB-IF](https://usb.org) — autoridade oficial de VID/PID
- [Board Profiles](boards/profiles/) — valores VID/PID por placa
