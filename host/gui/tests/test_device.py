import unittest

from openkey_manager.core.device import DeviceBackend, DeviceController, DeviceError
from openkey_manager.core.models import ConnectionState, DeviceCandidate


class FakeCredentialManager:
    """Fake do CredentialManagementClient do openkey-sdk.

    ``credentials``: lista de tuplas
    ``(rp_id, rp_name, credential_id, user_id, user_name, user_display_name)``.
    """

    def __init__(self, credentials=None):
        self._creds = []
        for rp_id, rp_name, cred_id, uid, uname, udisp in (credentials or []):
            self._creds.append(
                {
                    "rp_id": rp_id,
                    "rp_name": rp_name,
                    "credential_id": cred_id,
                    "user_id": uid,
                    "user_name": uname,
                    "user_display_name": udisp,
                }
            )
        self.deleted = []

    def enumerate_rps(self):
        from openkey.credential_management import RpInfo

        seen = {}
        for c in self._creds:
            seen.setdefault(c["rp_id"], c["rp_name"])
        return [RpInfo(id=rp_id, name=name) for rp_id, name in seen.items()]

    def enumerate_credentials(self, rp_id):
        from openkey.credential_management import CredentialInfo, RpInfo, UserInfo

        result = []
        for c in self._creds:
            if c["rp_id"] == rp_id:
                result.append(
                    CredentialInfo(
                        credential_id=c["credential_id"],
                        rp_id=rp_id,
                        user=UserInfo(
                            id=c["user_id"],
                            name=c["user_name"],
                            display_name=c["user_display_name"],
                        ),
                        rp=RpInfo(id=rp_id, name=c["rp_name"]),
                    )
                )
        return result

    def delete_credential(self, credential_id, rp_id):
        self.deleted.append((credential_id, rp_id))
        before = len(self._creds)
        self._creds = [
            c
            for c in self._creds
            if not (c["credential_id"] == credential_id and c["rp_id"] == rp_id)
        ]
        if len(self._creds) == before:
            raise ValueError("credential not found")


class FakeGetInfo:
    aaguid = b"\x01" * 16
    versions = ["FIDO_2_0", "FIDO_2_1"]
    options = {"rk": True, "clientPin": True, "credentialMgmt": True}
    max_msg_size = 1200
    pin_uv_auth_protocols = [1, 2]
    extensions = ["credProtect"]


class FakeCtap2:
    def __init__(self):
        self.pin_requests = 0


class FakeDevice:
    def __init__(self):
        self._ctap2 = FakeCtap2()
        self._backend = FakeTransport()
        self.get_info_calls = 0
        self.reset_calls = 0

    def get_info(self):
        self.get_info_calls += 1
        return FakeGetInfo()

    def reset(self):
        self.reset_calls += 1

    def connect(self):
        pass


class FakeTransport:
    def __init__(self):
        self.closed = False

    def close(self):
        self.closed = True


class FakePinClient:
    def __init__(self, protocol_version):
        self.protocol_version = protocol_version
        self.key_agreements = 0
        self.set_pins = []
        self.changes = []
        self.retries = 5
        self.fail_set = None
        self.fail_change = None
        self.fail_retries = None

    def get_key_agreement(self):
        self.key_agreements += 1

    def set_pin(self, new_pin):
        if self.fail_set is not None:
            raise self.fail_set
        self.set_pins.append(new_pin)

    def change_pin(self, old_pin, new_pin):
        if self.fail_change is not None:
            raise self.fail_change
        self.changes.append((old_pin, new_pin))

    def get_pin_retries(self):
        if self.fail_retries is not None:
            raise self.fail_retries
        return self.retries


class FakeBackend(DeviceBackend):
    def __init__(self, devices=None, credential_manager=None):
        self.devices = devices if devices is not None else [
            DeviceCandidate(
                vid=0x16C0,
                pid=0x27DB,
                serial_number="123456",
                product_string="OpenKey",
            )
        ]
        self.cm = (
            credential_manager
            if credential_manager is not None
            else FakeCredentialManager()
        )
        self.opened = []
        self.closed = []
        self.fail_open = None
        self.fail_info = None
        self.pin_clients = []
        self.last_cm_pin = None
        self.last_cm_protocol = None
        self.pin_fail_set = None
        self.pin_fail_change = None
        self.pin_fail_retries = None

    def discover(self, vid=None, pid=None, serial_number=None):
        return [self._clone(d) for d in self.devices]

    @staticmethod
    def _clone(candidate):
        return DeviceCandidate(**candidate.as_dict())

    def open(self, candidate=None, *, vid=None, pid=None, serial_number=None, path=None):
        if self.fail_open:
            raise RuntimeError("open failed")
        device = FakeDevice()
        self.opened.append((candidate, device))
        return device

    def close(self, device):
        self.closed.append(device)
        device._backend.close()

    def get_info(self, device):
        if self.fail_info:
            raise RuntimeError("info failed")
        return device.get_info()

    def reset(self, device):
        device.reset()

    def pin_client(self, device, protocol_version):
        client = FakePinClient(protocol_version)
        client.fail_set = self.pin_fail_set
        client.fail_change = self.pin_fail_change
        client.fail_retries = self.pin_fail_retries
        self.pin_clients.append(client)
        return client

    def credential_manager(self, device, pin, protocol_version):
        self.last_cm_pin = pin
        self.last_cm_protocol = protocol_version
        return self.cm

    def ctap2(self, device):
        return device._ctap2


