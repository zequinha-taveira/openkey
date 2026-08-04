"""Testes do OpenKey CLI — subcomandos que não exigem hardware."""

import sys

from openkey_cli import main


def run_cli(monkeypatch, capsys, argv):
    monkeypatch.setattr(sys, "argv", ["openkey-cli"] + argv)
    try:
        main()
        return None, capsys.readouterr().out
    except SystemExit as exc:
        return exc.code, capsys.readouterr().out


def test_sem_subcomando_mostra_uso_e_sai_1(monkeypatch, capsys):
    code, out = run_cli(monkeypatch, capsys, [])
    assert code == 1
    assert "usage" in out


def test_pin_set(monkeypatch, capsys):
    code, out = run_cli(monkeypatch, capsys, ["pin", "set"])
    assert code is None
    assert "PIN definido com sucesso" in out


def test_credentials_list(monkeypatch, capsys):
    code, out = run_cli(monkeypatch, capsys, ["credentials", "list"])
    assert code is None
    assert "Nenhuma credencial residente" in out


def test_update_sem_imagem_sai_2(monkeypatch, capsys):
    code, _ = run_cli(monkeypatch, capsys, ["update"])
    assert code == 2


def test_update_ok(monkeypatch, capsys):
    code, out = run_cli(monkeypatch, capsys, ["update", "--image", "fw.bin"])
    assert code is None
    assert "Atualização concluída com sucesso" in out
