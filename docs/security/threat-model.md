# Modelo de Ameaças, Ativos e Mitigações (`docs/security/threat-model.md`)

O **OpenKey** é um autenticador de hardware FIDO2 / CTAP2 e WebAuthn projetado para operar em ambientes host potencialmente hostis ou comprometidos. Este documento estabelece o modelo de ameaças formal baseado na metodologia **STRIDE**, mapeando ativos protegidos, modelos de atacante, vetores de ataque e mecanismos de mitigação arquiteturais e criptográficos.

---

## 🎯 1. Classificação de Ativos (Assets)

Os ativos do OpenKey são divididos em quatro categorias principais de acordo com seu impacto de segurança e ciclo de vida:

### 1.1 Segredos de Fábrica e Dispositivo (Hardware Assets)
- **Chave Privada de Atestação (`Attestation Private Key`)**: Chave assimétrica (ECDSA P-256 ou Ed25519) injetada em ambiente seguro de fábrica. Utilizada para assinar certificados FIDO2 durante o registro de novas credenciais.
- **Certificado de Atestação (`Attestation Certificate`)**: Certificado X.509 assinado pela CA da OpenKey para comprovar a autenticidade do hardware.
- **Master Root Key / Seed de Dispositivo**: Segredo mestre armazenado em regióes protegidas de memória a partir do qual chaves derivativas são geradas.

### 1.2 Segredos do Usuário (User Assets)
- **Hash do PIN de Usuário**: Derivação criptográfica forte do PIN do usuário utilizando `PBKDF2-HMAC-SHA256` ou `Argon2` combinada com um salt exclusivo por dispositivo.
- **Chaves Privadas de Credenciais Residentes (RK)**: Chaves de autenticação individuais (ECDSA P-256, Ed25519 ou RSA) geradas para cada Relying Party (RP).
- **Chaves de Credenciais Não-Residentes**: Parâmetros e sementes protegidos envolvidos (*wrapped*) em Key Handles.

### 1.3 Dados de Estado e Integridade (System State Assets)
- **Contador Monotônico Global de Assinatura (`Signature Counter`)**: Contador global persistente incrementado a cada operação de assinatura de autenticação para prevenção de ataques de *replay* e clonagem.
- **Contadores de Tentativas de PIN e Estado de Bloqueio**: Registro de falhas de autenticação de PIN para mitigar ataques de força bruta.
- **Mapeamento de Relying Party ID (RP ID)**: Vínculo unívoco entre a origem da aplicação web (ex: `google.com`) e o conjunto de credenciais associadas.

### 1.4 Segredos Efêmeros de Sessão (Volatile Session Assets)
- **Chave Privada ECDH de Sessão CTAP2**: Par de chaves efémero gerado a cada negociação de canal seguro (`pinUvAuthProtocol`).
- **Token de Sessão Autenticado (`pinUvAuthToken`)**: Token temporário mantido exclusivamente em RAM para autorizar operações privilegiadas durante uma sessão USB/NFC ativa.
- **Buffers de Mensagens CBOR e Pacotes HID**: Dados transitórios de requisições e respostas CTAP2 em processamento na RAM.

---

## 🎭 2. Modelos de Atacante e Fronteiras de Confiança

O OpenKey define quatro perfis de atacantes em seu modelo de ameaças:

```text
+-----------------------------------------------------------------------+
|                       [ Atacante A2: RP Maliciosa / Web ]              |
|                                     |                                 |
|                                     v                                 |
|                       [ Atacante A1: Host OS Malware ]                |
|                                     | (USB HID / NFC Channel)         |
+-------------------------------------|---------------------------------+
                                      v
+-----------------------------------------------------------------------+
| FRONTEIRA DE CONFIANÇA DO HARDWARE OPENKEY                            |
|                                                                       |
|   [ Atacante A3: Físico Não-Invasivo (Side-Channel / Glitch) ]        |
|   [ Atacante A4: Físico Invasivo (Microprobing / Flash Decap) ]       |
|                                                                       |
|   +---------------------------------------------------------------+   |
|   | Microcontrolador Seguro (Flash Protect RDP / MPU / TRNG)       |   |
|   |  - Core CTAP2 / Engine Criptográfica                           |   |
|   |  - Memória Zeroizada (SRAM)                                   |   |
|   +---------------------------------------------------------------+   |
+-----------------------------------------------------------------------+
```

1. **Atacante A1 (Host OS Malware / USB Man-in-the-Middle)**: Malware executando no sistema operacional do usuário com permissão para enviar relatórios USB HID arbitrários ao autenticador.
2. **Atacante A2 (Relying Party Maliciosa / Phishing WebApp)**: Aplicação web maliciosa que tenta solicitar credenciais de outros domínios ou forjar parâmetros CTAP2.
3. **Atacante A3 (Atacante Físico Não-Invasivo)**: Possui posse física do dispositivo e utiliza análise de consumo de energia (SPA/DPA), análise de tempo (Timing Attacks) ou indução de falhas de clock/tensão (Fault Injection/Glitch).
4. **Atacante A4 (Atacante Físico Invasivo / Semi-Invasivo)**: Possui laboratório avançado para decapsulamento de chip, microprobing de barramentos internos ou leitura ótica de Flash/SRAM.

