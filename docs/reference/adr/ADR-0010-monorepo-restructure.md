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
├── protocol/        # Núcleo do protocolo (no_std, independente de MCU)
│   ├── core/        # Engine de segurança e CTAP2 (openkey-core)
│   └── protocols/   # CBOR, COSE, CTAP-HID, CTAP2, WebAuthn (openkey-protocols)
├── firmware/        # Todo código embarcado (no_std)
│   ├── platform/
│   │   └── mcu/    # Implementações por MCU
│   ├── storage/
│   ├── crypto/
│   ├── usb/
│   ├── config/
│   └── boot/
├── simulator/       # Simulador de software (openkey-simulator)
├── boards/          # Apenas perfis YAML (sem código Rust)
│   ├── profiles/
│   ├── templates/
│   └── examples/
├── host/            # Software host
│   ├── sdk-python/  # (era sdk/)
│   ├── cli/
│   ├── configurator/ # (era gui/)
│   ├── provisioner/
│   ├── updater/
│   └── diagnostics/
├── tools/           # Ferramentas internas
│   ├── manufacturing/
│   ├── migration/
│   ├── scripts/
│   └── generators/
├── tests/           # Testes por objetivo
│   ├── unit/
│   ├── integration/ # (era host/tests/)
│   ├── interoperability/
│   ├── hardware/
│   └── regression/
├── cmake/
└── packaging/
```

### Principais mudanças

1. **`firmware/`**: Agrupa todos os crates Rust `no_std` sob um diretório pai claro.
2. **`boards/`**: Convertido de crate Rust para repositório de perfis YAML puros.
3. **`host/sdk/` → `host/sdk-python/`**: Nome explicita a linguagem alvo.
4. **`host/gui/` → `host/configurator/`**: Nome descreve responsabilidade, não tecnologia.
5. **`host/simulator/` → `simulator/`**: O simulador é uma ferramenta de desenvolvimento, não um produto host. *(Emenda 2026-08-04: promovido de `tools/simulator/` para a raiz do monorepo.)*
6. **`host/tests/` → `tests/integration/`**: Testes migram para a suíte centralizada `tests/`.
7. **Novos crates firmware**: `usb/`, `config/`, `boot/` — separação de responsabilidades no firmware.
8. **Novos diretórios host**: `provisioner/`, `updater/`, `diagnostics/`.
9. **`cmake/` e `packaging/`**: Infraestrutura de build e distribuição.

### Emenda (2026-08-04) — Separação protocol / firmware / simulador

10. **Novo `protocol/`**: O núcleo do protocolo (`openkey-core` e `openkey-protocols`) sai de `firmware/` para `protocol/`, separando código de protocolo (independente de MCU) do firmware embarcado.
11. **`tools/simulator/` → `simulator/`**: O simulador é promovido a diretório raiz, reforçando a separação em cinco áreas: núcleo do protocolo, firmware Rust, simulador, testes e ferramentas.
12. **`firmware/`**: Passa a conter exclusivamente firmware embarcado (`platform/`, `storage/`, `crypto/`, `usb/`, `config/`, `boot/`).

## Consequências

### Positivas

- **Clareza de propósito**: `firmware/` contém exclusivamente código embarcado.
- **Separação protocol/firmware**: O núcleo do protocolo é reutilizável e testável independentemente de MCU.
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
