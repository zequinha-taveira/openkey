# OpenKey Git Branch Strategy

## Branches Permanentes

```text
main      → versão estável

develop   → integração contínua do desenvolvimento
```

---

# Fluxo de Trabalho

```text
main
 │
 ├──────────────┐
 │              │
 ▼              │
develop         │
 │              │
 ├── feature/*  │
 ├── fix/*      │
 ├── docs/*     │
 ├── refactor/* │
 ├── security/* │
 └── test/*     │
```

---

# Branch `main`

Contém apenas versões aprovadas.

Características:

* compilando;
* testes aprovados;
* documentação sincronizada;
* sem funcionalidades incompletas;
* pronta para release.

Nunca realizar desenvolvimento direto nesta branch.

---

# Branch `develop`

Branch principal de desenvolvimento.

Características:

* integração contínua;
* novas funcionalidades;
* correções;
* refatorações;
* atualizações de documentação.

Todo o trabalho converge para esta branch antes de chegar à `main`.

---

# Branches Temporárias

## feature/

Nova funcionalidade.

Exemplo:

```text
feature/ctap2-large-blobs
```

---

## fix/

Correção de bug.

```text
fix/usb-timeout
```

---

## security/

Correções de segurança.

```text
security/pin-validation
```

---

## docs/

Alterações de documentação.

```text
docs/testing-guide
```

---

## refactor/

Mudanças estruturais sem alterar comportamento.

```text
refactor/storage-layer
```

---

## test/

Infraestrutura de testes.

```text
test/mock-usb
```

---

# Critérios para Merge em `develop`

* código compilando;
* testes executados;
* documentação atualizada;
* revisão técnica concluída;
* nenhuma pendência crítica.

---

# Critérios para Merge em `main`

* fase aprovada;
* testes automatizados aprovados;
* testes de integração aprovados;
* revisão de segurança concluída;
* CHANGELOG atualizado;
* VERSION_HISTORY atualizado;
* documentação sincronizada.

---

# Releases

Cada release deve ser criada a partir da `main`.

Exemplos:

* v0.1.0
* v0.2.0
* v0.3.0
* v1.0.0

Cada tag deve corresponder a uma versão documentada.

---

# Proteção das Branches

## main

* Pull Request obrigatório.
* Revisão obrigatória.
* CI obrigatória.
* Sem commits diretos.

## develop

* Pull Request recomendado.
* CI obrigatória.
* Testes obrigatórios antes do merge.

---

# Evolução Futura (Manutenção Paralela)

Caso o OpenKey venha a necessitar de suporte a múltiplas versões em paralelo (ex.: manutenção da v1.x enquanto a v2.x é desenvolvida em `develop`), a estrutura atual pode ser estendida de forma transparente sem alterar o fluxo básico:

- `release/vX.Y`: Preparação de releases e estabilização de versão específica.
- `hotfix/*`: Correções críticas em versões de manutenção já lançadas na `main`.

---

# Filosofia

A branch `main` representa o estado estável do OpenKey.

A branch `develop` representa a próxima versão do projeto.

Nenhuma funcionalidade é considerada concluída até ser integrada à `develop`, validada e posteriormente promovida para a `main`.
