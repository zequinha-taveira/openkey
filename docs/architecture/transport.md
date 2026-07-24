# Camada de Transporte (`docs/architecture/transport.md`)

## 🚚 Abstração de Transporte (USB HID & NFC)

O OpenKey suporta múltiplos meios de transporte físicos garantindo transparência no nível de aplicação CTAP2.

## 🔌 USB HID (CTAPHID)

- **Tamanho do Frame**: 64 bytes por relatório HID.
- **Protocolo de Enquadramento**:
  - `INIT`: Abre um ID de Canal (`CID`) de 32 bits exclusivo.
  - `CONT`: Pacotes de continuação encadeados por número de sequência (0x00 a 0x7F).
  - Timeout de canal por inatividade (500 ms).

## 📡 NFC (CTAPNFC)

- Emulação de Cartão ISO/IEC 7816-4.
- Seleção de Applet FIDO via AID (`A0000006472F0001`).
- Empacotamento de APDUs curtas e estendidas.
