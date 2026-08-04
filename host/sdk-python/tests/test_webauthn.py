import unittest
import hashlib

import cbor2

from openkey.ctap2 import (
    CMD_GET_ASSERTION,
    CMD_MAKE_CREDENTIAL,
    Ctap2Client,
    CtapLogRecorder,
)
from openkey.exceptions import CtapError
from openkey.webauthn import (
    AuthenticatorData,
    PublicKeyCredentialDescriptor,
    RpEntity,
    UserEntity,
    WebAuthnError,
)


def _client_data_hash(msg: str = "challenge") -> bytes:
    return hashlib.sha256(msg.encode("utf-8")).digest()


class FakeAuthenticatorWebAuthn:
    """Simula o lado autenticador de makeCredential/getAssertion."""

    def __init__(self):
        self.credentials = []
        self._counter = 0

    def make_ctap2_client(self):
        def send(cmd: int, payload: bytes) -> bytes:
            params = cbor2.loads(payload)
            if cmd == CMD_MAKE_CREDENTIAL:
                response = self._make_credential(params)
            elif cmd == CMD_GET_ASSERTION:
                response = self._get_assertion(params)
            else:
                raise AssertionError(f"comando inesperado: 0x{cmd:02x}")
            return b"\x00" + cbor2.dumps(response)

        return Ctap2Client(send)

    def _auth_data(self, rp_id: str, attested: bool = False) -> bytes:
        flags = 0x01 | (0x40 if attested else 0)  # UP [+ AT]
        rp_hash = hashlib.sha256(rp_id.encode("utf-8")).digest()
        data = rp_hash + bytes([flags]) + (1).to_bytes(4, "big")
        if attested:
            data += b"\xaa" * 16  # aaguid
            data += (16).to_bytes(2, "big")
            data += self._last_cred_id()
        return data

    def _last_cred_id(self) -> bytes:
        return self.credentials[-1]["id"]

    def _make_credential(self, params: dict) -> dict:
        rp = params[2]
        user = params[3]
        algs = params[4]
        if not any(p.get(3) == -7 for p in algs):
            raise CtapError(0x26)  # CTAP2_ERR_UNSUPPORTED_ALGORITHM
        options = params.get(7, {})
        rk = bool(options.get("rk", False))

        self._counter += 1
        cred_id = self._counter.to_bytes(16, "big")
        self.credentials.append(
            {
                "rp_id": rp[1],
                "user": user,
                "id": cred_id,
                "resident": rk,
            }
        )
        return {
            1: "none",
            2: self._auth_data(rp[1], attested=True),
            3: {},
        }

    def _get_assertion(self, params: dict) -> dict:
        rp_id = params[1]
        client_data_hash = params[2]
        allow_list = params.get(3)
        options = params.get(5, {})

        creds = [c for c in self.credentials if c["rp_id"] == rp_id]
        if allow_list is not None:
            allowed = {d[1] for d in allow_list}
            creds = [c for c in creds if c["id"] in allowed]
        if not creds:
            raise CtapError(0x1A)  # CTAP2_ERR_NO_CREDENTIALS

        cred = creds[0]
        auth_data = self._auth_data(rp_id, attested=False)
        signature = hashlib.sha256(auth_data + client_data_hash).digest()
        response = {
            1: {1: cred["id"], 2: "public-key"},
            2: auth_data,
            3: signature,
            4: cred["user"],
        }
        if len(creds) > 1:
            response[5] = len(creds)
        return response


class TestMakeCredential(unittest.TestCase):

    def test_make_credential_basic(self):
        fake = FakeAuthenticatorWebAuthn()
        ctap2 = fake.make_ctap2_client()
        rp = RpEntity(id="example.com", name="Example")
        user = UserEntity(id=b"user-1", name="alice", display_name="Alice")

        resp = ctap2.make_credential(
            client_data_hash=_client_data_hash(),
            rp=rp,
            user=user,
            pub_key_cred_params=[-7, -257],
        )

        self.assertEqual(resp.fmt, "none")
        self.assertIsNotNone(resp.auth_data_obj)
        ad = resp.auth_data_obj
        assert ad is not None
        self.assertTrue(ad.attested)
        self.assertTrue(ad.user_present)
        self.assertEqual(ad.sign_count, 1)
        self.assertEqual(ad.aaguid, b"\xaa" * 16)
        self.assertEqual(len(ad.credential_id), 16)
        self.assertEqual(ad.rp_id_hash, hashlib.sha256(b"example.com").digest())

    def test_make_credential_resident_option(self):
        fake = FakeAuthenticatorWebAuthn()
        ctap2 = fake.make_ctap2_client()
        rp = RpEntity(id="example.com")
        user = UserEntity(id=b"u1", name="alice")

        resp = ctap2.make_credential(
            client_data_hash=_client_data_hash(),
            rp=rp,
            user=user,
            pub_key_cred_params=[{3: -7}],
            options={"rk": True},
        )
        self.assertEqual(resp.fmt, "none")
        self.assertTrue(fake.credentials[0]["resident"])

    def test_make_credential_rejects_short_client_data_hash(self):
        fake = FakeAuthenticatorWebAuthn()
        ctap2 = fake.make_ctap2_client()
        with self.assertRaises(ValueError):
            ctap2.make_credential(
                client_data_hash=b"\x00" * 31,
                rp=RpEntity(id="example.com"),
                user=UserEntity(id=b"u1"),
                pub_key_cred_params=[-7],
            )

    def test_make_credential_unsupported_algorithm_raises(self):
        fake = FakeAuthenticatorWebAuthn()
        ctap2 = fake.make_ctap2_client()
        with self.assertRaises(CtapError):
            ctap2.make_credential(
                client_data_hash=_client_data_hash(),
                rp=RpEntity(id="example.com"),
                user=UserEntity(id=b"u1"),
                pub_key_cred_params=[-999],
            )


