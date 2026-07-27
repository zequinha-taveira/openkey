# Arquitetura do Firmware

## Estrutura

O firmware é compilado para alvos ARM Cortex-M utilizando Rust `no_std`.

## Fluxo de Execução

1. **Boot Sequencer**: Inicializa clock, TRNG e periféricos
2. **Loop de Eventos**: Processa interrupções USB/NFC
3. **Despachante CTAP2**: Processa comandos e verifica presença de usuário
4. **Resposta**: Gera credenciais e envia respostas CBOR

## Componentes

- **Security Core**: Lógica de protocolo e segurança
- **Platform Services**: Orquestração de hardware
- **HAL**: Abstrações de baixo nível