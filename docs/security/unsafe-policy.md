# Política de Código Unsafe (`docs/security/unsafe-policy.md`)

## ⚠️ Requisitos Obrigatórios para `unsafe` Rust

Devido aos riscos de segurança de memória, o uso da palavra-chave `unsafe` no OpenKey é estritamente controlado.

## 📋 Regras de Auditoria

1. **Justificativa Explicita**: Todo bloco `unsafe` DEVE conter um comentário `// SAFETY:` imediatamente acima do bloco, detalhando as invariantes garantidas pelo chamador.
2. **Revisão Obrigatória por 2 Mantenedores de Segurança**: Mudanças que adicionem ou modifiquem código `unsafe` exigem aprovação explícita do Comitê de Segurança.
3. **Isolamento em Abstrações Seguras**: Código `unsafe` deve ser encapsulado em APIs puramente seguras com verificações estáticas de limites.

Consulte [`docs/adr/ADR-0004-unsafe.md`](../adr/ADR-0004-unsafe.md) para o histórico da decisão sobre `unsafe`.
