import unittest
import cbor2

from openkey.ctap2 import CMD_CLIENT_PIN, Ctap2Client
from openkey.exceptions import CtapError
from openkey.pin import (
    PIN_PROTOCOL_V1,
    PIN_PROTOCOL_V2,
    CoseEc2Key,
    PinUvAuthProtocol,
    pad_pin,
    pin_hash,
)
from openkey.pin_client import PinClient, change_device_pin, setup_pin


def _cose_key_from_bytes(x: bytes, y: bytes) -> dict:
    return {1: 2, 3: -25, -1: 1, -2: x, -3: y}


class FakeAuthenticatorPin:
    """Simula o lado autenticador do authenticatorClientPIN.

    Implementa o mesmo ECDH/AES/HMAC do protocolo para validar o cliente de
    forma independente (sem compartilhar código com o PinUvAuthProtocol).
    """

    def __init__(self, protocol_version: int = PIN_PROTOCOL_V1):
        self.version = protocol_version
        self._private_key = PinUvAuthProtocol._new_test_keypair() if False else None
        self._stored_pin_hash = None
        self._set_private_key()

    def _set_private_key(self):
        from cryptography.hazmat.primitives.asymmetric import ec
        self._private_key = ec.generate_private_key(ec.SECP256R1())
        pub = self._private_key.public_key().public_numbers()
        self._public_key = CoseEc2Key(
            pub.x.to_bytes(32, "big"), pub.y.to_bytes(32, "big")
        )

    # ------------------------------------------------------------------
    # Transporte simulado
    # ------------------------------------------------------------------

    def make_ctap2_client(self):
        def send(cmd: int, payload: bytes) -> bytes:
            if cmd == CMD_CLIENT_PIN:
                params = cbor2.loads(payload)
                return b"\x00" + cbor2.dumps(self._handle(params))
            raise AssertionError(f"comando inesperado: 0x{cmd:02x}")

        return Ctap2Client(send)

    def _handle(self, params: dict) -> dict:
        sub = params[2]
        if sub == 0x01:  # getPinRetries
            return {3: 8}
        if sub == 0x02:  # getKeyAgreement
            return {1: self._public_key.to_cose()}
        if sub == 0x03:  # setPIN
            return self._handle_set(params)
        if sub == 0x04:  # changePIN
            return self._handle_change(params)
        if sub == 0x05:  # getPINToken
            return self._handle_token(params)
        raise AssertionError(f"subcomando inesperado: {sub}")

    # ------------------------------------------------------------------
    # Criptografia do autenticador
    # ------------------------------------------------------------------

    def _shared_secret(self, peer_cose: dict) -> bytes:
        from cryptography.hazmat.primitives.asymmetric import ec
        peer = CoseEc2Key.from_cose(peer_cose).to_ec_public_key()
        ecdh_x = self._private_key.exchange(ec.ECDH(), peer)
        if self.version == PIN_PROTOCOL_V1:
            return ecdh_x
        salt = (
            b"\x00"
            + peer_cose[-2]
            + peer_cose[-3]
            + self._public_key.x
            + self._public_key.y
        )
        from cryptography.hazmat.primitives import hashes
        from cryptography.hazmat.primitives.kdf.hkdf import HKDF
        return HKDF(
            algorithm=hashes.SHA256(),
            length=32,
            salt=salt,
            info=b"CTAP2 shared secret\x00",
        ).derive(ecdh_x)

    def _aes_key(self, shared: bytes) -> bytes:
        if self.version == PIN_PROTOCOL_V1:
            return shared
        from cryptography.hazmat.primitives import hashes
        from cryptography.hazmat.primitives.kdf.hkdf import HKDF
        salt = (
            b"\x00"
            + self._last_peer_x
            + self._last_peer_y
            + self._public_key.x
            + self._public_key.y
        )
        return HKDF(
            algorithm=hashes.SHA256(),
            length=32,
            salt=salt,
            info=b"CTAP2 AES key\x00",
        ).derive(shared)

    def _hmac_key(self, shared: bytes) -> bytes:
        if self.version == PIN_PROTOCOL_V1:
            return shared
        from cryptography.hazmat.primitives import hashes
        from cryptography.hazmat.primitives.kdf.hkdf import HKDF
        salt = (
            b"\x00"
            + self._last_peer_x
            + self._last_peer_y
            + self._public_key.x
            + self._public_key.y
        )
        return HKDF(
            algorithm=hashes.SHA256(),
            length=32,
            salt=salt,
            info=b"CTAP2 HMAC key\x00",
        ).derive(shared)

    def _aes_decrypt(self, key: bytes, data: bytes) -> bytes:
        from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
        cipher = Cipher(algorithms.AES(key), modes.CBC(b"\x00" * 16))
        return cipher.decryptor().update(data) + cipher.decryptor().finalize()

    def _hmac(self, key: bytes, data: bytes) -> bytes:
        import hashlib
        import hmac
        return hmac.new(key, data, hashlib.sha256).digest()

    def _record_peer(self, params: dict):
        key = params[3]
        self._last_peer_x = key[-2]
        self._last_peer_y = key[-3]

    def _handle_set(self, params: dict):
        self._record_peer(params)
        shared = self._shared_secret(params[3])
        new_pin_enc = params[5]
        auth = params[7]
        expected = self._hmac(
            self._hmac_key(shared), bytes([0x03]) + new_pin_enc
        )
        if not self._hmac_equals(auth, expected):
            raise CtapError(0x30)  # CTAP2_ERR_PIN_AUTH_INVALID
        new_pin = self._aes_decrypt(self._aes_key(shared), new_pin_enc)[:63]
        self._stored_pin_hash = pin_hash(new_pin.rstrip(b"\x00").decode("utf-8"))
        return {}

    def _handle_change(self, params: dict):
        self._record_peer(params)
        shared = self._shared_secret(params[3])
        pin_hash_enc = params[4]
        new_pin_enc = params[5]
        auth = params[7]
        expected = self._hmac(
            self._hmac_key(shared),
            bytes([0x04]) + pin_hash_enc + new_pin_enc,
        )
        if not self._hmac_equals(auth, expected):
            raise CtapError(0x30)
        if self._stored_pin_hash is None:
            raise CtapError(0x32)  # CTAP2_ERR_PIN_NOT_SET
        old_hash = self._aes_decrypt(self._aes_key(shared), pin_hash_enc)[:16]
        if old_hash != self._stored_pin_hash:
            raise CtapError(0x31)  # CTAP2_ERR_PIN_INVALID
        new_pin = self._aes_decrypt(self._aes_key(shared), new_pin_enc)[:63]
        self._stored_pin_hash = pin_hash(new_pin.rstrip(b"\x00").decode("utf-8"))
        return {}

    def _handle_token(self, params: dict):
        self._record_peer(params)
        shared = self._shared_secret(params[3])
        pin_hash_enc = params[4]
        if self._stored_pin_hash is None:
            raise CtapError(0x32)
        dec = self._aes_decrypt(self._aes_key(shared), pin_hash_enc)[:16]
        if dec != self._stored_pin_hash:
            raise CtapError(0x31)
        token = self._hmac(self._hmac_key(shared), b"\x00" * 32)
        return {2: token}

    @staticmethod
    def _hmac_equals(a: bytes, b: bytes) -> bool:
        # comparação em tempo constante
        return hmac_compare(a, b)


