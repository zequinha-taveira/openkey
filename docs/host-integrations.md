# Integrações host

O crate `openkey-host` é o SDK comum das aplicações executadas no computador. Ele recebe um `HostTransport` e não conhece USB real, sistema operacional ou MCU.

## Operações

- `get_info`: versão do protocolo, firmware e estado de provisionamento;
- `provision`: inicialização do dispositivo com um label validado;
- `update`: envio de pacote com versão, imagem e resultado de assinatura;
- `diagnostics`: saúde do transporte, armazenamento e firmware.

A CLI usa o SDK e o `OfflineTransport` para desenvolvimento sem hardware:

```bash
cargo run -p openkey-cli -- info
cargo run -p openkey-cli -- provision
cargo run -p openkey-cli -- update
cargo run -p openkey-cli -- diagnostics
```

A GUI desktop deve consumir o mesmo SDK. Nenhuma aplicação host deve acessar diretamente chaves privadas, flash interna ou lógica de protocolo.

O `OfflineTransport` não representa uma verificação criptográfica de produção: a assinatura real será conectada quando COSE e o transporte físico estiverem implementados.
