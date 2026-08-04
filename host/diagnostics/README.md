# OpenKey Diagnostics (`host/diagnostics/`)

Serviço de diagnóstico e análise de chaves OpenKey (G10-T10):

- Verificações de integridade e conformidade FIDO2/CTAP2 sobre um adapter
  duck-typed do dispositivo (`DiagnosticsService`).
- Checks de conectividade (GetInfo), versões, AAGUID, opções, maxMsgSize e
  subsistema de PIN (`pinUvAuthProtocols`, `pinRetries`).
- Integração com o OpenKey Manager via `core/diagnostics.py` (GUI).
- Suporte futuro a checks de firmware (flash/RNG/secrets) quando o protocolo
  de diagnóstico do firmware estiver disponível no SDK.

Estrutura:

```
host/diagnostics/
├── pyproject.toml              # pacote "openkey-diagnostics" (sem deps)
├── openkey_diagnostics/
│   ├── __init__.py
│   └── diagnostics.py          # DiagnosticsService + DiagnosticsReport
└── tests/
    └── test_diagnostics.py     # 12 testes com adapters fake
```

Testes:

```bash
pip install -r requirements-dev.txt   # na raiz do monorepo (instala o pacote em modo editable)
python -m pytest host/diagnostics -q
```

Consulte [`docs/reference/host/gui.md`](../../docs/reference/host/gui.md) para a
integração com a GUI.
