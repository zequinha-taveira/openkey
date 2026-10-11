# Arquitetura

O monorepo separa domínio, criptografia, vault, protocolos, transporte USB e atualização. Os crates reutilizáveis em `crates/` são independentes de MCU e podem ser compilados como bibliotecas `no_std` sem conhecer uma placa, registrador, HAL ou SDK específico.

## Regra de independência de hardware

Nenhum crate reutilizável pode depender de:

- uma família de MCU (`RP23xx`, STM32, nRF ou ESP32);
- registradores ou endereços de memória de uma placa;
- um driver USB específico de hardware;
- um perfil de `boards/` ou SDK de fabricante;
- uma implementação de boot, flash ou GPIO.

Esses detalhes pertencem a adaptadores de target futuros. O core depende apenas de contratos como `RngProvider`, `KeyStore` e `Transport`. O simulador implementa os mesmos contratos em ambiente nativo.

A regra é verificada pelo gate `scripts/check-mcu-independence.sh` e executada pela CI.

## Fluxo inicial

```text
openkey-core → protocolos FIDO/APDU → USB/vault/update → adaptador de target
                                      ├── simulador
                                      └── placa comercial futura
```

A implementação de uma nova placa deve adicionar um adaptador, não duplicar a lógica FIDO2/WebAuthn.
