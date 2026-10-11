# Organização prática do repositório

**Status:** proposta aplicada sem mudança de caminhos do Cargo  
**Objetivo:** deixar claro o que é código, documentação, ferramenta e contexto de automação sem criar uma árvore paralela.

## 1. Decisão de segurança

A organização deve preservar os caminhos usados pelo workspace Cargo e pelos pacotes Python. Portanto, esta etapa **não move crates, firmware, targets ou pacotes host**. Qualquer migração futura exige uma mudança explícita de manifesto, atualização de imports, testes e um PR separado.

## 2. Inventário por responsabilidade

### Código de produção

| Área | Caminho canônico | Regra |
|---|---|---|
| Protocolos e domínio | `protocol/core/`, `protocol/protocols/` | Independente de MCU; participa do workspace Cargo |
| Firmware | `firmware/` | Serviços embarcados, HAL, crypto, storage, USB, config e boot |
| Target de referência | `firmware/platform/mcu/rp23xx/` | Código específico do RP23xx |
| Simulador | `simulator/` | Target nativo do workspace; não é pacote host |
| Placas | `boards/` | Perfis e metadados; não duplicar lógica Rust |
| Software host | `host/` | Seis pacotes Python independentes com dependências locais |

### Verificação e operação

| Área | Caminho canônico | Regra |
|---|---|---|
| Testes | `tests/` | Organizados por objetivo: unit, integration, interoperability, regression, hardware |
| Fuzzing | `fuzz/` | Entradas malformadas e harnesses; não importar em produção |
| Scripts | `scripts/` | Automação do repositório e CI |
| Ferramentas | `tools/` | Geradores, manufacturing, migration e scripts especializados |
| Empacotamento | `packaging/` | Artefatos de distribuição, separados do código-fonte |
| Dependências de terceiros | `third_party/` | Código externo ou adaptadores, com licença registrada |

### Documentação

| Categoria | Caminho | Uso |
|---|---|---|
| Entrada do projeto | `README.md`, `../reference/ecosystem.md`, `../explanation/product-overview.md` | Visão, escopo e links para os detalhes |
| Governança | `CONTRIBUTING.md`, `GOVERNANCE.md`, `SECURITY.md`, `RELEASING.md` | Regras e processos do projeto |
| Planejamento | `../planning/development-plan.md`, `../planning/phases.md`, `../planning/tasks.md` | Roadmap, fases e tarefas rastreáveis |
| Tutoriais | `docs/tutorials/` | Aprendizado passo a passo |
| Procedimentos | `docs/how-to/` | Build, flash, provisionamento e release |
| Referência | `docs/reference/` | APIs, protocolos, arquitetura e ADRs |
| Explicação | `docs/explanation/` | Contexto, decisões, segurança e conceitos |
| Diagramas | `docs/diagrams/` | Fontes editáveis de diagramas |

Os arquivos de raiz permanecem reservados aos pontos de entrada do GitHub e à governança. Os documentos de produto, arquitetura e planejamento ficam em `docs/`; links canônicos evitam manter o mesmo texto em dois lugares.

### Contextos de IA e automação

| Área | Caminho | Tratamento |
|---|---|---|
| Contexto compartilhado do projeto | `.context/` | Fonte principal para agentes, políticas, skills e contexto do projeto |
| Agentes Claude | `.claude/` | Integração específica do Claude; preservar até existir migração validada |
| Agentes OpenCode | `.opencode/` | Prompts e playbooks específicos do OpenCode; não misturar com código de produção |
| Contexto gerado | `target/` | Saída de build local; não versionar nem documentar como código |

Essas áreas não fazem parte do workspace Cargo e não devem ser movidas para `docs/` apenas para deixar a árvore visualmente uniforme. Elas devem continuar claramente identificadas como configuração de ferramentas.

## 3. Duplicações e inconsistências encontradas

- `docs/explanation/product-overview.md`, `docs/reference/ecosystem.md` e `docs/planning/development-plan.md` são os documentos estratégicos canônicos; os arquivos em `docs/explanation/` são resumos ou explicações Diátaxis, não cópias byte a byte.
- `README.md`, `../reference/ecosystem.md` e documentos antigos continham exemplos de caminhos conceituais como `core/`, `platform/`, `host/sdk` e `host/simulator`. Os caminhos implementados são `protocol/core/`, `firmware/platform/`, `host/sdk-python/` e `simulator/`.
- `.context/`, `.claude/` e `.opencode/` têm finalidades diferentes. Não há justificativa segura para fundi-los ou apagar agentes sem confirmar os consumidores.
- A árvore `legacy/`, as crates paralelas em `crates/` e o diretório `openkey-rs/` não fazem parte do layout canônico e não devem ser reintroduzidos.

## 4. Mudanças práticas aplicadas nesta etapa

1. Corrigidos mapas e links de documentação para usar os caminhos reais.
2. Criado o mapa canônico em [`docs/reference/ecosystem-architecture.md`](../reference/ecosystem-architecture.md).
3. Criado `scripts/check-repository-layout.sh` para validar diretórios, manifests e ausência de árvores paralelas.
4. O CI executa esse gate antes de formatar e testar Rust.
5. Nenhum caminho de crate, pacote Python ou target foi movido.

## 5. Processo para mudanças futuras

Antes de mover qualquer diretório:

1. registrar os consumidores com `rg` e `cargo metadata`;
2. alterar o manifesto e os imports na mesma mudança;
3. manter compatibilidade temporária quando houver usuários externos;
4. executar o gate de layout, `cargo fmt`, `cargo test`, Clippy e testes host;
5. atualizar README, referência técnica, ADR e scripts de release;
6. separar migrações de código de reorganizações puramente documentais.

A organização correta é a que reduz ambiguidade sem aumentar risco de build.
