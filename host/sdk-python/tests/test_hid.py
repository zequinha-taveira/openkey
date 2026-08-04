import unittest
from unittest import mock

import openkey.hid as hid_module
from openkey.client import OpenKeyDevice
from openkey.exceptions import TransportError
from openkey.hid import (
    OPENKEY_VID,
    OPENKEY_PID,
    HidTransportBackend,
    discover_devices,
    open_device,
)
from openkey.transport import (
    CTAPHID_PACKET_SIZE,
    CTAPHID_BROADCAST_CID,
    CMD_INIT,
    CMD_CBOR,
    CMD_KEEPALIVE,
    CMD_ERROR,
    CtapHidMessageAssembler,
)


class FakeHidDevice:
    """Fake do objeto retornado por ``hid.device()``."""

    def __init__(self, responder=None):
        self.responder = responder
        self.opened = False
        self.closed = False
        self.writes = []
        self._path = None
        self._vid = None
        self._pid = None
        self._serial = None

    def open(self, vid, pid, serial_number=None):
        self.opened = True
        self._vid = vid
        self._pid = pid
        self._serial = serial_number

    def open_path(self, path):
        self.opened = True
        self._path = path

    def close(self):
        self.closed = True

    def write(self, data):
        self.writes.append(bytes(data))
        if self.responder is not None:
            return len(data)
        return len(data)

    def read(self, max_length, timeout_ms=0):
        if self.responder is None:
            return None
        return self.responder.next_report()


class FakeHidModule:
    """Fake do módulo ``hid`` (hidapi)."""

    def __init__(self, devices=None):
        self.enumerated = devices or []
        self.opened = []
        self.device_factory = None

    def enumerate(self):
        return list(self.enumerated)

    def device(self):
        if self.device_factory is not None:
            dev = self.device_factory()
        else:
            dev = FakeHidDevice()
        self.opened.append(dev)
        return dev


class ReportQueue:
    """Responde com uma sequência de relatórios (um por read)."""

    def __init__(self, reports):
        self.reports = [bytes(r) for r in reports]

    def next_report(self):
        if not self.reports:
            return None
        return self.reports.pop(0)


def hid_report(cid, cmd, payload, seq=None, total_len=None):
    """Gera um relatório HID de 64 bytes no formato CTAPHID."""
    if seq is None:
        # Init packet
        if total_len is None:
            total_len = len(payload)
        return CtapHidMessageAssembler.fragment_message(cid, cmd, payload[:57])[0]
    return CtapHidMessageAssembler.fragment_message(cid, cmd, payload)[0]


def init_response(nonce, cid=0x12345678):
    payload = nonce + cid.to_bytes(4, "big") + b"\x02\x01\x00\x00\x01"
    return payload


class TestHidDiscovery(unittest.TestCase):

    def test_discover_filters_vid_pid(self):
        fake = FakeHidModule([
            {"vendor_id": OPENKEY_VID, "product_id": OPENKEY_PID, "path": b"a"},
            {"vendor_id": 0x1234, "product_id": 0x5678, "path": b"b"},
        ])
        with mock.patch.object(hid_module, "hid", fake):
            devs = discover_devices()
        self.assertEqual(len(devs), 1)
        self.assertEqual(devs[0]["path"], b"a")

    def test_discover_no_filter_returns_all(self):
        fake = FakeHidModule([
            {"vendor_id": 1, "product_id": 2, "path": b"a"},
            {"vendor_id": 3, "product_id": 4, "path": b"b"},
        ])
        with mock.patch.object(hid_module, "hid", fake):
            devs = discover_devices(vid=None, pid=None)
        self.assertEqual(len(devs), 2)

    def test_discover_filters_serial(self):
        fake = FakeHidModule([
            {"vendor_id": OPENKEY_VID, "product_id": OPENKEY_PID,
             "serial_number": "SN1", "path": b"a"},
            {"vendor_id": OPENKEY_VID, "product_id": OPENKEY_PID,
             "serial_number": "SN2", "path": b"b"},
        ])
        with mock.patch.object(hid_module, "hid", fake):
            devs = discover_devices(serial_number="SN2")
        self.assertEqual(len(devs), 1)
        self.assertEqual(devs[0]["serial_number"], "SN2")

    def test_discover_requires_hidapi(self):
        with mock.patch.object(hid_module, "hid", None):
            with self.assertRaises(TransportError):
                discover_devices()


