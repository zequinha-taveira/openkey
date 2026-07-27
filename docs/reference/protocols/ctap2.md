# Protocolo CTAP2

## Visão Geral

Client-to-Authenticator Protocol version 2.0/2.1.

## Comandos Principais

- `authenticatorGetInfo` - Informações do dispositivo
- `authenticatorMakeCredential` - Criação de credencial
- `authenticatorGetAssertion` - Autenticação
- `authenticatorClientPIN` - Gestão de PIN
- `authenticatorReset` - Reset de fábrica
- `authenticatorGetDisposition` - Status

## Formato

- Mensagens codificadas em CBOR
- Framing CTAPHID USB HID
- Pacotes de 64 bytes

## Implementação

O motor CTAP2 reside em `core/` e comunica-se com a PAL para acesso a hardware.