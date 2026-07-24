# Modelo de Governança (GOVERNANCE.md)

O projeto **OpenKey** adere a um modelo de governança aberto, transparente e meritocrático, orientado pelo compromisso com a segurança e o padrão FIDO2/WebAuthn.

## 🏛️ Papéis no Projeto

### Contribuidores (Contributors)
Qualquer pessoa que submeta código, documentação, especificações, esquemas de hardware ou correções ao repositório.

### Mantenedores (Maintainers)
Membros da comunidade experientes que possuem acesso de escrita no repositório. São responsáveis por:
- Revisão e mesclagem de Pull Requests.
- Triagem de issues e acompanhamento da comunidade.
- Participação nas decisões de arquitetura via ADR.

### Comitê de Segurança (Security Steering Committee)
Grupo restrito responsável por gerenciar relatórios de vulnerabilidade recebidos sob a política de [`SECURITY.md`](SECURITY.md), assinar atualizações de firmware de produção e autorizar exceções de código em `unsafe-policy.md`.

## ⚖️ Tomada de Decisão

- Decisões técnicas triviais são resolvidas por consenso nos PRs.
- Mudanças arquiteturais relevantes exigem a redação de uma **ADR (Architecture Decision Record)** conforme detalhado em [`docs/adr/README.md`](docs/adr/README.md).
