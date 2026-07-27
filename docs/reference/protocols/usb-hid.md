# USB HID

## Framing CTAPHID

Protocolo de enquadramento para comunicação USB HID.

## Pacotes

- **INIT** - Inicialização de canal
- **MSG** - Mensagem CTAP2
- **PING** - Ping de teste
- **CANCEL** - Cancelamento de comando
- **ERROR** - Resposta de erro

## Tamanho

- Pacotes de 64 bytes (padrão HID)
- Fragmentação automática para mensagens maiores

## Implementação

Reside em `platform/src/hal/usb.rs` e `protocols/`.