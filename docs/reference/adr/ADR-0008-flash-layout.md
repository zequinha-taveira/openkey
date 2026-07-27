# ADR-0008: Layout de Memória Flash e Bootloader Dual-Bank com Assinatura Assimétrica

- **Status**: Aceito
- **Data**: 2026-07-24
- **Autores**: Equipe de Firmware e Bootloader

## 📌 Contexto

Atualizações seguras de firmware no ambiente de campo (Device Firmware Update - DFU) exigem que a chave não fique inutilizável caso ocorra uma perda imprevisível de energia durante o processo de gravação.

## 💡 Decisão

Implementaremos uma estrutura de **Memória Flash Dual-Bank (Bank A / Bank B)** acompanhada por um **Bootloader Imutável** com verificação de assinatura digital ECDSA P-256 antes da troca de execução de banco.

## 🚀 Consequências

### Positivas
- Mecanismo à prova de falhas (*fail-safe DFU*): rollback automático se a nova imagem apresentar falhas de integridade.
- Bloqueio definitivo contra execução de firmwares adulterados não assinados.

### Compromissos (Trade-offs)
- Divisão do espaço total de Flash disponível pela metade para comportar ambas as imagens em paralelo.
