#!/usr/bin/env python3
"""OpenKey Configurator (`host/configurator/`)

Ferramenta de configuração e gerenciamento visual do dispositivo OpenKey:
- Gerenciamento de preferências de aplicativo (CTAP2, CCID, OpenPGP, PIV).
- Leitura de status e diagnósticos.
- Configuração de políticas de segurança.
"""

import sys
from openkey.client import OpenKeyDevice

class OpenKeyConfigurator:
    def __init__(self):
        self.device = None

    def connect(self):
        self.device = OpenKeyDevice().connect()

    fn_run_interactive = lambda self: None

    def run_cli(self):
        print("=== OpenKey Configurator ===")
        self.connect()
        info = self.device.get_info()
        print(f"Dispositivo Conectado! AAGUID: {info.aaguid}")
        print("Opções Ativas:")
        for key, val in info.options.items():
            print(f"  - {key}: {val}")

def main():
    configurator = OpenKeyConfigurator()
    configurator.run_cli()

if __name__ == "__main__":
    main()
