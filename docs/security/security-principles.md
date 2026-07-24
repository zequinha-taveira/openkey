# Princípios de Engenharia Segura (`docs/security/security-principles.md`)

Este documento estabelece o conjunto de **regras normativas e imutáveis de engenharia segura** que todo o código no repositório **OpenKey** — incluindo firmware `no_std`, SDKs host, CLIs e ferramentas de teste — deve obedecer estritamente.

---

## 🧱 1. Regras Permanentes de Engenharia Segura

### Regra 1: Defesa em Profundidade (Defense in Depth)
- **Princípio**: O sistema não pode depender de uma única camada de segurança. A falha de um mecanismo de defesa não deve resultar no comprometimento do sistema.
- **Aplicação Prática**: 
  - Um pacote CTAP2 válido deve passar por validação de tamanho USB HID, validação de enquadramento de canal, decodificação CBOR canônica com verificação de tipos e autorização de token `pinUvAuthToken` antes de executar qualquer operação criptográfica.
  - Dados gravados na Flash usam proteção física de leitura (Flash RDP L2), isolamento por MPU e checksums criptográficos HMAC.

---

### Regra 2: Menor Privilégio e Isolamento de Domínios (Least Privilege)
- **Princípio**: Cada módulo, função ou driver deve possuir apenas os privilégios estritamente necessários para cumprir sua tarefa designada.
- **Aplicação Prática**:
  - Drivers de periféricos (ex: USB, SPI, TRNG) não têm acesso direto aos endereços de memória contendo sementes criptográficas ou contadores de PIN.
  - Abstrações de hardware utilizam a tipagem estática de Rust (Ownership / Borrow Checker) para garantir que apenas o proprietário do periférico possa interagir com seus registradores.

---

### Regra 3: Falha Segura e Defaults Seguros (Fail-Safe Defaults)
- **Princípio**: Diante de qualquer erro não esperado, estado inconsistente, falha de integridade ou condição limite, a reação padrão do sistema deve ser o encerramento seguro e imediato da operação.
- **Aplicação Prática**:
  - Funções de produção no firmware **nunca** devem utilizar `panic!`, `unwrap()` ou `expect()`. Em caso de erro, devem retornar tipos fortemente tipados `Result<T, CtapError>`.
  - Se o gerador de números aleatórios de hardware (TRNG) falhar em um teste de entropia contínuo, a transação criptográfica em curso é cancelada e os tokens de sessão ativos são imediatamente invalidados.

---

### Regra 4: Zero-Knowledge & Zeroização Obliterativa (Zero-Trace Memory)
- **Princípio**: Dados sensíveis e segredos devem residir em memória RAM pelo menor tempo possível e ser completamente limpos após o uso.
- **Aplicação Prática**:
  - Todas as estruturas contendo chaves privadas, segredos efêmeros ECDH, PINs ou tokens de sessão implementam a trait `Zeroize` (ou `ZeroizeOnDrop`).
  - O compilador é impedido de otimizar ou remover chamadas de limpeza de memória por meio de barreiras explícitas de memória (`compiler_fence`).
  - Mensagens de log ou respostas de erro para o host jamais contêm fragmentos de chaves, PINs ou hashes intermediários.

---

### Regra 5: Execução de Tempo Constante (Constant-Time Operations)
- **Princípio**: Nenhuma operação que manipule dados secretos pode introduzir variações de tempo de execução dependentes do valor do segredo.
- **Aplicação Prática**:
  - Comparações de bytes de hashes, tokens ou assinaturas usam funções de tempo constante como `subtle::ConstantTimeEq`.
  - Multiplicações de pontos em curvas elípticas utilizam algoritmos de rotina fixa e desvios ramificados (*branchless code*) imunes a ataques de canal lateral (SPA/DPA/Timing).

---

### Regra 6: Validação Rigorosa de Entrada e Parsing Seguro (Strict Parsing)
- **Princípio**: Toda entrada proveniente do host ou de interfaces externas é considerada não confiável e maliciosa até ser completamente sanitizada.
- **Aplicação Prática**:
  - Estruturas CBOR são decodificadas usando validadores de limite rígidos (*bounds checking*), impedindo qualquer tentativa de estouro de memória (*buffer overflow*).
  - Alocações dinâmicas não determinísticas na heap são proibidas no firmware `no_std`. Os buffers de recepção possuem tamanho fixo alocado estaticamente na compilação.

---

### Regra 7: Política Estrita para Código `unsafe`
- **Princípio**: O uso de blocos `unsafe` em Rust é minimizado e mantido sob controle e auditoria rigorosos.
- **Aplicação Prática**:
  - Todo bloco `unsafe` DEVE ser precedido por um comentário explicativo no formato `// SAFETY: <invariantes mantidas>`.
  - Códigos `unsafe` devem ser obrigatoriamente encapsulados em abstrações seguras e exigem aprovação de pelo menos dois mantenedores de segurança, em conformidade com o [`ADR-0004`](../adr/ADR-0004-unsafe.md) e a [`Política de Unsafe`](unsafe-policy.md).

---

### Regra 8: Monotonicidade e Proteção Anti-Replay
- **Princípio**: O estado do autenticador deve impedir estritamente a reutilização de assinaturas ou regressões de contadores de integridade.
- **Aplicação Prática**:
  - O Contador Monotônico Global de Assinaturas é armazenado na Flash e incrementado de forma atômica a cada emissão de instrução `authenticatorGetAssertion`.
  - Operações de escrita na Flash garantem persistência síncrona antes de responder com sucesso ao host.

---

### Regra 9: Negação Explícita por Padrão (Explicit Access Control)
- **Princípio**: Qualquer requisição ou operação que não esteja explicitamente permitida pela matriz de autorização deve ser rejeitada imediatamente.
- **Aplicação Prática**:
  - Operações que exigem PIN de Usuário ou Verificação de Presença (UP) rejeitam o comando se as flags de autorização não estiverem ativas no token de sessão `pinUvAuthToken`.

---

## 📋 2. Checklist Normativa de Revisão de Código de Segurança

Antes de submeter um Pull Request no OpenKey, os desenvolvedores devem verificar se o código cumpre os seguintes pontos:

- [ ] Nenhum `unwrap()`, `expect()` ou `panic!` foi introduzido no caminho de execução de produção.
- [ ] Todas as estruturas que armazenam segredos em RAM derivam ou implementam `ZeroizeOnDrop`.
- [ ] Todas as comparações de segredos ou tokens usam primitivas de tempo constante (`ConstantTimeEq`).
- [ ] Nenhum bloco `unsafe` novo foi adicionado sem justificativa `// SAFETY:` detalhada.
- [ ] Entradas de pacotes externos possuem limites estáticos e verificações de tamanho explícitas.
- [ ] Funções que modificam estado ou emitem assinaturas respeitam a monotonicidade do contador.
- [ ] Todos os testes unitários e de integração relevantes foram executados com sucesso (`cargo test --workspace`).
