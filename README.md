# OpenKey

OpenKey é um framework open-source universal para chaves de segurança FIDO2/WebAuthn. O projeto é software-first: não exige uma PCB própria e usa placas de desenvolvimento como plataformas de execução.

## Estrutura

```text
openkey/
├── crates/                  # Núcleo Rust modular
│   ├── openkey-core/        # Domínio e políticas
│   ├── openkey-crypto/      # Interfaces criptográficas
│   ├── openkey-vault/       # Ciclo de vida e armazenamento de chaves
│   ├── openkey-protocol-fido/ # CTAP/FIDO
│   ├── openkey-protocol-apdu/ # APDU
│   ├── openkey-usb/         # Transporte USB
│   └── openkey-update/      # Atualizações assinadas
├── boards/                  # Perfis de placas
├── tools/openkey-cli/       # CLI oficial
├── apps/desktop/            # Aplicação desktop futura
├── tests/                   # Simulador, protocolo e interoperabilidade
├── docs/                    # Arquitetura, segurança e desenvolvimento
└── .github/workflows/       # CI
```

## Começar

```bash
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo run -p openkey-cli -- info
```

A implementação será incremental: primeiro contratos e testes, depois transportes, targets de hardware e integrações host.

## Princípios

- separação entre domínio, protocolos, transporte e hardware;
- Rust seguro por padrão e falha fechada;
- nenhuma dependência de uma placa específica no núcleo;
- testes de unidade, protocolo e interoperabilidade desde o início;
- mudanças estruturais pequenas, revisáveis e documentadas.

Consulte [`docs/architecture.md`](docs/architecture.md), [`docs/threat-model.md`](docs/threat-model.md) e [`docs/security-policy.md`](docs/security-policy.md).
