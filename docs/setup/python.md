# Python Setup & Host SDK (`docs/setup/python.md`)

## 📌 Objetivo

Configurar o ambiente Python para desenvolver, testar e utilizar o **OpenKey Python SDK** (`host/sdk`), scripts de diagnósticos e ferramentas de automação.

---

## 🐍 Requisitos de Ambiente

- **Python 3.10** ou superior.
- Utilitário `pip` atualizado.
- Módulo `venv` para criação de ambientes virtuais isolados.

---

## ⚙️ Configuração do Ambiente Virtual

É altamente recomendado utilizar um ambiente virtual (`venv`) dedicado no diretório do projeto:

### No Linux / macOS:

```bash
# Navegar até o diretório do SDK
cd host/sdk

# Criar o ambiente virtual
python3 -m venv .venv

# Ativar o ambiente virtual
source .venv/bin/activate

# Atualizar pip e setuptools
pip install --upgrade pip setuptools wheel
```

### No Windows (PowerShell):

```powershell
# Navegar até o diretório do SDK
cd host\sdk

# Criar o ambiente virtual
python -m venv .venv

# Ativar o ambiente virtual
.\.venv\Scripts\Activate.ps1

# Atualizar pip
pip install --upgrade pip setuptools wheel
```

---

## 📦 Instalação do OpenKey SDK em Modo Desenvolvimento

Para instalar o SDK em modo editável (`-e`), permitindo que alterações de código reflitam imediatamente:

```bash
pip install -e .[dev]
```

### Dependências Principais do SDK
- `fido2` (`python-fido2`): Suporte ao protocolo CTAP2/WebAuthn.
- `hidapi`: Comunicação de baixo nível com a interface USB HID.
- `pyscard`: Suporte à interface CCID/PC-SC.
- `pytest`: Suíte de testes automatizados.

---

## 🧪 Executando Testes Automatizados Python

Para executar os testes do SDK contra o **Simulador de Software** ou contra um dispositivo **RP2350** físico conectado:

```bash
# Executar a suíte completa de testes com pytest
pytest

# Executar com detalhamento de logs
pytest -v -s
```
