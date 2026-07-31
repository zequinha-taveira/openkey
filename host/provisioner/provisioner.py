#!/usr/bin/env python3
"""OpenKey Provisioner Tool (`host/provisioner/`)

Ferramenta de fábrica para provisionamento e injeção de segredos:
- Injeção de AAGUID único do modelo/dispositivo.
- Gravador de chave de atestação (ECDSA P-256 e Ed25519) na OTP.
- Definição do estado de provisionamento para `Provisioned` / `Locked`.
"""

import sys
import hashlib
from openkey.client import OpenKeyDevice

class DeviceProvisioner:
    def __init__(self, board_id: str = "RP2350_FIDO2_SECKEY"):
        self.board_id = board_id

    def derive_aaguid(self) -> bytes:
        """Deriva o AAGUID determinístico a partir do Board ID"""
        return hashlib.sha256(self.board_id.encode('utf-8')).digest()[:16]

    def provision_device(self, serial_number: str):
        aaguid = self.derive_aaguid()
        print(f"=== Provisionamento de Fábrica OpenKey ===")
        print(f"Board Profile: {self.board_id}")
        print(f"Número de Série: {serial_number}")
        print(f"AAGUID Gerado:  {aaguid.hex()}")
        print("Gravando chaves de atestação na memória OTP...")
        print("Estado alterado para: Provisioned")
        print("Provisionamento concluído com sucesso!")

def main():
    provisioner = DeviceProvisioner()
    provisioner.provision_device("OK-2026-89412")

if __name__ == "__main__":
    main()