class TestGetAssertion(unittest.TestCase):

    def test_get_assertion_after_registration(self):
        fake = FakeAuthenticatorWebAuthn()
        ctap2 = fake.make_ctap2_client()
        ctap2.make_credential(
            client_data_hash=_client_data_hash("reg"),
            rp=RpEntity(id="example.com"),
            user=UserEntity(id=b"u1", name="alice"),
            pub_key_cred_params=[-7],
            options={"rk": True},
        )

        assertion = ctap2.get_assertion(
            rp_id="example.com",
            client_data_hash=_client_data_hash("auth"),
        )

        self.assertEqual(len(assertion.signature), 32)
        self.assertIsNotNone(assertion.credential)
        self.assertEqual(assertion.credential.id, fake.credentials[0]["id"])
        self.assertEqual(assertion.credential.type, "public-key")
        self.assertEqual(assertion.user.id, b"u1")
        self.assertIsNotNone(assertion.auth_data_obj)
        assert assertion.auth_data_obj is not None
        self.assertTrue(assertion.auth_data_obj.user_present)
        self.assertFalse(assertion.auth_data_obj.attested)

    def test_get_assertion_with_allow_list(self):
        fake = FakeAuthenticatorWebAuthn()
        ctap2 = fake.make_ctap2_client()
        ctap2.make_credential(
            client_data_hash=_client_data_hash("reg"),
            rp=RpEntity(id="example.com"),
            user=UserEntity(id=b"u1"),
            pub_key_cred_params=[-7],
        )
        ctap2.make_credential(
            client_data_hash=_client_data_hash("reg2"),
            rp=RpEntity(id="example.com"),
            user=UserEntity(id=b"u2"),
            pub_key_cred_params=[-7],
        )

        target = fake.credentials[1]["id"]
        assertion = ctap2.get_assertion(
            rp_id="example.com",
            client_data_hash=_client_data_hash(),
            allow_list=[PublicKeyCredentialDescriptor(id=target)],
        )
        self.assertEqual(assertion.credential.id, target)

    def test_get_assertion_no_credentials_raises(self):
        fake = FakeAuthenticatorWebAuthn()
        ctap2 = fake.make_ctap2_client()
        with self.assertRaises(CtapError):
            ctap2.get_assertion(
                rp_id="example.com",
                client_data_hash=_client_data_hash(),
            )


class TestAuthenticatorData(unittest.TestCase):

    def test_parse_assertion_auth_data(self):
        rp_hash = hashlib.sha256(b"example.com").digest()
        data = rp_hash + bytes([0x01]) + (42).to_bytes(4, "big")
        ad = AuthenticatorData.parse(data)
        self.assertEqual(ad.sign_count, 42)
        self.assertTrue(ad.user_present)
        self.assertFalse(ad.attested)
        self.assertIsNone(ad.credential_id)

    def test_parse_too_short_raises(self):
        with self.assertRaises(WebAuthnError):
            AuthenticatorData.parse(b"\x00" * 10)

    def test_parse_attested_with_short_cred_data_raises(self):
        data = bytearray(47)
        data[32] = 0x40  # AT flag
        with self.assertRaises(WebAuthnError):
            AuthenticatorData.parse(bytes(data))

    def test_public_key_credential_descriptor_roundtrip(self):
        descriptor = PublicKeyCredentialDescriptor(id=b"\x01" * 16)
        cbor_map = descriptor.to_cbor()
        parsed = PublicKeyCredentialDescriptor.from_cbor(cbor_map)
        self.assertEqual(parsed.id, b"\x01" * 16)
        self.assertEqual(parsed.type, "public-key")


class TestLogHook(unittest.TestCase):

    def test_recorder_captures_send_and_recv(self):
        fake = FakeAuthenticatorWebAuthn()
        recorder = CtapLogRecorder()
        ctap2 = fake.make_ctap2_client()
        ctap2._log = recorder.record

        ctap2.make_credential(
            client_data_hash=_client_data_hash(),
            rp=RpEntity(id="example.com"),
            user=UserEntity(id=b"u1"),
            pub_key_cred_params=[-7],
        )

        self.assertEqual(len(recorder.entries), 2)
        send = recorder.entries[0]
        recv = recorder.entries[1]
        self.assertEqual(send.direction, "send")
        self.assertEqual(send.command, CMD_MAKE_CREDENTIAL)
        self.assertEqual(send.command_name, "makeCredential")
        self.assertGreater(len(send.payload), 0)
        self.assertEqual(recv.direction, "recv")
        self.assertEqual(recv.command, CMD_MAKE_CREDENTIAL)
        self.assertEqual(recorder.commands_sent, [CMD_MAKE_CREDENTIAL])

    def test_recorder_records_assertion_commands(self):
        fake = FakeAuthenticatorWebAuthn()
        recorder = CtapLogRecorder()
        ctap2 = fake.make_ctap2_client()
        ctap2._log = recorder.record

        with self.assertRaises(CtapError):
            ctap2.get_assertion(rp_id="x.com", client_data_hash=_client_data_hash())
        # getAssertion falha (sem credenciais) mas o envio é logado
        self.assertEqual(recorder.commands_sent, [CMD_GET_ASSERTION])


if __name__ == "__main__":
    unittest.main()
