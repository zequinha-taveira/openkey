"""Configuração compartilhada dos testes do OpenKey Manager.

Força o backend Qt a rodar em modo *offscreen* para permitir testes de widgets
headless na CI (ADR-0013).
"""

import os

os.environ.setdefault("QT_QPA_PLATFORM", "offscreen")