def hmac_compare(a: bytes, b: bytes) -> bool:
    import hmac
    return hmac.compare_digest(a, b)


class TestPinHelpers(unittest.TestCase):

    def test_pin_hash_length_and_value(self):
        h = pin_hash("1234")
        self.assertEqual(len(h), 16)
        import hashlib
        expected = hashlib.sha256(b"1234").digest()[:16]
        self.assertEqual(h, expected)

    def test_pad_pin(self):
        padded = pad_pin("1234")
        self.assertEqual(len(padded), 64)
        self.assertEqual(padded[:4], b"1234")
        self.assertTrue(padded[4:] == b"\x00" * 60)

    def test_pad_pin_rejects_empty(self):
        from openkey.pin import PinError
        with self.assertRaises(PinError):
            pad_pin("")

    def test_pad_pin_rejects_too_long(self):
        from openkey.pin import PinError
        with self.assertRaises(PinError):
            pad_pin("x" * 64)


class TestPinUvAuthProtocol(unittest.TestCase):

    def test_cose_key_roundtrip(self):
        proto = PinUvAuthProtocol(PIN_PROTOCOL_V1)
        cose = proto.platform_public_key.to_cose()
        parsed = CoseEc2Key.from_cose(cose)
        self.assertEqual(parsed.x, proto.platform_public_key.x)
        self.assertEqual(parsed.y, proto.platform_public_key.y)

    def test_invalid_protocol_version(self):
        from openkey.pin import PinError
        with self.assertRaises(PinError):
            PinUvAuthProtocol(99)

    def test_shared_secret_derivation_v1(self):
        proto = PinUvAuthProtocol(PIN_PROTOCOL_V1)
        auth = FakeAuthenticatorPin(PIN_PROTOCOL_V1)
        proto.set_peer_public_key(auth._public_key.to_cose())
        expected = auth._shared_secret(proto.platform_public_key.to_cose())
        self.assertEqual(proto._shared_secret, expected)
        self.assertEqual(len(proto._shared_secret), 32)

    def test_shared_secret_derivation_v2(self):
        proto = PinUvAuthProtocol(PIN_PROTOCOL_V2)
        auth = FakeAuthenticatorPin(PIN_PROTOCOL_V2)
        proto.set_peer_public_key(auth._public_key.to_cose())
        expected = auth._shared_secret(proto.platform_public_key.to_cose())
        self.assertEqual(proto._shared_secret, expected)
        self.assertEqual(len(proto._shared_secret), 32)


