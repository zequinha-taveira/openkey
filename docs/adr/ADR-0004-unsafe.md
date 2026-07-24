# ADR-0004: Política Estrita de Isolamento e Auditoria de Código `unsafe`

- **Status**: Aceito
- **Data**: 2026-07-24
- **Autores**: Comitê de Segurança OpenKey

## 📌 Contexto

Embora o Rust ofereça garantias de segurança de memória por padrão, interações com registradores de hardware (MMIO) e certas instruções de montagem inline exigem o uso da instrução `unsafe`.

## 💡 Decisão

Estabelecemos a regra de **Zero Unsafe Arbitrário**: todo e qualquer bloco `unsafe` no firmware deve ser explicitamente encapsulado em uma API segura (`safe wrapper`), documentado com comentários `// SAFETY:` verificáveis e auditado por dois membros do comitê de segurança.

## 🚀 Consequências

### Positivas
- Redução drástica da superfície de potenciais vulnerabilidades de memória.
- Facilidade de auditoria e revisão de segurança automatizada.

### Compromissos (Trade-offs)
- Maior rigor na aceitação de Pull Requests que toquem a camada HAL.
