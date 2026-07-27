# Provisionamento

## Visão Geral

Processo de configuração inicial do dispositivo.

## Etapas

1. **Conexão** - Conectar dispositivo OpenKey
2. **Identificação** - Detectar tipo de board
3. **Board Profile** - Definir características da placa
4. **Device Profile** - Atribuir número de série e identidade USB
5. **Application Configuration** - Configurar CTAP2, PIN, políticas
6. **Salvamento** - Gravar na Flash persistente

## Arquivos de Configuração

- `board.json` - Configuração do board
- `device.json` - Dados do dispositivo
- `app.json` - Configuração da aplicação

## Verificação

Após provisionamento, o dispositivo deve:
- Enumerar como USB HID
- Responder a `authenticatorGetInfo`
- Estar pronto para credenciais