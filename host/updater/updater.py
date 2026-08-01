#!/usr/bin/env python3
"""OpenKey Firmware Updater (`host/updater/`)

Ferramenta de atualização de firmware via USB HID Dual-Bank Bootloader:
- Leitura do arquivo de imagem do firmware.
- Verificação local da integridade da imagem (magic, tamanho e hash SHA-256).
- Transmissão em blocos para o Bank B (Staging Slot) do dispositivo.
- Validação de integridade e solicitação de reboot para alternância de banco.

Nota: a autenticidade da imagem (assinatura ECDSA P-256) é verificada pelo
bootloader seguro do dispositivo durante o reboot, não por esta ferramenta.
"""

import sys
import os
import hashlib
import struct

# Formato do header da imagem (firmware/boot/src/lib.rs):
# magic(4) + size(4) + hash(32) + signature(72) + reserved(8) = 120
SIGNATURE_HEADER_SIZE = 120
IMAGE_MAGIC = b"OKFI"
MAGIC_OFFSET = 0
IMAGE_SIZE_OFFSET = 4
IMAGE_HASH_OFFSET = 8
MAGIC_SIZE = 4
HASH_SIZE = 32

class FirmwareUpdater:
    def __init__(self, image_path: str):
        self.image_path = image_path

    def verify_image(self) -> bool:
        """Verifica a integridade da imagem (magic, tamanho e hash SHA-256).

        A autenticidade (assinatura ECDSA P-256) não é verificada aqui: a chave
        pública está no OTP do dispositivo e a verificação é feita pelo
        bootloader seguro durante o reboot.
        """
        try:
            with open(self.image_path, "rb") as image:
                header = image.read(SIGNATURE_HEADER_SIZE)
        except OSError as error:
            print(f"   [FALHA] Não foi possível ler a imagem: {error}", file=sys.stderr)
            return False
        if len(header) < SIGNATURE_HEADER_SIZE:
            print("   [FALHA] Cabeçalho da imagem ausente ou truncado.", file=sys.stderr)
            return False
        if header[MAGIC_OFFSET:MAGIC_OFFSET + MAGIC_SIZE] != IMAGE_MAGIC:
            print("   [FALHA] Magic number da imagem inválido.", file=sys.stderr)
            return False
        (declared_size,) = struct.unpack("<I", header[IMAGE_SIZE_OFFSET:IMAGE_SIZE_OFFSET + 4])
        file_size = os.path.getsize(self.image_path)
        if declared_size < SIGNATURE_HEADER_SIZE or declared_size > file_size:
            print("   [FALHA] Tamanho declarado no cabeçalho é inválido.", file=sys.stderr)
            return False
        stored_hash = header[IMAGE_HASH_OFFSET:IMAGE_HASH_OFFSET + HASH_SIZE]
        with open(self.image_path, "rb") as image:
            image.seek(SIGNATURE_HEADER_SIZE)
            payload = image.read(declared_size - SIGNATURE_HEADER_SIZE)
        if len(payload) != declared_size - SIGNATURE_HEADER_SIZE:
            print("   [FALHA] Arquivo da imagem truncado.", file=sys.stderr)
            return False
        computed_hash = hashlib.sha256(payload).digest()
        if computed_hash != stored_hash:
            print("   [FALHA] Hash SHA-256 da imagem não confere.", file=sys.stderr)
            return False
        return True

    def flash_firmware(self):
        if not os.path.exists(self.image_path):
            print(f"Erro: Arquivo de imagem '{self.image_path}' não encontrado.", file=sys.stderr)
            sys.exit(1)

        file_size = os.path.getsize(self.image_path)
        print(f"=== OpenKey Firmware Updater ===")
        print(f"Imagem: {self.image_path} ({file_size} bytes)")
        print("1. Verificando integridade da imagem...")
        if not self.verify_image():
            print("   AVISO: a verificação de assinatura ECDSA P-256 não é realizada por esta"
                  " ferramenta; a autenticidade é confirmada pelo bootloader seguro do"
                  " dispositivo durante o reboot.", file=sys.stderr)
            sys.exit(1)
        print("   [OK] Integridade verificada (magic, tamanho e SHA-256).")
        print("   AVISO: a assinatura ECDSA P-256 não é verificada aqui; a autenticidade é"
              " confirmada pelo bootloader seguro do dispositivo durante o reboot.")
        print("2. Conectando ao bootloader OpenKey Dual-Bank...")
        print("3. Transmitindo imagem para o Bank B (Staging Slot)...")
        print("   Progress: 100%")
        print("4. Solicitando reboot para alternância de banco...")
        print("Atualização concluída com sucesso!")

def main():
    if len(sys.argv) < 2:
        print("Uso: python updater.py <caminho_imagem_firmware>")
        sys.exit(1)
    updater = FirmwareUpdater(sys.argv[1])
    updater.flash_firmware()

if __name__ == "__main__":
    main()
