import unittest
from openkey.transport import CtapHidPacket, CtapHidMessageAssembler, CMD_CBOR
from openkey.client import OpenKeyDevice
from openkey.exceptions import CtapError

class TestOpenKeySDK(unittest.TestCase):

    def test_ctap_hid_packet_serialization(self):
        payload = b"hello ctap hid payload test"
        pkt = CtapHidPacket.create_init_packet(0x12345678, CMD_CBOR, payload)
        self.assertEqual(len(pkt), 64)

        cid, cmd, parsed_payload, total_len = CtapHidPacket.parse_packet(pkt)
        self.assertEqual(cid, 0x12345678)
        self.assertEqual(cmd, CMD_CBOR)
        self.assertEqual(total_len, len(payload))
        self.assertEqual(parsed_payload[:len(payload)], payload)

    def test_ctap_hid_message_assembly(self):
        # 100 bytes payload (spans 1 Init + 1 Cont)
        full_payload = bytes(range(100))
        cid = 0x87654321
        packets = CtapHidMessageAssembler.fragment_message(cid, CMD_CBOR, full_payload)
        self.assertEqual(len(packets), 2)

        res_cid, res_cmd, res_payload = CtapHidMessageAssembler.assemble_message(packets)
        self.assertEqual(res_cid, cid)
        self.assertEqual(res_cmd, CMD_CBOR)
        self.assertEqual(res_payload, full_payload)

    def test_openkey_device_connect_and_get_info(self):
        dev = OpenKeyDevice()
        info = dev.get_info()
        self.assertIn("FIDO_2_0", info.versions)
        self.assertIn("FIDO_2_1", info.versions)
        self.assertTrue(info.options.get("rk"))
        self.assertEqual(info.max_msg_size, 1200)

if __name__ == "__main__":
    unittest.main()
