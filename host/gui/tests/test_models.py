import unittest

from openkey_manager.core.models import (
    ConnectionState,
    Credential,
    DeviceCandidate,
    DeviceInfo,
    DiagnosticsReport,
    UpdateSession,
    UpdateStage,
)


class TestDeviceCandidate(unittest.TestCase):

    def test_label_with_serial(self):
        candidate = DeviceCandidate(vid=0x16C0, pid=0x27DB, serial_number="123456")
        self.assertEqual(candidate.label, "OpenKey 123456")

    def test_label_without_serial(self):
        candidate = DeviceCandidate(vid=0x16C0, pid=0x27DB)
        self.assertEqual(candidate.label, "OpenKey 16C0:27DB")

    def test_vid_pid(self):
        candidate = DeviceCandidate(vid=0x16C0, pid=0x27DB)
        self.assertEqual(candidate.vid_pid, "16C0:27DB")

    def test_as_dict(self):
        candidate = DeviceCandidate(
            vid=0x16C0, pid=0x27DB, serial_number="123456", path=b"\\\\.\\hid"
        )
        data = candidate.as_dict()
        self.assertEqual(data["vid"], 0x16C0)
        self.assertEqual(data["serial_number"], "123456")
        self.assertEqual(data["path"], b"\\\\.\\hid")


class TestDeviceInfo(unittest.TestCase):

    def _info(self, **overrides):
        base = {
            "aaguid": b"\x01" * 16,
            "versions": ["FIDO_2_0"],
            "options": {"rk": True, "clientPin": True, "credentialMgmt": True},
        }
        base.update(overrides)
        return DeviceInfo(**base)

    def test_properties(self):
        info = self._info()
        self.assertEqual(info.aaguid_hex, "01" * 16)
        self.assertTrue(info.supports_resident_keys)
        self.assertTrue(info.supports_pin)
        self.assertTrue(info.supports_credential_management)

    def test_no_resident_keys(self):
        info = self._info(options={"rk": False})
        self.assertFalse(info.supports_resident_keys)

    def test_no_pin(self):
        info = self._info(options={"rk": False})
        self.assertFalse(info.supports_pin)

    def test_label_with_serial(self):
        info = self._info(serial_number="ABC")
        self.assertEqual(info.label, "OpenKey ABC")

    def test_label_without_serial(self):
        info = self._info()
        self.assertEqual(info.label, "OpenKey 01010101")


class TestCredential(unittest.TestCase):

    def test_display_name_priority(self):
        cred = Credential(
            rp_id="example.com",
            credential_id=b"\x11" * 16,
            user_name="alice",
            user_display_name="Alice",
        )
        self.assertEqual(cred.display_name, "Alice")
        self.assertEqual(cred.credential_id_hex, "11" * 16)

    def test_display_name_fallback(self):
        cred = Credential(rp_id="example.com", credential_id=b"\x22" * 16)
        self.assertEqual(cred.display_name, "example.com")


class TestDiagnosticsReport(unittest.TestCase):

    def test_counts(self):
        report = DiagnosticsReport(
            checks={"flash": True, "random": True, "secrets": False}
        )
        self.assertEqual(report.passed_checks, 2)
        self.assertEqual(report.failed_checks, 1)


class TestUpdateSession(unittest.TestCase):

    def test_defaults(self):
        session = UpdateSession()
        self.assertEqual(session.stage, UpdateStage.IDLE)
        self.assertEqual(session.progress, 0.0)
        self.assertIsNone(session.error)


class TestConnectionState(unittest.TestCase):

    def test_values(self):
        self.assertEqual(ConnectionState.CONNECTED.value, "connected")
        self.assertEqual(ConnectionState.DISCONNECTED.value, "disconnected")


if __name__ == "__main__":
    unittest.main()