class TestPinClientProtocol(unittest.TestCase):

    def _run(self, version: int):
        auth = FakeAuthenticatorPin(version)
        ctap2 = auth.make_ctap2_client()
        client = PinClient(ctap2, protocol_version=version)

        # getPinRetries
        self.assertEqual(client.get_pin_retries(), 8)

        # getKeyAgreement
        peer = client.get_key_agreement()
        self.assertEqual(peer.x, auth._public_key.x)
        self.assertEqual(peer.y, auth._public_key.y)

        # setPIN
        client.set_pin("1234")
        self.assertIsNotNone(auth._stored_pin_hash)
        self.assertEqual(auth._stored_pin_hash, pin_hash("1234"))

        # changePIN
        client.change_pin("1234", "5678")
        self.assertEqual(auth._stored_pin_hash, pin_hash("5678"))

        # getPINToken
        token = client.get_pin_token("5678")
        self.assertEqual(len(token), 32)

    def test_full_flow_v1(self):
        self._run(PIN_PROTOCOL_V1)

    def test_full_flow_v2(self):
        self._run(PIN_PROTOCOL_V2)


class TestPinClientErrors(unittest.TestCase):

    def test_set_pin_wrong_auth_raises_ctap_error(self):
        auth = FakeAuthenticatorPin(PIN_PROTOCOL_V1)
        ctap2 = auth.make_ctap2_client()
        client = PinClient(ctap2, protocol_version=PIN_PROTOCOL_V1)
        client.get_key_agreement()
        # força HMAC inválido corrompendo o request
        request = client._protocol.set_pin_request("1234")
        request[7] = b"\x00" * 32
        with self.assertRaises(CtapError):
            client._client_pin(request)

    def test_change_pin_wrong_old_pin_raises(self):
        auth = FakeAuthenticatorPin(PIN_PROTOCOL_V1)
        ctap2 = auth.make_ctap2_client()
        client = PinClient(ctap2, protocol_version=PIN_PROTOCOL_V1)
        client.get_key_agreement()
        client.set_pin("1234")
        with self.assertRaises(CtapError):
            client.change_pin("9999", "5678")

    def test_setup_pin_convenience(self):
        auth = FakeAuthenticatorPin(PIN_PROTOCOL_V1)
        ctap2 = auth.make_ctap2_client()
        setup_pin(ctap2, "1234", protocol_version=PIN_PROTOCOL_V1)
        self.assertEqual(auth._stored_pin_hash, pin_hash("1234"))

    def test_change_device_pin_convenience(self):
        auth = FakeAuthenticatorPin(PIN_PROTOCOL_V1)
        ctap2 = auth.make_ctap2_client()
        setup_pin(ctap2, "1234", protocol_version=PIN_PROTOCOL_V1)
        change_device_pin(ctap2, "1234", "abcd", protocol_version=PIN_PROTOCOL_V1)
        self.assertEqual(auth._stored_pin_hash, pin_hash("abcd"))


if __name__ == "__main__":
    unittest.main()
