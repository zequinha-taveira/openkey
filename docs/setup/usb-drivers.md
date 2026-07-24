# Guia de Drivers USB e Identidade VID/PID (`docs/setup/usb-drivers.md`)

## 📌 Objetivo

Documentar a arquitetura de enquadramento USB, política estrita de **Vendor ID (VID) / Product ID (PID)**, classes de dispositivos compostos (FIDO HID e CCID) e configuração de drivers nos sistemas operacionais host.

---

## 🆔 Identidade USB e Política de VID/PID

O OpenKey adere a uma política clara de governança para identificadores USB, estabelecendo uma distinção rígida entre artefatos de produção e perfis de testes:

```text
                                 Build Profiles
                                       │
                 ┌─────────────────────┴─────────────────────┐
                 ▼                                           ▼
      [ Build Oficial OpenKey ]                   [ Build de Interoperabilidade ]
      - Usa VID/PID próprio legal                  - Usa VID/PID de testes (ex: 1209:0001)
      - Distribuição em Releases                   - Desenvolvimento e Validação Interna
      - Identidade Registrada                      - NUNCA personifica terceiros
```

### 1. Build Oficial (Release Binaries)
- Utiliza o **Vendor ID (VID)** e **Product ID (PID)** registrados oficialmente do projeto OpenKey (obtidos legalmente, por exemplo via *pid.codes* ou atribuição formal USB-IF).
- Nome da string de produto USB: `OpenKey Security Key`.
- Nome do leitor PC/SC registrado: `OpenKey`.

### 2. Builds de Interoperabilidade (Development Profiles)
- O sistema de build fornece perfis configuráveis opcionais (ex: `--features interop-profile`) para testes de interoperabilidade e homologação.
- Esses perfis destinam-se **exclusivamente** a ambiente de desenvolvimento local.
- **Regra de Compliance**: As builds de interoperabilidade utilizam um identificador alternativo de testes e **nunca** utilizam identificadores pertencentes comercialmente a outros fabricantes de hardware.

---

## 🔌 Enumeração Dispositivo Composto (USB Composite Device)

O OpenKey enumera como um dispositivo composto USB contendo as seguintes interfaces independentes:

```text
Dispositivo Composto USB (OpenKey)
├── Interface 0: FIDO HID
│   ├── Class: HID (0x03)
│   ├── Usage Page: 0xF1D0 (FIDO Alliance)
│   ├── Usage: 0x0001 (FIDO Authenticator)
│   └── Endpoints: Interrupt IN (64 bytes), Interrupt OUT (64 bytes)
│
└── Interface 1: CCID (Smart Card Reader - Opcional)
    ├── Class: Smart Card (0x0B)
    ├── SubClass: 0x00, Protocol: 0x00
    └── Endpoints: Bulk IN, Bulk OUT, Interrupt IN
```

---

## 🛠️ Drivers por Sistema Operacional

| Sistema | Interface FIDO HID | Interface CCID | Ação Recomendada |
| :--- | :--- | :--- | :--- |
| **Linux** | Driver kernel `hidraw` | Daemon `pcscd` | Adicionar regra udev `/etc/udev/rules.d/70-openkey.rules`. |
| **Windows** | Driver nativo `HIDClass` | Driver nativo `Microsoft Usbccid` | Nenhuma instalação de driver manual necessária. |
| **macOS** | Driver nativo `IOHIDFamily` | Framework nativo `PCSC.framework` | Nenhuma instalação de driver manual necessária. |
| **Android** | Android USB Host API / HID | Suporte via USB OTG | Nenhuma instalação de driver necessária. |
| **iOS** | CoreBluetooth / USB Native | Apple External Accessory | Nenhuma instalação de driver necessária. |

---

## 🔒 Diagnóstico de Permissões e Conexão USB

### Verificar a Enumeração USB no Linux:
```bash
lsusb -v -d 1209:0001
```

### Regra udev de Produção (`/etc/udev/rules.d/70-openkey.rules`):
```udev
# OpenKey FIDO2 / CTAP2 Security Key
KERNEL=="hidraw*", SUBSYSTEM=="hidraw", ATTRS{idVendor}=="1209", ATTRS{idProduct}=="0001", MODE="0660", GROUP="plugdev", TAG+="uaccess"
```
