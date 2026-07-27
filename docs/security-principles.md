# Princípios de Segurança do OpenKey

## 🛡️ Princípios Fundamentais

1. **Fail Closed**: Diante de qualquer erro de parsing ou exceção inesperada, a sessão é invalidada imediatamente e o sistema retorna ao estado seguro.

2. **Zeroização de Memória**: Estruturas que mantêm chaves privadas, PINs ou tokens de sessão implementam `Zeroize` no encerramento de escopo.

3. **Execução em Tempo Constante**: Comparações de segredos, hashes e tokens de sessão devem ser estritamente em tempo constante (`subtle::ConstantTimeEq`).

4. **Código Seguro por Padrão**: Todo o `openkey-core` é escrito em Safe Rust. O uso de `unsafe` é restrito à PAL/HAL.

5. **Validação de Entrada**: Todos os dados de entrada são validados rigorosamente antes de processamento.

## 🔐 Boas Práticas

- Nunca desabilite verificações de bounds
- Nunca ignore erros
- Nunca utilize credenciais fixas
- Nunca introduza comportamento indefinido

## 📖 Documentação Relacionada

- [Threat Model](threat-model.md) - Modelo de Ameaças (STRIDE)
- [Secure Development](security/secure-development.md) - Práticas de Desenvolvimento Seguro
- [Unsafe Policy](security/unsafe-policy.md) - Política Estrita para Código `unsafe`
- [Cryptography](security/cryptography.md) - Primitivas e Algoritmos Criptográficos