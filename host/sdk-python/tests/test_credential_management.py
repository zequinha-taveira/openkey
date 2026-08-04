import unittest
import hashlib

from openkey.credential_management import (
    CredentialManagementClient,
    CredentialManagementError,
    RpInfo,
    UserInfo,
)
from openkey.pin import PIN_PROTOCOL_V1, PIN_PROTOCOL_V2, pin_hash
from openkey.pin_client import PinClient
from openkey.exceptions import CtapError

from test_pin import FakeAuthenticatorPin


def _make_cm(pin: str = "1234", version: int = PIN_PROTOCOL_V1):
    auth = FakeAuthenticatorPin(version)
    ctap2 = auth.make_ctap2_client()
    pin_client = PinClient(ctap2, protocol_version=version)
    pin_client.get_key_agreement()
    pin_client.set_pin(pin)
    cm = CredentialManagementClient(ctap2, pin_client, pin=pin)
    return auth, ctap2, pin_client, cm


class TestCredentialManagementClient(unittest.TestCase):

    def test_get_metadata_empty(self):
        auth, ctap2, pin_client, cm = _make_cm()
        metadata = cm.get_metadata()
        self.assertEqual(metadata["existing_count"], 0)
        self.assertEqual(metadata["max_count"], 32)

    def test_enumerate_rps_multiple(self):
        auth, ctap2, pin_client, cm = _make_cm()
        auth.add_credential("example.com", b"u1", "User 1", b"\x00" * 16 + b"\x01")
        auth.add_credential("example.com", b"u2", "User 2", b"\x00" * 16 + b"\x02")
        auth.add_credential("other.org", b"u3", "User 3", b"\x00" * 16 + b"\x03")

        rps = cm.enumerate_rps()
        self.assertEqual([rp.id for rp in rps], ["example.com", "other.org"])
        self.assertEqual(rps[0].name, "example.com")

    def test_enumerate_rps_empty(self):
        auth, ctap2, pin_client, cm = _make_cm()
        self.assertEqual(cm.enumerate_rps(), [])

    def test_enumerate_credentials(self):
        auth, ctap2, pin_client, cm = _make_cm()
        auth.add_credential("example.com", b"u1", "Alice", b"\x11" * 16)
        auth.add_credential("example.com", b"u2", "Bob", b"\x22" * 16)
        auth.add_credential("other.org", b"u3", "Carol", b"\x33" * 16)

        creds = cm.enumerate_credentials("example.com")
        self.assertEqual(len(creds), 2)
        ids = [c.credential_id for c in creds]
        self.assertEqual(ids, [b"\x11" * 16, b"\x22" * 16])
        self.assertEqual(creds[0].credential_type, "public-key")
        self.assertEqual(creds[0].rp_id, "example.com")
        self.assertEqual(creds[0].user.id, b"u1")
        self.assertEqual(creds[0].user.name, "Alice")

    def test_enumerate_credentials_empty(self):
        auth, ctap2, pin_client, cm = _make_cm()
        self.assertEqual(cm.enumerate_credentials("example.com"), [])

    def test_delete_credential(self):
        auth, ctap2, pin_client, cm = _make_cm()
        auth.add_credential("example.com", b"u1", "Alice", b"\x11" * 16)
        auth.add_credential("example.com", b"u2", "Bob", b"\x22" * 16)

        cm.delete_credential(b"\x11" * 16, "example.com")
        creds = cm.enumerate_credentials("example.com")
        self.assertEqual([c.credential_id for c in creds], [b"\x22" * 16])
        metadata = cm.get_metadata()
        self.assertEqual(metadata["existing_count"], 1)

    def test_delete_missing_credential_raises(self):
        auth, ctap2, pin_client, cm = _make_cm()
        with self.assertRaises(CtapError):
            cm.delete_credential(b"\x99" * 16, "example.com")

    def test_full_flow_v2(self):
        auth, ctap2, pin_client, cm = _make_cm(version=PIN_PROTOCOL_V2)
        auth.add_credential("example.com", b"u1", "Alice", b"\x11" * 16)
        metadata = cm.get_metadata()
        self.assertEqual(metadata["existing_count"], 1)
        rps = cm.enumerate_rps()
        self.assertEqual(rps[0].id, "example.com")
        creds = cm.enumerate_credentials("example.com")
        self.assertEqual(len(creds), 1)
        cm.delete_credential(b"\x11" * 16, "example.com")
        self.assertEqual(cm.get_metadata()["existing_count"], 0)


class TestCredentialManagementParsing(unittest.TestCase):

    def test_rp_info_from_cose_map(self):
        rp = RpInfo.from_cose_map({1: "example.com", 2: "Example"})
        self.assertEqual(rp.id, "example.com")
        self.assertEqual(rp.name, "Example")

    def test_user_info_from_cose_map(self):
        user = UserInfo.from_cose_map({1: b"uid", 2: "alice", 3: "Alice"})
        self.assertEqual(user.id, b"uid")
        self.assertEqual(user.display_name, "Alice")

    def test_invalid_response_raises(self):
        from openkey.ctap2 import Ctap2Client
        from openkey.credential_management import CredentialManagementClient
        from openkey.pin import PIN_PROTOCOL_V1
        from openkey.pin_client import PinClient

        def send(cmd, payload):
            return b"\x00"  # apenas status, sem payload CBOR

        ctap2 = Ctap2Client(send)
        pin_client = PinClient(ctap2, protocol_version=PIN_PROTOCOL_V1)
        cm = CredentialManagementClient(ctap2, pin_client, pin="1234")
        cm._token = b"\x00" * 32  # evita handshake real
        with self.assertRaises(CredentialManagementError):
            cm._cmd(0x01, b"\x01")


if __name__ == "__main__":
    unittest.main()
