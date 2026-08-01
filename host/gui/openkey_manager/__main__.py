"""Permite executar com ``python -m openkey_manager``."""

import sys

from openkey_manager.app import main

if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
