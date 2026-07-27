# Crypto

## Arquitetura

`firmware/crypto` fornece primitivas criptográficas independentes de Core e
Platform. Isso preserva a direção de dependências: Core → Platform → Crypto.

## Configuração persistente

AES-256-GCM do RustCrypto é usado in-place, sem heap, para confidencialidade e
autenticidade da configuração. A Platform fornece a chave via
`ConfigKeyProvider` e nonce via `RngProvider`; Crypto não mantém chaves nem
acessa hardware. Chaves e buffers temporários são zeroizados pela Platform.

## Princípios

- Sem chaves fixas no firmware.
- Nonce de 96 bits novo por escrita e por chave.
- Falha de autenticação ou RNG falha fechada.
- Execução em tempo constante conforme as garantias do alvo e do RustCrypto.
