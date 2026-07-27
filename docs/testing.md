# Estratégia de Testes do OpenKey

## 🧪 Tipos de Testes

### Testes Unitários
- Executados com `cargo test --workspace`
- Cobrem unidades individuais de código
- Foco em lógica de protocolo e segurança

### Testes de Integração
- Testes de comunicação entre componentes
- Verificação de fluxos de credenciais

### Fuzzing
- Testes de fuzzing para parsers CBOR e CTAP2
- Usam `cargo-fuzz` ou libFuzzer
- Alvo: `host/simulator/`

### Testes de Hardware
- Testes no dispositivo RP2350 real
- Validação de comportamento em diferentes condições

## 📋 Comando de Teste

```bash
# Executar todos os testes
cargo test --workspace

# Executar testes específicos
cargo test --package openkey-core

# Executar fuzzing
cargo fuzz run <target>
```

## 🔍 Cobertura de Código

- Meta: >90% de cobertura em código crítico
- Ferramentas: tarpaulin, llvm-cov

## 📖 Documentação Relacionada

- [Roadmap de Desenvolvimento](development/roadmap.md)
- [CI](development/ci.md)