class TestDiscover(unittest.TestCase):

    def test_discover_returns_candidates(self):
        backend = FakeBackend()
        controller = DeviceController(backend=backend)
        candidates = controller.discover()
        self.assertEqual(len(candidates), 1)
        self.assertEqual(candidates[0].serial_number, "123456")


class TestConnect(unittest.TestCase):

    def test_connect_success_maps_info(self):
        backend = FakeBackend()
        controller = DeviceController(backend=backend)
        candidate = DeviceCandidate(vid=0x16C0, pid=0x27DB, serial_number="123456")

        info = controller.connect(candidate)

        self.assertTrue(controller.is_connected)
        self.assertEqual(controller.state, ConnectionState.CONNECTED)
        self.assertEqual(info.aaguid_hex, "01" * 16)
        self.assertEqual(info.vid, 0x16C0)
        self.assertEqual(info.serial_number, "123456")
        self.assertTrue(info.supports_resident_keys)
        self.assertTrue(info.supports_pin)
        self.assertEqual(backend.opened[0][0], candidate)

    def test_connect_failure_sets_error_state(self):
        backend = FakeBackend()
        backend.fail_open = "boom"
        controller = DeviceController(backend=backend)

        with self.assertRaises(DeviceError):
            controller.connect()
        self.assertEqual(controller.state, ConnectionState.ERROR)
        self.assertFalse(controller.is_connected)

    def test_connect_info_failure_closes_and_errors(self):
        backend = FakeBackend()
        backend.fail_info = "boom"
        controller = DeviceController(backend=backend)

        with self.assertRaises(DeviceError):
            controller.connect()
        self.assertEqual(controller.state, ConnectionState.ERROR)
        self.assertEqual(len(backend.closed), 1)


class TestOperations(unittest.TestCase):

    def _connected_controller(self):
        backend = FakeBackend()
        controller = DeviceController(backend=backend)
        controller.connect()
        return backend, controller

    def test_get_info_requires_connection(self):
        controller = DeviceController(backend=FakeBackend())
        with self.assertRaises(DeviceError):
            controller.get_info()

    def test_reset(self):
        backend, controller = self._connected_controller()
        controller.reset()
        self.assertEqual(backend.opened[0][1].reset_calls, 1)

    def test_reset_requires_connection(self):
        controller = DeviceController(backend=FakeBackend())
        with self.assertRaises(DeviceError):
            controller.reset()

    def test_disconnect(self):
        backend, controller = self._connected_controller()
        device = backend.opened[0][1]
        controller.disconnect()
        self.assertEqual(controller.state, ConnectionState.DISCONNECTED)
        self.assertFalse(controller.is_connected)
        self.assertIn(device, backend.closed)

    def test_setup_pin(self):
        backend, controller = self._connected_controller()
        controller.setup_pin("1234")
        self.assertEqual(backend.pin_clients[0].set_pins, ["1234"])
        self.assertEqual(backend.pin_clients[0].key_agreements, 1)

    def test_change_pin(self):
        backend, controller = self._connected_controller()
        controller.change_pin("1234", "5678")
        self.assertEqual(backend.pin_clients[0].changes, [("1234", "5678")])

    def test_get_pin_retries(self):
        backend, controller = self._connected_controller()
        self.assertEqual(controller.get_pin_retries(), 5)

    def test_preferred_protocol_uses_v2(self):
        backend, controller = self._connected_controller()
        controller.setup_pin("1234")
        self.assertEqual(backend.pin_clients[0].protocol_version, 2)


class TestListeners(unittest.TestCase):

    def test_state_transitions_notified(self):
        backend = FakeBackend()
        controller = DeviceController(backend=backend)
        events = []
        controller.add_listener(lambda state, msg: events.append((state, msg)))

        controller.connect()
        controller.disconnect()

        self.assertEqual(
            events[0][0], ConnectionState.CONNECTING
        )
        self.assertEqual(events[1][0], ConnectionState.CONNECTED)
        self.assertEqual(events[-1][0], ConnectionState.DISCONNECTED)


if __name__ == "__main__":
    unittest.main()
