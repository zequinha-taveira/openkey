# Protocolo USB HID / CTAPHID (`docs/protocols/hid.md`)

## 🔌 Especificação de Protocolo USB HID

O protocolo **CTAPHID** encapsula pacotes CTAP2 sobre o transporte USB HID com suporte a multiplexação de múltiplos canais.

## 📦 Formato dos Relatórios HID (64 Bytes)

### Pacote Inicial (`INIT` Frame)

```text
+--------------+-------------+----------------+----------------+
| CID (4B)     | CMD (1B)    | BCNTH (1B)     | BCNTL (1B)     |
| Channel ID   | Command     | Payload Length | Payload Length |
+--------------+-------------+----------------+----------------+
| DATA (57 bytes...)                                           |
+--------------------------------------------------------------+
```

### Pacote de Continuação (`CONT` Frame)

```text
+--------------+-------------+---------------------------------+
| CID (4B)     | SEQ (1B)    | DATA (59 bytes...)              |
| Channel ID   | Sequence    | Payload segment                 |
+--------------+-------------+---------------------------------+
```
