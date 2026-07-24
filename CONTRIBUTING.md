# Guia de Contribuição (CONTRIBUTING.md)

Obrigado por seu interesse em contribuir para o projeto **OpenKey**! Como um projeto de segurança de hardware de código aberto, valorizamos contribuições de código, documentação, revisão de arquitetura e pesquisas de segurança.

## 📋 Como Contribuir

1. **Abra um Issue ou Discuta**: Para alterações significativas ou novas funcionalidades, crie uma discussão ou issue antes de iniciar o código.
2. **Fork e Branch**: Faça um fork do repositório e crie uma branch com nome descritivo (`feature/meu-recurso` ou `fix/correcao-bug`).
3. **Padrões de Código**:
   - Respeite o guia de estilo em [`docs/development/coding-style.md`](docs/development/coding-style.md).
   - Mantenha a documentação atualizada conforme [`docs/README.md`](docs/README.md).
4. **Commits**: Escreva mensagens de commit claras e padronizadas (ex: `feat(firmware): adiciona suporte a HMAC-secret`).

## 🧪 Processo de Pull Request (PR)

- Todo PR passa por testes automatizados de CI em `.github/workflows/ci.yml`.
- É necessária aprovação de pelo menos 2 mantenedores para código de firmware ou segurança.
- Certifique-se de que `cargo test` passa localmente sem erros ou warnings.

Para instruções completas sobre ambiente de desenvolvimento, depuração e simulação, veja [`docs/development/contributing.md`](docs/development/contributing.md).
