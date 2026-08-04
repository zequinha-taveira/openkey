"""Testes do OpenKey Provisioner (sem hardware)."""

import hashlib

from provisioner import DeviceProvisioner

DEFAULT_BOARD = "RP2350_FIDO2_SECKEY"


def test_derive_aaguid_is_16_bytes():
    aaguid = DeviceProvisioner().derive_aaguid()
    assert isinstance(aaguid, bytes)
    assert len(aaguid) == 16


def test_derive_aaguid_is_deterministic():
    p = DeviceProvisioner()
    assert p.derive_aaguid() == p.derive_aaguid()


def test_derive_aaguid_matches_sha256_prefix():
    expected = hashlib.sha256(DEFAULT_BOARD.encode("utf-8")).digest()[:16]
    assert DeviceProvisioner().derive_aaguid() == expected


def test_derive_aaguid_changes_with_board_id():
    a = DeviceProvisioner("BOARD_A").derive_aaguid()
    b = DeviceProvisioner("BOARD_B").derive_aaguid()
    assert a != b


def test_default_board_id():
    assert DeviceProvisioner().board_id == DEFAULT_BOARD
