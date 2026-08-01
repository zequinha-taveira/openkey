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
- `make_credential()` - Cria nova credencial (planejado, Fase 10)
- `get_assertion()` - Autenticação (planejado, Fase 10)
- `set_pin()` / `change_pin()` - Gestão de PIN (planejado, Fase 10)

> Os métodos marcados como "planejado" são gaps do SDK a serem implementados na
> Fase 10 (ver `TASKS.md` G10-T02 a G10-T05).
