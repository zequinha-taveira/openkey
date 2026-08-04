import unittest

from openkey_manager.core.device import DeviceBackend
from openkey_manager.core.discovery import DiscoveryService
from openkey_manager.core.models import DeviceCandidate

from test_device import FakeBackend


def _candidate(serial: str, path=None):
    return DeviceCandidate(
        vid=0x16C0, pid=0x27DB, serial_number=serial, path=path
    )


class TestDiscoveryService(unittest.TestCase):

    def test_refresh_initial_snapshot(self):
        backend = FakeBackend(devices=[_candidate("A"), _candidate("B")])
        service = DiscoveryService(backend=backend)

        devices = service.refresh()
        self.assertEqual(len(devices), 2)
        self.assertEqual(len(service.snapshot()), 2)

    def test_attach_detected(self):
        backend = FakeBackend(devices=[_candidate("A")])
        service = DiscoveryService(backend=backend)
        service.refresh()

        backend.devices.append(_candidate("B"))
        events = []
        service.add_listener(lambda attached, detached: events.append((attached, detached)))

        devices = service.refresh()
        self.assertEqual(len(devices), 2)
        self.assertEqual(len(events), 1)
        attached, detached = events[0]
        self.assertEqual([c.serial_number for c in attached], ["B"])
        self.assertEqual(detached, [])

    def test_detach_detected(self):
        backend = FakeBackend(devices=[_candidate("A"), _candidate("B")])
        service = DiscoveryService(backend=backend)
        service.refresh()

        backend.devices = [_candidate("A")]
        events = []
        service.add_listener(lambda attached, detached: events.append((attached, detached)))

        service.refresh()
        self.assertEqual(len(events), 1)
        attached, detached = events[0]
        self.assertEqual(attached, [])
        self.assertEqual([c.serial_number for c in detached], ["B"])

    def test_no_change_no_notification(self):
        backend = FakeBackend(devices=[_candidate("A")])
        service = DiscoveryService(backend=backend)
        service.refresh()

        events = []
        service.add_listener(lambda attached, detached: events.append((attached, detached)))
        service.refresh()
        self.assertEqual(events, [])

    def test_key_by_path(self):
        backend = FakeBackend(
            devices=[_candidate("A", path=b"\x00\x01"), _candidate("B", path=b"\x00\x02")]
        )
        service = DiscoveryService(backend=backend)
        service.refresh()
        self.assertEqual(len(service.snapshot()), 2)


if __name__ == "__main__":
    unittest.main()
