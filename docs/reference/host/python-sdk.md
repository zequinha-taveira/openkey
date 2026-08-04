# SDK Python

O SDK Python (`host/sdk-python/`) é a biblioteca cliente para comunicação com
dispositivos OpenKey (físicos USB HID ou emulados).

## Instalação

```bash
# Sem suporte a hardware real (somente mock/simulador)
pip install openkey-sdk

# Com suporte a transporte USB HID real (hidapi)
pip install "openkey-sdk[hid]"
```

## Uso Básico

```python
from openkey import OpenKeyDevice

# Modo mock (sem hardware)
dev = OpenKeyDevice()
info = dev.get_info()

# Modo USB HID real (descobre e abre o primeiro dispositivo OpenKey)
dev = OpenKeyDevice.from_hid()
info = dev.get_info()
print(f"AAGUID: {info.aaguid.hex()}")
print(f"Versões: {info.versions}")
print(f"Opções: {info.options}")
```

## Descoberta de Dispositivos (USB HID)

```python
from openkey import discover_devices, open_device, OPENKEY_VID, OPENKEY_PID

# Lista os dispositivos OpenKey conectados (filtro por VID/PID padrão)
devices = discover_devices()

# Lista todos os dispositivos HID
all_devs = discover_devices(vid=None, pid=None)

# Abre e inicializa o canal CTAPHID explicitamente
backend = open_device(vid=OPENKEY_VID, pid=OPENKEY_PID, serial_number="SN123")
dev = OpenKeyDevice(transport_backend=backend)
```

## Transporte

O backend de transporte implementa o contrato `send_cmd(cid, cmd, payload)`:

- `HidTransportBackend` — USB HID real via `hidapi` (CTAPHID sobre relatórios
  de 64 bytes). Executa `CTAPHID_INIT` para obter o CID do canal, remonta
  mensagens multipacote e ignora keepalives (`CTAPHID_KEEPALIVE`).
- Mock integrado em `OpenKeyDevice` — usado quando nenhum backend é fornecido
  (útil para testes e desenvolvimento sem hardware).

## APIs

- `OpenKeyDevice()` - Instância de dispositivo (mock por padrão)
- `OpenKeyDevice.from_hid(...)` - Conecta via USB HID real (faz descoberta + INIT)
- `discover_devices(vid, pid, serial_number)` - Descobre dispositivos HID
- `open_device(vid, pid, serial_number, path)` - Abre e inicializa um dispositivo
- `get_info()` - Informações do autenticador (`authenticatorGetInfo`)
- `reset()` - Reset de fábrica
- `setup_pin(ctap2, pin, protocol_version)` - Define o PIN (key agreement + setPIN)
- `change_device_pin(ctap2, current_pin, new_pin, protocol_version)` - Altera o PIN
- `PinClient` - Cliente de alto nível (get_pin_retries, set_pin, change_pin, get_pin_token)
- `CredentialManagementClient(ctap2, pin_client, pin)` - Gestão de credenciais residentes
- `CredentialManagementClient.get_metadata()` - Contadores de credenciais (getCredsMetadata)
- `CredentialManagementClient.enumerate_rps()` - Lista as Relaying Parties (enumerateRPs + paginação)
- `CredentialManagementClient.enumerate_credentials(rp_id)` - Credenciais residentes de uma RP (enumerateCredentials + paginação)
- `CredentialManagementClient.delete_credential(credential_id, rp_id)` - Remove uma credencial residente
- `Ctap2Client.make_credential(client_data_hash, rp, user, pub_key_cred_params, ...)` - Cria credencial (authenticatorMakeCredential)
- `Ctap2Client.get_assertion(rp_id, client_data_hash, ...)` - Autenticação (authenticatorGetAssertion)
- `AuthenticatorData.parse(auth_data)` - Parse do authenticator data (flags UP/UV/AT, signCount, aaguid)
- `OpenKeyDevice(..., log_hook=...)` / `Ctap2Client(..., log_hook=...)` - Hook de log de pacotes CTAP
- `CtapLogRecorder` - Coletor de entradas de log (`CtapLogEntry`) para o visualizador da GUI

> Protocolo PIN: `authenticatorClientPIN` (CTAP2.1 §6.5) com pinUvAuthProtocol
> v1 e v2 (ECDH P-256, AES-256-CBC, HMAC-SHA-256). O `pinUvAuthToken` derivado
> é efêmero e nunca é logado ou persistido.

> Gestão de credenciais: `authenticatorCredentialManagement` (CTAP2.1 §6.8). O
> `pinAuth` de cada subcomando é derivado do `pinUvAuthToken`; a paginação de
> RPs/credenciais é transparente para o chamador.

> WebAuthn: `make_credential` (CTAP2.1 §6.2) e `get_assertion` (CTAP2.1 §6.3)
> aceitam `RpEntity`/`UserEntity`/`PublicKeyCredentialDescriptor` e retornam
> respostas tipadas com o authenticator data já parseado.

> Logging de pacotes CTAP: o hook `log_hook(direction, ctap_cmd, payload)` é
> invocado para cada comando enviado (`LOG_SEND`) e resposta recebida
> (`LOG_RECV`), permitindo captura para o visualizador de logs (G10-T12).
