"""Testes do diálogo de PIN (validação + modos set/change)."""

import pytest

from PySide6.QtWidgets import QDialog

from openkey_manager.ui.pin_dialog import PinDialog, PinMode


def _dialog(mode=PinMode.SET):
    dialog = PinDialog(mode)
    return dialog


def test_set_mode_hides_current_field():
    dialog = _dialog(PinMode.SET)
    assert dialog._current_edit is None


def test_change_mode_shows_current_field():
    dialog = _dialog(PinMode.CHANGE)
    assert dialog._current_edit is not None


def test_set_accepts_with_matching_confirmation():
    dialog = _dialog(PinMode.SET)
    dialog._new_edit.setText("1234")
    dialog._confirm_edit.setText("1234")
    dialog.accept()
    assert dialog.result() == QDialog.Accepted
    assert dialog.new_pin == "1234"
    assert dialog.current_pin is None


def test_change_accepts_with_current_and_matching():
    dialog = _dialog(PinMode.CHANGE)
    dialog._current_edit.setText("1234")
    dialog._new_edit.setText("5678")
    dialog._confirm_edit.setText("5678")
    dialog.accept()
    assert dialog.result() == QDialog.Accepted
    assert dialog.current_pin == "1234"
    assert dialog.new_pin == "5678"


def test_mismatched_confirmation_rejected():
    dialog = _dialog(PinMode.SET)
    dialog._new_edit.setText("1234")
    dialog._confirm_edit.setText("4321")
    dialog.accept()
    assert dialog.result() != QDialog.Accepted
    assert "não confere" in dialog._error_label.text()


def test_too_short_pin_rejected():
    dialog = _dialog(PinMode.SET)
    dialog._new_edit.setText("12")
    dialog._confirm_edit.setText("12")
    dialog.accept()
    assert dialog.result() != QDialog.Accepted
    assert "pelo menos 4" in dialog._error_label.text()


def test_empty_pin_rejected():
    dialog = _dialog(PinMode.SET)
    dialog._new_edit.setText("")
    dialog._confirm_edit.setText("")
    dialog.accept()
    assert dialog.result() != QDialog.Accepted


def test_change_requires_current_pin():
    dialog = _dialog(PinMode.CHANGE)
    dialog._new_edit.setText("5678")
    dialog._confirm_edit.setText("5678")
    dialog.accept()
    assert dialog.result() != QDialog.Accepted
    assert "PIN atual" in dialog._error_label.text()
