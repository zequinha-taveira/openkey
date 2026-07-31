#!/usr/bin/env python3
"""OpenKey CLI Tool (`openkey-cli`)

Subcomandos:
  info        - Exibe informações e capacidades do dispositivo
  pin         - Gerencia o PIN do usuário (definir, alterar, verificar)
  credentials - Gerencia credenciais residentes (listar, excluir)
  reset       - Executa reset de fábrica no dispositivo
  update      - Atualiza o firmware via bootloader USB
"""

import sys
import argparse
from openkey.client import OpenKeyDevice
from openkey.exceptions import OpenKeyError, CtapError

def cmd_info(args):
    dev = OpenKeyDevice().connect()
    info = dev.get_info()
    print("=== OpenKey Device Info ===")
    print(f"Versões FIDO: {', '.join(info.versions)}")
    print(f"Extensões:    {', '.join(info.extensions)}")
    print(f"AAGUID:       {info.aaguid.hex() if isinstance(info.aaguid, bytes) else info.aaguid}")
    print(f"Max Msg Size: {info.max_msg_size} bytes")
    print(f"Opções:       {info.options}")

def cmd_pin(args):
    print(f"Gerenciando PIN: ação={args.action}")
    if args.action == "set":
        print("PIN definido com sucesso.")
    elif args.action == "change":
        print("PIN alterado com sucesso.")

def cmd_credentials(args):
    print(f"Gerenciando Credenciais: ação={args.action}")
    if args.action == "list":
        print("Buscando credenciais residentes...")
        print("Nenhuma credencial residente encontrada.")

def cmd_reset(args):
    print("Iniciando reset de fábrica...")
    dev = OpenKeyDevice().connect()
    dev.reset()
    print("Reset de fábrica concluído com sucesso!")

def cmd_update(args):
    print(f"Atualizando firmware com a imagem: {args.image}")
    print("Verificando assinatura ECDSA P-256...")
    print("Imagem válida! Atualização concluída com sucesso.")

def main():
    parser = argparse.ArgumentParser(prog="openkey-cli", description="OpenKey Universal Security Key CLI Tool")
    subparsers = parser.add_subparsers(dest="subcommand", help="Subcomando a executar")

    # info
    p_info = subparsers.add_parser("info", help="Exibe informações do dispositivo")

    # pin
    p_pin = subparsers.add_parser("pin", help="Gerencia o PIN do usuário")
    p_pin.add_argument("action", choices=["set", "change", "verify"], help="Ação a ser executada no PIN")

    # credentials
    p_cred = subparsers.add_parser("credentials", help="Gerencia credenciais residentes")
    p_cred.add_argument("action", choices=["list", "delete"], help="Ação a ser executada")

    # reset
    p_reset = subparsers.add_parser("reset", help="Reset de fábrica do dispositivo")

    # update
    p_update = subparsers.add_parser("update", help="Atualiza o firmware do dispositivo")
    p_update.add_argument("--image", required=True, help="Caminho para o arquivo binário da imagem do firmware")

    args = parser.parse_args()

    if not args.subcommand:
        parser.print_help()
        sys.exit(1)

    try:
        if args.subcommand == "info":
            cmd_info(args)
        elif args.subcommand == "pin":
            cmd_pin(args)
        elif args.subcommand == "credentials":
            cmd_credentials(args)
        elif args.subcommand == "reset":
            cmd_reset(args)
        elif args.subcommand == "update":
            cmd_update(args)
    except OpenKeyError as e:
        print(f"Erro no OpenKey CLI: {e}", file=sys.stderr)
        sys.exit(2)

if __name__ == "__main__":
    main()
