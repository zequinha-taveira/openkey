# OpenKey Python SDK (`host/sdk-python/`)

Biblioteca cliente Python para comunicação com dispositivos OpenKey e simuladores
FIDO2 via USB HID.

## Recursos

- API Python para operações CTAP2 (`GetInfo`, `Reset`; `MakeCredential`,
  `GetAssertion`, `ClientPin` e `CredentialManagement` planejados na Fase 10).
- Transporte USB HID real via `hidapi` (`OpenKeyDevice.from_hid()`,
  `discover_devices()`, `open_device()`).
- Descoberta de dispositivos com filtro por VID/PID/número de série.
- Backend mock integrado (sem hardware) para testes e desenvolvimento.

## Instalação

```bash
# Sem suporte a hardware real
pip install openkey-sdk

# Com transporte USB HID real
pip install "openkey-sdk[hid]"
```

## Uso rápido

```python
from openkey import OpenKeyDevice

# Conecta ao primeiro dispositivo OpenKey via USB HID
dev = OpenKeyDevice.from_hid()
info = dev.get_info()
print(info.aaguid.hex(), info.versions, info.options)

# Modo mock (sem hardware)
dev = OpenKeyDevice()
print(dev.get_info())
```

Documentação da API em [`docs/reference/host/python-sdk.md`](../../docs/reference/host/python-sdk.md).
