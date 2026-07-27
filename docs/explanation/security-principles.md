# Princípios de Segurança

## Princípios Fundamentais

1. **Fail Closed** - Erros invalidam sessões
2. **Zeroização de Memória** - Chaves e PINs são limpos
3. **Execução em Tempo Constante** - Mitiga side-channels
4. **Código Seguro por Padrão** - Safe Rust, unsafe com justificativa
5. **Validação de Entrada** - Todos os dados são validados

## Boas Práticas

- Nunca desabilite verificações de bounds
- Nunca ignore erros
- Nunca utilize credenciais fixas
- Nunca introduza comportamento indefinido

## Referências

- [Threat Model](threat-model.md)
- [ADR-0004: Unsafe Code Policy](../security/unsafe-policy.md)