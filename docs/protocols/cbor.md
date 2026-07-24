# Codificação CBOR Canônica (RFC 8949) (`docs/protocols/cbor.md`)

## 📦 Regras de Decodificação e Codificação CBOR

O OpenKey exige conformidade estrita com as regras de **Canonical CBOR Encoding** (FIDO2 / CTAP2 Standard).

## 🔒 Restrições Importantes de Segurança

1. **Ordenação de Chaves de Maps**: As chaves de mapas CBOR devem ser ordenadas pelo comprimento em bytes e depois por ordem lexicográfica de seus bytes.
2. **Campos Desconhecidos**: Campos desconhecidos em comandos de entrada são rejeitados com o erro `CTAP2_ERR_INVALID_CBOR`.
3. **Ausência de Recursão Aberta**: O parser de CBOR não utiliza chamadas recursivas ilimitadas para evitar estouro da pilha de execução (Stack Overflow).
