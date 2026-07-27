# ADR-0003: Implementação de Pilha USB HID e Multiplexação de Canais

- **Status**: Aceito
- **Data**: 2026-07-24
- **Autores**: Equipe de Transporte

## 📌 Contexto

A especificação FIDO2 CTAPHID exige enquadramento de relatórios USB HID de 64 bytes e gerenciamento de múltiplos canais lógicos simultâneos para evitar travamentos durante a comunicação com múltiplos processos no sistema operacional host.

## 💡 Decisão

Implementaremos uma pilha **CTAPHID em Rust** pura, sem dependências de frameworks externos não auditados, com suporte a alocação dinâmica de IDs de canais (`CID`) de 32 bits e buffers de reconstrução de pacotes isolados por canal.

## 🚀 Consequências

### Positivas
- Resposta determinística e baixo jitter de pacotes.
- Isolamento total entre requisições de diferentes aplicações host.

### Compromissos (Trade-offs)
- Requer gerenciamento cuidadoso de timeouts de inatividade para liberar canais abandonados.