---

## 🛡️ 3. Matriz STRIDE de Ameaças e Mitigações

| Categoria STRIDE | Ameaça Concreta | Vetor de Ataque | Impacto | Mitigação no OpenKey |
| :--- | :--- | :--- | :--- | :--- |
| **Spoofing** | Emulação de Autenticador OpenKey legítimo | Clonagem de firmware ou forjamento de IDs USB em hardware não autorizado | Alto | Certificado de Atestação de Fabricante injetado em fábrica assinado por chave privada protegida por Flash RDP Level 2. |
| **Spoofing** | Forjamento de Relying Party ID por aplicativo malicioso | Malware injeta RP ID falso (ex: `bank.com`) via CTAP2 em canal aberto | Alto | Validação obrigatória da origem do cliente via navegador (WebAuthn ClientDataHash) e derivação do RP ID Hash. |
| **Tampering** | Adulteração do Firmware via atualização USB | Injeção de imagem de firmware modificada via interface DFU/USB | Crítico | Bootloader seguro com verificação de assinatura digital ECDSA P-256 da imagem antes da gravação e rollback protection. |
| **Tampering** | Adulteração de dados de credenciais na Flash | Leitura/Escrita direta nos setores de armazenamento da Flash por código malicioso | Alto | Proteção de memória MPU, Flash Read-Out Protection (RDP L2) e somas de verificação HMAC em estruturas de armazenamento. |
| **Repudiation** | Negação de autorização de autenticação pelo usuário | Malware solicita assinaturas em segundo plano sem conhecimento do usuário | Crítico | Exigência física incondicional de Teste de Presença de Usuário (User Presence - UP) via toque no sensor capacitivo para cada assinatura. |
| **Information Disclosure** | Extração de chaves por análise de canal lateral (Timing Attack) | Medição de tempo de resposta em operações ECC/AES durante o CTAP2 | Crítico | Implementação estritamente em tempo constante (*constant-time*) para todas as operações criptográficas e comparações de bytes. |
| **Information Disclosure** | Remanência de segredos na memória SRAM | Leitura de buffers de RAM após conclusão do processamento de comando CTAP2 | Alto | Uso da trait `Zeroize` do Rust em todas as estruturas que contêm chaves, PINs ou tokens ao saírem de escopo. |
| **Denial of Service** | Bloqueio de dispositivo por exaustão de tentativas de PIN | Inundação de requisições de verificação de PIN incorreto por malware no host | Médio | Contador de tentativas de PIN com retardo exponencial e bloqueio definitivo (factory reset necessário) após 8 falhas consecutivas. |
| **Denial of Service** | Inundação de pacotes USB HID (HID Flooding) | Malware envia tempestade de pacotes malformados USB HID | Médio | Máquina de estados estrita do protocolo HID com time-out de canal de 3 segundos e validação de tamanho pré-alocada. |
| **Elevation of Privilege** | Vazamento cruzado de credenciais entre RPs distintas | RP A tenta solicitar a chave privada ou assinatura associada à RP B | Crítico | Isolamento estrito por Hash de RP ID. A chave é derivada deterministicamente utilizando `HKDF-SHA256(MasterSeed, RP_ID_Hash)` ou isolada em Flash por ID. |

---

## 🔐 4. Estratégias Principais de Mitigação Técnica

### 4.1 Operações em Tempo Constante (*Constant-Time*)
Nenhuma comparação de chave, hash, token ou operação criptográfica pode utilizar desvios condicionais baseados no valor do segredo (`if secret == target`). Todas as primitivas utilizam abstrações como `subtle::ConstantTimeEq`.

### 4.2 Sanitização e Zeroização de Memória
A memória SRAM é estritamente limpa após o processamento. Variáveis contendo segredos implementam a trait `Drop` com sanitização forçada através de instruções de gravação direta e barreiras de compilação (*compiler memory barriers*).

### 4.3 Isolamento de Periféricos e Proteção de Hardware
- **MPU (Memory Protection Unit)**: Configurada para isolar o stack do firmware contra estouro e impedir execução a partir de regiões de dados (RAM / Flash de dados).
- **TRNG Hardware**: Gerador de números aleatórios de hardware com testes contínuos de entropia baseados no padrão NIST SP 800-90B. Se a entropia falhar, o dispositivo interrompe o processamento de novos comandos.

---

## ⚠️ 5. Riscos Residuais e Suposições de Segurança

1. **Comprometimento Físico Invasivo Extremo (Atacante A4)**: Ataques com microscópio de varredura eletrônica (SEM) e FIB (Focused Ion Beam) estão fora do escopo de mitigação de firmware e dependem exclusivamente da resistência física da camada de encapsulamento de silício.
2. **Confiabilidade do Host Browser / SO**: Assume-se que o navegador do usuário valida corretamente o cabeçalho `Origin` antes de enviar o `clientDataHash` ao OpenKey via CTAP2.
