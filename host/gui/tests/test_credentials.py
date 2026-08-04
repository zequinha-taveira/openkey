import unittest

from openkey_manager.core.credentials import CredentialError, CredentialService
from openkey_manager.core.device import DeviceController
from openkey_manager.core.models import Credential

from test_device import FakeBackend, FakeCredentialManager


def _credentials():
    return FakeCredentialManager(
        credentials=[
            ("example.com", "Example", b"\x11" * 16, b"u1", "alice", "Alice"),
            ("example.com", "Example", b"\x22" * 16, b"u2", "bob", "Bob"),
            ("other.org", "Other", b"\x33" * 16, b"u3", "carol", None),
        ]
    )


class TestCredentialService(unittest.TestCase):

    def _service(self, cm=None, pin_provider=None):
        backend = FakeBackend(credential_manager=cm or _credentials())
        controller = DeviceController(backend=backend)
        controller.connect()
        service = CredentialService(controller, pin_provider=pin_provider or (lambda: "1234"))
        return backend, controller, service

    def test_list_credentials_aggregates_rps(self):
        backend, controller, service = self._service()
        credentials = service.list_credentials()

        self.assertEqual(len(credentials), 3)
        self.assertEqual(backend.last_cm_pin, "1234")
        rps = {c.rp_id for c in credentials}
        self.assertEqual(rps, {"example.com", "other.org"})

    def test_list_credentials_maps_fields(self):
        backend, controller, service = self._service()
        creds = {c.credential_id: c for c in service.list_credentials()}

        alice = creds[b"\x11" * 16]
        self.assertIsInstance(alice, Credential)
        self.assertEqual(alice.rp_id, "example.com")
        self.assertEqual(alice.rp_name, "Example")
        self.assertEqual(alice.user_id, b"u1")
        self.assertEqual(alice.user_name, "alice")
        self.assertEqual(alice.user_display_name, "Alice")
        self.assertEqual(alice.credential_id_hex, "11" * 16)

        carol = creds[b"\x33" * 16]
        self.assertEqual(carol.user_display_name, None)
        self.assertEqual(carol.display_name, "carol")

    def test_delete_credential(self):
        backend, controller, service = self._service()
        service.delete_credential(b"\x11" * 16, "example.com")
        self.assertEqual(backend.cm.deleted, [(b"\x11" * 16, "example.com")])
        remaining = service.list_credentials()
        self.assertEqual(len(remaining), 2)

    def test_pin_required_without_provider(self):
        backend = FakeBackend(credential_manager=_credentials())
        controller = DeviceController(backend=backend)
        controller.connect()
        service = CredentialService(controller, pin_provider=None)
        with self.assertRaises(CredentialError):
            service.list_credentials()

    def test_pin_provider_returning_none_raises(self):
        backend = FakeBackend(credential_manager=_credentials())
        controller = DeviceController(backend=backend)
        controller.connect()
        service = CredentialService(controller, pin_provider=lambda: None)
        with self.assertRaises(CredentialError):
            service.list_credentials()

    def test_reset_session_reuses_client_then_recreates(self):
        backend, controller, service = self._service()
        service.list_credentials()
        client_1 = service._client
        self.assertIsNotNone(client_1)

        service.reset_session()
        self.assertIsNone(service._client)
        # próximo acesso recria (novo pin)
        calls = {"n": 0}

        def provider():
            calls["n"] += 1
            return "5678"

        service._pin_provider = provider
        service.list_credentials()
        self.assertEqual(backend.last_cm_pin, "5678")


if __name__ == "__main__":
    unittest.main()
