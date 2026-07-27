# CBOR

## Visão Geral

Concise Binary Object Representation (RFC 8949).

## Características

- Codificação canônica estática
- Sem alocação dinâmica na heap
- Validação rigorosa de estruturas

## Uso no OpenKey

- Codificação de comandos CTAP2
- Codificação de respostas
- Serialização de credenciais

## Implementação

Parser em `core/` com foco em segurança e eficiência.