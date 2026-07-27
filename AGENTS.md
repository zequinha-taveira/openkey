# Diretrizes para Agentes de IA e Automação (AGENTS.md)

Este documento estabelece as regras e convenções para agentes de Inteligência Artificial e sistemas automatizados que contribuem ou modificam o repositório OpenKey.

## 🎯 Regras Globais

1. **Segurança em Primeiro Lugar**: O OpenKey é uma chave de segurança de hardware. Códigos que afetem criptografia, manipulação de chaves, parsing de protocolo CBOR ou gerenciamento de memória em Rust **nunca** devem ignorar verificações de limite (*bounds checks*) ou introduzir blocos `unsafe` sem justificação explícita registrada em um ADR.
2. **Respeito aos ADRs**: Antes de sugerir refatorações estruturais em pacotes como `firmware` ou `host/sdk`, consulte os [ADRs (Architecture Decision Records)](docs/reference/adr/README.md).
3. **Manutenção da Documentação**: Qualquer alteração em APIs públicas ou protocolos deve vir acompanhada da atualização correspondente em `docs/reference/api/` e `docs/reference/protocols/`.

## 🛠️ Convenções de Código Rust

- **Firmware (`no_std`)**: Deve ser mantido livre de alocações dinâmicas não determinísticas na heap sempre que possível.
- **Tratamento de Erros**: Utilize tipos de erro fortemente tipados (`enum Error`) em vez de panics (`panic!`, `unwrap()`, `expect()`) no caminho de execução de produção do firmware.
- **Uso de `unsafe`**: Cada bloco `unsafe` exige um comentário `// SAFETY:` justificando as invariantes de segurança mantidas, em conformidade com [`docs/security/unsafe-policy.md`](docs/security/unsafe-policy.md).

## 🧪 Verificação Obrigatória

Antes de considerar uma modificação como finalizada:
- Execute `cargo fmt --check`
- Execute `cargo clippy --all-targets -- -D warnings`
- Execute a suíte de testes: `cargo test --workspace`
## AI Context References
- Documentation index: `.context/docs/README.md`
- Agent playbooks: `.context/agents/README.md`

