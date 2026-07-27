# SDK Python

## Instalação

```bash
pip install openkey-sdk
```

## Uso Básico

```python
from openkey import SecurityKey

key = SecurityKey()
info = key.get_info()
print(f"Vendor: {info.vendor}")
print(f"Product: {info.product}")
```

## APIs

- `discover()` - Descobre dispositivos conectados
- `get_info()` - Informações do dispositivo
- `make_credential()` - Cria nova credencial
- `get_assertion()` - Autenticação
- `set_pin()` - Define PIN
- `reset()` - Reset de fábrica