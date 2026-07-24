# Plano de Desenvolvimento — OpenKey (`Development Plan.md`)

**Versão:** 1.0  
**Status:** Aprovado  
**Filosofia:** Desenvolvimento Incremental e Reutilização Universal de Código.

---

## 🎯 1. Filosofia de Desenvolvimento

O desenvolvimento do **OpenKey Framework** segue uma abordagem estritamente incremental. Cada fase estabelece uma camada de funcionalidade utilizável, testada e validada tanto na **implementação de referência RP2350** quanto no **Simulador de Software**.

### Princípios de Execução
- **Decoupled Architecture**: O núcleo de segurança (`Security Core`) e o protocolo (`CTAP2`) são desenvolvidos de forma totalmente independente de hardware.
- **Definition of Done Rigorosa**: Nenhum código é integrado à `main` sem testes unitários, validação de linting, documentação e aprovação em CI.
- **Simulador em Primeiro Lugar**: Novas funcionalidades de protocolo são validadas primeiro no simulador e depois portadas/testadas no hardware RP2350.

---

## 🚀 2. Fases de Implementação

### Fase 1: Infraestrutura do Framework e Monorepo
- **Escopo**:
  - Reorganização do monorepo, governança e pipeline de compilação em Rust (`Cargo.toml` workspace).
  - Estruturação do `openkey-core` (`no_std`), `openkey-pal` (traits), `targets/simulator` e `targets/rp2350`.
  - Configuração do pipeline de Integração Contínua (GitHub Actions) com linters e checadores estáticos.
- **Entrega**: O projeto compila com sucesso para os alvos `simulator` e `rp2350`.

---

### Fase 2: Camada de Abstração de Plataforma (PAL HAL)
- **Escopo**:
  - Definição das traits Rust da PAL: `RngProvider`, `FlashStorageProvider`, `UsbTransportProvider`, `GpioUserPresenceProvider`.
  - Implementação da PAL de Referência para o microcontrolador **RP2350** (drivers C/Rust Pico SDK, TinyUSB, TRNG de hardware).
  - Implementação da PAL para o **Simulador de Software** (emulação em memória e arquivos locais).
- **Entrega**: Dispositivo enumera como USB HID no RP2350 e inicializa socket/HID virtual no simulador.

---

### Fase 3: Camada de Transporte e Framing (CTAPHID)
- **Escopo**:
  - Implementação do protocolo de enquadramento `CTAPHID` no `openkey-core` agnóstico.
  - Alocação e gerenciamento de canais (`CTAPHID_INIT`, `CTAPHID_PING`, `CTAPHID_MSG`, `CTAPHID_CANCEL`, `CTAPHID_ERROR`).
  - Timeout de canal e remontagem de pacotes fragmentados.
- **Entrega**: Comunicação bi-direcional de mensagens CTAP funcional via USB HID no simulador e no RP2350.

---

### Fase 4: Núcleo do Protocolo CTAP2 (Engine CTAP2.1)
- **Escopo**:
  - Implementação do parser e serializador CBOR canônico estático (sem alocação na heap).
  - Comandos CTAP2 primários: `authenticatorGetInfo`, `authenticatorMakeCredential` (modo sem PIN) e `authenticatorGetAssertion` (modo sem PIN).
  - Validação estrita de opções (`rk`, `up`, `uv`).
- **Entrega**: Registros e autenticações WebAuthn funcionais em modo básico sem PIN.

---

### Fase 5: Suíte Criptográfica e Integridade TRNG
- **Escopo**:
  - Abstração da suíte criptográfica no `openkey-core` (`CryptoEngine` trait).
  - Suporte a curvas elípticas ECDSA P-256 / Ed25519 e hashes SHA-256.
  - Implementação de algoritmos de tempo constante (*constant-time execution*) para mitigar side-channel attacks.
  - Verificação contínua de entropia do TRNG (NIST SP 800-90B).
- **Entrega**: Geração de pares de chaves e assinaturas de credenciais criptograficamente válidas.

---

### Fase 6: Gerenciador de Armazenamento e Wear-Leveling
- **Escopo**:
  - Abstração de persistência segura (`StorageManager`).
  - Armazenamento de credenciais não-residentes (Key Handles criptografados) e credenciais residentes (Resident Keys - RK).
  - Manutenção atômica e monotônica do `Signature Counter` global.
  - Algoritmo de nivelamento de desgaste (*wear-leveling*) na Flash para preservação física do chip.
