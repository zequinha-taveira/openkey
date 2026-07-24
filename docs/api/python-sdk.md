# Referência da API Python SDK (`docs/api/python-sdk.md`)

## 🐍 Biblioteca `pyopenkey`

```python
from openkey import OpenKeyClient

client = OpenKeyClient.connect_usb()
info = client.get_info()
print(f"OpenKey Firmware Version: {info.version}")

# Exemplo de MakeCredential
credential = client.make_credential(
    rp_id="example.com",
    user_id=b"user_123",
    user_name="alice@example.com"
)
```
