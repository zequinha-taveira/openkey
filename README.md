# OpenKey

OpenKey é um framework open-source universal para chaves de segurança FIDO2/WebAuthn. O projeto é software-first: não exige uma PCB própria e usa placas de desenvolvimento como plataformas de execução.

## Estrutura

```text
openkey/
├── crates/                  # Núcleo Rust modular
│   ├── openkey-core/        # Estado e políticas do autenticador
│   ├── openkey-crypto/      # RNG e interfaces criptográficas
│   ├── openkey-vault/       # Contrato de armazenamento de chaves
│   ├── openkey-protocol-fido/ # Comandos CTAP2 e GetInfo
│   ├── openkey-protocol-apdu/ # Parsing de cabeçalho APDU
│   ├── openkey-usb/         # Transporte bidirecional
│   └── openkey-update/      # Manifesto e anti-rollback inicial
├── simulator/               # Transporte em memória e target nativo
├── boards/                  # Perfis de placas
├── tools/openkey-cli/       # CLI oficial
├── apps/desktop/            # Aplicação desktop futura
├── tests/                   # Contratos e interoperabilidade
├── docs/                    # Arquitetura, segurança e desenvolvimento
└── .github/workflows/       # CI
```

## Começar

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo run -p openkey-simulator
cargo run -p openkey-cli -- info
```

A implementação evolui de forma incremental: primeiro contratos e simulador, depois CBOR/CTAP completos, targets de hardware e integrações host.

## Princípios

- separação entre domínio, protocolos, transporte e hardware;
- Rust seguro por padrão e falha fechada;
- nenhuma dependência de uma placa específica no núcleo;
- testes de unidade, contrato e interoperabilidade desde o início;
- mudanças estruturais pequenas, revisáveis e documentadas.

Consulte [`docs/architecture.md`](docs/architecture.md), [`docs/threat-model.md`](docs/threat-model.md) e [`docs/security-policy.md`](docs/security-policy.md).