- **Entrega**: Persistência atômica de dados e prevenção contra perda de energia (*power-loss recovery*).

---

### Fase 7: Gestão de PIN e Protocolos de Segurança (`ClientPIN`)
- **Escopo**:
  - Implementação da especificação `authenticatorClientPIN` (`pinUvAuthProtocol` V1 e V2).
  - Troca de chaves ECDH e derivação de `pinUvAuthToken` efémero mantido estritamente em RAM zeroizada (`Zeroize`).
  - Proteção contra força bruta: retardo exponencial, contadores de tentativas e bloqueio definitivo (factory reset necessário).
- **Entrega**: Autenticação forte com proteção por PIN e tokens de autorização de sessão ativos.

---

### Fase 8: OpenKey Host SDK (Python / Rust Bindings)
- **Escopo**:
  - Construção da biblioteca client `openkey-sdk` em Python e bindings Rust.
  - Abstração da comunicação com dispositivos OpenKey (detecção USB HID / transporte simulador).
  - APIs de gerenciamento de credenciais, leitura de status, diagnósticos e automação.
- **Entrega**: SDK utilizável para integração por desenvolvedores e aplicações host.

---

### Fase 9: Interface de Linha de Comando (`openkey-cli`)
- **Escopo**:
  - Desenvolvimento do utilitário oficial `openkey-cli`.
  - Subcomandos: `info`, `pin set/change`, `credentials list/delete`, `reset`, `update-firmware`, `diagnostics`.
- **Entrega**: Ferramenta CLI oficial para administração completa da chave de segurança.

---

### Fase 10: Aplicação Desktop Graphic Manager (`openkey-gui`)
- **Escopo**:
  - Aplicação gráfica intuitiva **OpenKey Manager** para sistemas Windows, macOS e Linux.
  - Visualização gráfica de credenciais residentes, alteração de PIN, diagnóstico de integridade e atualizações de firmware com assistente visual.
- **Entrega**: Aplicação desktop gráfica pronta para usuários finais.

---

### Fase 11: Suporte Multi-Target e Matriz de Interoperabilidade
- **Escopo**:
  - Validação da PAL em novos alvos de hardware comunitários (ex: STM32, nRF52/53, ESP32).
  - Matriz de testes de interoperabilidade com navegadores (Chrome, Firefox, Edge, Safari) e sistemas operacionais (Windows Hello, macOS TouchID/WebAuthn, Linux PAM FIDO2).
- **Entrega**: Compatibilidade universal verificada e documentada.

---

### Fase 12: Hardening, Fuzzing, Auditoria e Release Candidate 1.0
- **Escopo**:
  - Fuzzing ostensivo das camadas CBOR, CTAPHID e CTAP2 no simulador via cargo-fuzz / libFuzzer.
  - Auditoria de código `unsafe` (conforme `unsafe-policy.md`), checagem Miri e otimização de binário.
  - Publicação da Release Candidate 1.0 e congelamento da API da PAL.
- **Entrega**: Versão 1.0 (GA) do OpenKey Universal Framework.

---

## 📏 3. Critérios de Qualidade e Definition of Done (DoD)

Para que qualquer Pull Request (PR) seja aceito no repositório OpenKey, ele DEVE satisfazer rigorosamente a seguinte checklist:

```text
[ Código em Rust/C ] ──► [ Testes Unitários ] ──► [ Linting & Formatting ] ──► [ CI Workflow ] ──► [ Code Review ] ──► [ Merge ]
```

1. **Compilação Limpa**: O código deve compilar sem avisos (*zero warnings*) via `cargo clippy --all-targets -- -D warnings`.
2. **Formatação Padronizada**: Código formatado estritamente via `cargo fmt --check`.
3. **Suíte de Testes Aprovada**: Todos os testes unitários e de integração devem passar com sucesso (`cargo test --workspace`).
4. **Isolamento de Platform (PAL)**: O código no `openkey-core` **não** pode importar ou depender de bibliotecas específicas de plataformas de hardware.
5. **Comentários de Segurança**: Todo uso de `unsafe` (restrito às camadas HAL/USB) deve conter justificativa explicativa no formato `// SAFETY:`.
6. **Atualização da Documentação**: Quaisquer alterações nas interfaces públicas ou contratos de API devem vir acompanhadas da atualização das documentações correspondentes em `docs/`.
