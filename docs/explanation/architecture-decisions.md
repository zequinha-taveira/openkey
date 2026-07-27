# Decisões de Arquitetura

## Princípios

1. **Firmware Universal** - Um firmware por arquitetura de MCU
2. **HAL Separado** - Hardware Abstraction Layer independente
3. **Board Profile** - Dados parametrizáveis da placa
4. **Device Profile** - Dados do dispositivo físico
5. **Configuração Persistente** - Gerenciada pelo Configuration Manager

## ADRs

Veja `docs/reference/adr/` para registros completos.

## Decisões Chave

- **Rust `no_std`** - Para previsibilidade e segurança
- **Safe Rust** - Para prevenir vulnerabilidades de memória
- **Traits de Hardware** - Para portabilidade
- **CBOR Canônico** - Para compatibilidade FIDO2