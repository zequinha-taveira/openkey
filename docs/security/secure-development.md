# Desenvolvimento Seguro (`docs/security/secure-development.md`)

## 🛠️ Práticas de Código Seguro

- **Limpeza Automática de Memória**: Uso da crate `zeroize` para zerar buffers contendo chaves privadas, segredos temporários e hashes de PIN imediatamente após o uso.
- **Operações em Tempo Constante**: Todas as verificações de hash, assinaturas e utilitários de comparação de arrays operam em tempo constante para evitar ataques de canal lateral baseados em tempo (*timing attacks*).
- **Proibição de Desalocação Dinâmica na Heap**: O firmware `no_std` opera apenas com alocações de tamanho fixo em stack e buffers estáticos delimitados.
