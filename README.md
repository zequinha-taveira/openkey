# OpenKey Rust Framework

Framework open-source para autenticadores e dispositivos de segurança em Rust. O workspace
ativo é modular, orientado a `no_std`, independente de MCU e testável no host.

> O projeto não declara certificação FIDO, FIPS, Common Criteria ou resistência a ataques
> físicos. As garantias dependem da plataforma, dos adaptadores e de validação formal.

## Estrutura do repositório

```text
openkey/
├── crates/                 # Bibliotecas centrais independentes de MCU
│   ├── openkey-core/       # Estados, políticas e sessões
│   ├── openkey-crypto/     # Interfaces criptográficas
│   ├── openkey-vault/      # Cofre lógico e identificadores opacos
│   ├── openkey-protocol-fido/
│   ├── openkey-protocol-apdu/
│   ├── openkey-usb/        # Contratos de transporte
│   └── openkey-update/     # Política de atualização assinada
├── boards/                # Perfis e futuros adaptadores de hardware
├── tools/openkey-cli/     # Ferramenta de linha de comando
├── apps/desktop/          # Aplicação desktop planejada
├── tests/                 # Simulador, protocolos e interoperabilidade
├── docs/                  # Arquitetura, segurança e documentação Diátaxis
├── legacy/                # Implementação anterior, fora do workspace ativo
└── .github/               # CI e automações
```

## Desenvolvimento

```bash
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

A partir da raiz, `cargo` opera somente sobre as crates em `crates/` e a CLI. O código
legado foi preservado em `legacy/` para facilitar auditoria e migração incremental.

## Documentação

- [Arquitetura](docs/architecture.md)
- [Modelo de ameaças](docs/threat-model.md)
- [Política de segurança](docs/security-policy.md)
- [Placas suportadas](docs/supported-boards.md)
- [Decisões arquiteturais](docs/decisions/README.md)
- [Índice completo da documentação](docs/README.md)

## Licença

Apache License 2.0 ou MIT. Consulte [LICENSE](LICENSE).
