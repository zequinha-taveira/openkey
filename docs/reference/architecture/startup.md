# Startup do OpenKey

## Sequência de Inicialização

1. **Reset Vector**: Entry point do firmware
2. **Clock Init**: Configura clocks do sistema
3. **TRNG Init**: Inicializa gerador de números aleatóórios
4. **USB Init**: Configura stack USB
5. **Storage Init**: Inicializa armazenamento Flash
6. **Config Load**: Carrega configuração persistente
7. **Ready State**: Firmware pronto para comandos

## Estado

- **Unprovisioned**: Nenhuma configuração carregada
- **Provisioned**: Configuração válida carregada