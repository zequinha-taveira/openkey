# ADR-0010: Reestruturação do Monorepo OpenKey

| Campo     | Valor                                      |
|-----------|--------------------------------------------|
| **ID**    | ADR-0010                                   |
| **Data**  | 2026-07-27                                 |
| **Status**| Aceito                                     |
| **Autores**| OpenKey Team                              |

---

## Contexto

O layout original do monorepo colocava os crates de firmware (`core/`, `platform/`,
`protocols/`, `crypto/`, `storage/`) diretamente na raiz do repositório.
À medida que o projeto cresce para suportar múltiplos MCUs, novas camadas de firmware
(`usb/`, `config/`, `boot/`) e um ecossistema de ferramentas host mais rico, esse
layout plano tornava-se difícil de navegar e contradiz a separação de responsabilidades
entre código embarcado, software host, ferramentas e dados de configuração.

Adicionalmente, `boards/rp2350/` era um crate Rust, quando o conceito de
"Board Profile" é puramente de dados — não código.

## Decisão

Adotar o seguinte layout estruturado para o monorepo:

```
openkey/
├── firmware/        # Todo código embarcado (no_std)
│   ├── core/
│   ├── platform/
│   │   └── mcu/    # Implementações por MCU
│   ├── protocols/
│   ├── storage/
│   ├── crypto/
│   ├── usb/         # NOVO
│   ├── config/      # NOVO
│   └── boot/        # NOVO
├── boards/          # Apenas perfis YAML (sem código Rust)
│   ├── profiles/
│   ├── templates/
│   └── examples/
├── host/            # Software host
│   ├── sdk-python/  # (era sdk/)
│   ├── cli/
│   ├── configurator/ # (era gui/)
│   ├── provisioner/ # NOVO
│   ├── updater/     # NOVO
│   └── diagnostics/ # NOVO
├── tools/           # Ferramentas internas
│   ├── manufacturing/
│   ├── migration/
│   ├── scripts/
│   ├── generators/
│   └── simulator/   # (era host/simulator/)
├── tests/           # Testes por objetivo
│   ├── unit/
│   ├── integration/ # (era host/tests/)
│   ├── interoperability/
│   ├── hardware/
│   └── regression/
├── cmake/           # NOVO
└── packaging/       # NOVO
```

### Principais mudanças

1. **`firmware/`**: Agrupa todos os crates Rust `no_std` sob um diretório pai claro.
2. **`boards/`**: Convertido de crate Rust para repositório de perfis YAML puros.
3. **`host/sdk/` → `host/sdk-python/`**: Nome explicita a linguagem alvo.
4. **`host/gui/` → `host/configurator/`**: Nome descreve responsabilidade, não tecnologia.
5. **`host/simulator/` → `tools/simulator/`**: O simulador é uma ferramenta de desenvolvimento, não um produto host.
6. **`host/tests/` → `tests/integration/`**: Testes migram para a suíte centralizada `tests/`.
7. **Novos crates firmware**: `usb/`, `config/`, `boot/` — separação de responsabilidades no firmware.
8. **Novos diretórios host**: `provisioner/`, `updater/`, `diagnostics/`.
9. **`cmake/` e `packaging/`**: Infraestrutura de build e distribuição.

## Consequências

### Positivas

- **Clareza de propósito**: `firmware/` contém exclusivamente código embarcado.
- **Escalabilidade**: Novos MCUs adicionam apenas `firmware/platform/mcu/<novo>/`.
- **Boards como dados**: Perfis YAML são editáveis sem recompilar.
- **Testes centralizados**: `tests/` unifica todos os objetivos de teste.
- **Host completo**: O `host/` cobre o ciclo completo de vida do dispositivo.

### Negativas / Trade-offs

- **Custo de migração**: Todos os `Cargo.toml` e imports precisam ser atualizados.
- **Histórico git**: A reestruturação aparece como remoção + adição (não `git mv` puro
  quando o sandbox bloqueia operações de ACL).
- **CI**: Workflows precisam referenciar novos paths.

## Alternativas Consideradas

1. **Manter o layout plano**: Rejeitado — não escala para múltiplos MCUs e ferramentas.
2. **Workspace por domínio separado**: Rejeitado — perde os benefícios de build unificado.
3. **Mover apenas firmware/**: Rejeitado — a reorganização parcial cria inconsistência.

## Referências

- [ADR-0001](ADR-0001-rust.md) — Escolha de Rust como linguagem primária
- [ADR-0006](ADR-0006-build.md) — Sistema de build
- Proposta de layout discutida em: `Development Plan.md`, `Ecosystem.md`
