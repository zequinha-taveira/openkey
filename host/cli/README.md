# OpenKey CLI Tool (`host/cli/`)

Interface de linha de comando (`openkey-cli`) para gerenciar chaves OpenKey:
- Configurar PIN / BioData.
- Listar e revogar Credenciais Residentes (RK).
- Atualizar firmware via Bootloader DFU seguro.

Documentação de uso em [`docs/reference/host/cli.md`](../../docs/reference/host/cli.md).

Requer o pacote `openkey-sdk` (caminho `host/sdk-python`); instale as dependências
do monorepo com `pip install -r requirements-dev.txt` (raiz).
