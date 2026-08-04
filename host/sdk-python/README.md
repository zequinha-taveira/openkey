# OpenKey Python SDK (`host/sdk-python/`)

Biblioteca cliente Python para comunicação com dispositivos OpenKey e simuladores
FIDO2 via USB HID.

## Recursos

- API Python para operações CTAP2 (`GetInfo`, `Reset`, `ClientPin`/PIN,
  `CredentialManagement`, `MakeCredential`, `GetAssertion`).
- Protocolo `authenticatorClientPIN` (pinUvAuthProtocol v1 e v2) com ECDH
  P-256, AES-256-CBC e HMAC-SHA-256 (`setup_pin`, `change_device_pin`,
  `PinClient`).
- Gestão de credenciais residentes (`CredentialManagementClient`): metadata,
  enumeração de RPs e credenciais, remoção de credenciais.
- Criação e autenticação de credenciais (`make_credential` / `get_assertion`)
  com respostas tipadas e parse do authenticator data.
- Hook de logging de pacotes CTAP (`log_hook` + `CtapLogRecorder`) para
  captura/depuração de tráfego.
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

## Gestão de credenciais residentes

```python
from openkey import Ctap2Client, PinClient, CredentialManagementClient

# Assume transporte aberto (ex.: OpenKeyDevice.from_hid())
ctap2 = Ctap2Client(dev.send_command)
pin = PinClient(ctap2)
pin.get_key_agreement()
pin.set_pin("1234")  # ou use o PIN existente via change_pin

cm = CredentialManagementClient(ctap2, pin, pin="1234")
print(cm.get_metadata())
for rp in cm.enumerate_rps():
    for cred in cm.enumerate_credentials(rp.id):
        print(cred.credential_id.hex(), cred.user)
# cm.delete_credential(cred_id, rp_id)
```

## WebAuthn (makeCredential / getAssertion)

```python
import hashlib
from openkey import Ctap2Client, RpEntity, UserEntity, PublicKeyCredentialDescriptor

ctap2 = Ctap2Client(dev.send_command)

client_data_hash = hashlib.sha256(b"challenge").digest()
rp = RpEntity(id="example.com", name="Example")
user = UserEntity(id=b"user-1", name="alice", display_name="Alice")

resp = ctap2.make_credential(
    client_data_hash, rp, user, pub_key_cred_params=[-7, -257]
)
print(resp.fmt, resp.auth_data_obj.attested)

assertion = ctap2.get_assertion(
    rp.id, client_data_hash,
    allow_list=[PublicKeyCredentialDescriptor(id=resp.auth_data_obj.credential_id)],
)
print(assertion.signature.hex(), assertion.user)
```

Documentação da API em [`docs/reference/host/python-sdk.md`](../../docs/reference/host/python-sdk.md).