class TestHidBackendInit(unittest.TestCase):

    def setUp(self):
        self.nonce = b"\x01\x02\x03\x04\x05\x06\x07\x08"
        self.response_payload = init_response(self.nonce, cid=0xCAFEBABE)

    def _queue_from_payload(self, payload, cid=CTAPHID_BROADCAST_CID, cmd=CMD_INIT):
        packets = CtapHidMessageAssembler.fragment_message(cid, cmd, payload)
        return ReportQueue(packets)

    def test_open_device_performs_init(self):
        queue = ReportQueue(
            CtapHidMessageAssembler.fragment_message(
                CTAPHID_BROADCAST_CID, CMD_INIT, self.response_payload
            )
        )
        fake_module = FakeHidModule()
        fake_module.device_factory = lambda: FakeHidDevice(responder=queue)

        with mock.patch.object(hid_module, "hid", fake_module):
            backend = open_device(vid=OPENKEY_VID, pid=OPENKEY_PID)

        self.assertEqual(backend.cid, 0xCAFEBABE)
        self.assertTrue(fake_module.opened[0].opened)

    def test_open_device_requires_hidapi(self):
        with mock.patch.object(hid_module, "hid", None):
            with self.assertRaises(TransportError):
                open_device()

    def test_send_cmd_before_init_raises(self):
        backend = HidTransportBackend()
        backend._device = FakeHidDevice()
        with self.assertRaises(TransportError):
            backend.send_cmd(0xCAFEBABE, CMD_CBOR, b"\x00")

    def test_send_cmd_without_open_raises(self):
        backend = HidTransportBackend()
        with self.assertRaises(TransportError):
            backend.send_cmd(CTAPHID_BROADCAST_CID, CMD_INIT, self.nonce)


class TestHidBackendSendRecv(unittest.TestCase):

    def _make_backend(self, reports):
        fake = FakeHidDevice(responder=ReportQueue(reports))
        backend = HidTransportBackend()
        backend._device = fake
        backend._cid = 0x12345678
        return backend, fake

    def test_cbor_response_single_packet(self):
        response = b"\x00\xa1\x01\x02"
        packets = CtapHidMessageAssembler.fragment_message(
            0x12345678, CMD_CBOR, response
        )
        backend, fake = self._make_backend(packets)
        result = backend.send_cmd(0x12345678, CMD_CBOR, b"\x04")
        self.assertEqual(result, response)

    def test_cbor_response_multipacket(self):
        response = b"x" * 100
        packets = CtapHidMessageAssembler.fragment_message(
            0x12345678, CMD_CBOR, response
        )
        backend, fake = self._make_backend(packets)
        result = backend.send_cmd(0x12345678, CMD_CBOR, b"\x04")
        self.assertEqual(result, response)

    def test_keepalive_packets_are_skipped(self):
        response = b"hello"
        keepalive = CtapHidMessageAssembler.fragment_message(
            0x12345678, CMD_KEEPALIVE, b"\x01"
        )
        real = CtapHidMessageAssembler.fragment_message(
            0x12345678, CMD_CBOR, response
        )
        backend, fake = self._make_backend(keepalive + real)
        result = backend.send_cmd(0x12345678, CMD_CBOR, b"\x04")
        self.assertEqual(result, response)

    def test_error_packet_raises(self):
        error = CtapHidMessageAssembler.fragment_message(
            0x12345678, CMD_ERROR, b"\x07"
        )
        backend, fake = self._make_backend(error)
        with self.assertRaises(TransportError):
            backend.send_cmd(0x12345678, CMD_CBOR, b"\x04")


class TestHidBackendWrite(unittest.TestCase):

    def test_write_report_writes_64_bytes_with_report_id(self):
        fake = FakeHidDevice()
        backend = HidTransportBackend()
        backend._device = fake
        packet = bytes(range(CTAPHID_PACKET_SIZE))
        backend._write_report(packet)
        self.assertEqual(len(fake.writes), 1)
        self.assertEqual(len(fake.writes[0]), CTAPHID_PACKET_SIZE + 1)
        self.assertEqual(fake.writes[0][0], 0x00)

    def test_write_report_rejects_bad_size(self):
        fake = FakeHidDevice()
        backend = HidTransportBackend()
        backend._device = fake
        with self.assertRaises(TransportError):
            backend._write_report(b"short")


class TestOpenKeyDeviceFromHid(unittest.TestCase):

    def test_from_hid_connects(self):
        nonce = b"\x01\x02\x03\x04\x05\x06\x07\x08"
        response = init_response(nonce, cid=0xCAFEBABE)
        init_packets = CtapHidMessageAssembler.fragment_message(
            CTAPHID_BROADCAST_CID, CMD_INIT, response
        )

        # cbor getInfo response (status 0x00 + CBOR)
        import cbor2
        info_cbor = cbor2.dumps({
            1: ["FIDO_2_0", "FIDO_2_1"],
            4: {"rk": True},
        })
        info_packets = CtapHidMessageAssembler.fragment_message(
            0xCAFEBABE, CMD_CBOR, b"\x00" + info_cbor
        )

        fake_module = FakeHidModule()
        fake_module.device_factory = lambda: FakeHidDevice(
            responder=ReportQueue(init_packets + info_packets)
        )

        with mock.patch.object(hid_module, "hid", fake_module):
            dev = OpenKeyDevice.from_hid()

        info = dev.get_info()
        self.assertIn("FIDO_2_0", info.versions)
        self.assertTrue(info.options.get("rk"))


if __name__ == "__main__":
    unittest.main()
