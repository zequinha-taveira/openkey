# OpenKey Provisioner (`host/provisioner/`)

Ferramenta de provisionamento de fábrica para chaves OpenKey:

- Grava Board Profile, Device Profile e Application Configuration na flash.
- Injeta chaves de fábrica e certificados de atestado.
- Verifica a integridade do firmware gravado.
- Suporte a fluxos de produção em lote.

Consulte [`docs/how-to/provision-device.md`](../../docs/how-to/provision-device.md) para o guia de uso.

Requer o pacote `openkey-sdk` (caminho `host/sdk-python`); instale as dependências
do monorepo com `pip install -r requirements-dev.txt` (raiz).
