# Como criar e evoluir o OpenKey

O OpenKey é um framework open-source universal para chaves de segurança FIDO2/WebAuthn. Ele é desenvolvido em Rust para oferecer segurança de memória, separação entre protocolo e hardware e robustez criptográfica.

O projeto é **software-first**: não exige uma PCB própria. O desenvolvimento começa no simulador e usa placas comerciais como plataformas de execução.

## 1. O que será criado

Uma implementação OpenKey é composta por:

- **protocolos e domínio** em `protocol/`;
- **serviços embarcados** em `firmware/`;
- **um target**, como `firmware/platform/mcu/rp23xx/`;
- **um simulador nativo** em `simulator/`;
- **ferramentas host** em `host/`;
- **perfis de placa** em `boards/`;
- **testes e fuzzing** em `tests/` e `fuzz/`.

A regra é simples: o protocolo não conhece o MCU, e o target não reimplementa o protocolo.

## 2. Pré-requisitos

No Linux, instale:

- Rust estável e Cargo;
- Python 3.11 ou superior;
- `pytest` e as dependências de `requirements-dev.txt`;
- ferramentas USB/HID necessárias ao seu sistema;
- uma placa compatível apenas quando for testar hardware real.

Clone o repositório e entre nele:

```bash
git clone https://github.com/zequinha-taveira/openkey.git
cd openkey
```

## 3. Validar a base antes de alterar código

Execute o gate do repositório e a suíte Rust:

```bash
./scripts/check-repository-layout.sh
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets \
  --exclude openkey-target-rp2350 -- -D warnings
```

O target RP2350 é validado separadamente porque é firmware embarcado e não deve ser compilado como um binário host comum.

Para os pacotes Python:

```bash
python3 -m venv .venv
. .venv/bin/activate
pip install -r requirements-dev.txt
python -m pytest host/sdk-python host/diagnostics host/gui host/cli host/provisioner -q
```

## 4. Começar pelo simulador

O simulador é o caminho recomendado para criar funcionalidades. Ele permite testar sem placa física e executa em CI:

```bash
cargo build -p openkey-simulator
cargo test -p openkey-simulator
cargo run -p openkey-simulator
```

Ao adicionar um comportamento, implemente primeiro o contrato de plataforma que o simulador consegue fornecer. Depois, implemente a mesma interface no target de hardware.

## 5. Implementar uma funcionalidade de protocolo

Para uma nova operação FIDO2/WebAuthn:

1. defina o formato e os erros em `protocol/core/`;
2. implemente codificação, decodificação e validação em `protocol/protocols/`;
3. defina limites de tamanho, profundidade e cardinalidade;
4. implemente o caminho criptográfico em `firmware/crypto/`;
5. use `firmware/storage/` para persistência segura, quando necessário;
6. conecte o serviço de plataforma sem importar detalhes de MCU para o protocolo;
7. adicione testes unitários e de integração;
8. atualize a referência correspondente em `docs/reference/`.

Não use `unwrap()`, `expect()` ou `unsafe` no caminho de produção sem justificativa e revisão de segurança.

## 6. Executar em uma placa de desenvolvimento

O OpenKey não define uma PCB própria. Escolha uma placa comercial compatível com o target, por exemplo uma placa da família RP23xx.

O fluxo geral é:

```bash
cargo check -p openkey-target-rp2350 \
  --target thumbv8m.main-none-eabihf
cargo build -p openkey-target-rp2350 \
  --target thumbv8m.main-none-eabihf
```

Antes de gravar uma placa real:

- confirme o perfil em `boards/`;
- confira memória, clock, RNG, USB e armazenamento;
- valide o boot seguro e o mecanismo de recuperação;
- execute os testes de hardware em `tests/hardware/`;
- registre limitações do target na documentação.

## 7. Criar suporte a uma nova família de MCU

Não copie o protocolo para uma nova pasta. Faça o seguinte:

1. crie a implementação em `firmware/platform/mcu/<familia>/`;
2. implemente os contratos de `firmware/platform/`;
3. adicione um perfil declarativo em `boards/profiles/<familia>/`;
4. adicione template e exemplo em `boards/templates/` e `boards/examples/`;
5. adicione um target Cargo somente quando houver código compilável;
6. crie um build check e testes mínimos para o target;
7. documente diferenças de memória, RNG, USB, boot e armazenamento.

A lógica de CTAP2, CBOR, COSE e WebAuthn continua em `protocol/`.

## 8. Integrar com o computador

Use o SDK Python em `host/sdk-python/` como camada de comunicação. As demais ferramentas dependem dele:

- `host/cli/`: automação de terminal;
- `host/configurator/`: configuração;
- `host/provisioner/`: provisionamento;
- `host/updater/`: atualização;
- `host/diagnostics/`: diagnóstico;
- `host/gui/`: interface desktop.

Nenhuma ferramenta host deve ler chaves privadas ou acessar diretamente o armazenamento interno do dispositivo. Toda operação passa por contratos públicos e validação no autenticador.

## 9. Testar segurança e interoperabilidade

Use camadas diferentes de teste:

```bash
# Rust e simulador
cargo test --workspace

# Fuzzing
# consulte fuzz/README.md e os harnesses disponíveis

# Host
python -m pytest host/sdk-python host/diagnostics host/gui host/cli host/provisioner -q
```

Inclua casos para:

- CBOR não canônico e entradas truncadas;
- tamanhos acima dos limites;
- canais CTAPHID fora de ordem;
- credenciais corrompidas;
- falha de energia durante persistência;
- RNG indisponível ou repetitivo;
- atualização inválida, antiga ou não assinada;
- ausência de presença do usuário;
- compatibilidade com clientes WebAuthn reais.

## 10. Documentar a mudança

Toda alteração relevante deve atualizar:

- o documento de referência do protocolo ou da API;
- o modelo de ameaças, quando houver impacto de segurança;
- o ADR correspondente, quando houver mudança de contrato ou layout;
- `docs/planning/tasks.md` e `docs/planning/phases.md`, quando alterar o roadmap;
- os testes e o workflow de CI.

Não crie outra árvore como `openkey-rs/`, `crates/` ou `legacy/` para testar uma ideia. Use o layout atual e uma branch isolada.

## 11. Definition of Done

Uma funcionalidade OpenKey está pronta quando:

- compila no ambiente aplicável;
- passa por `cargo fmt` e Clippy;
- possui testes unitários e de integração adequados;
- funciona no simulador quando a funcionalidade é simulável;
- tem validação de limites e falhas seguras;
- não expõe material secreto em logs ou diagnósticos;
- está documentada em `docs/`;
- mantém os caminhos do Cargo e o CI verde.

Esse fluxo permite construir o OpenKey de forma incremental: primeiro o contrato e o simulador, depois o target de hardware e, por fim, as integrações host e os testes de interoperabilidade.
