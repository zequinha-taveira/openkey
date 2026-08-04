"""Protocolo authenticatorClientPIN (FIDO CTAP2.1 Section 6.5)

Implementa pinUvAuthProtocol v1 e v2: troca de chaves ECDH (P-256), derivação
do shared secret, criptografia AES-256-CBC e HMAC-SHA-256 para os subcomandos
getPinRetries, getKeyAgreement, setPIN, changePIN e getPINToken.

Referência: FIDO CTAP2.1 spec, seção 6.5.5 (Client PIN) e 6.5.6
(pinUvAuthToken).

Nota de segurança: o material de chave derivado (shared secret, token) é
ephemeral e não deve ser logado ou persistido.
"""

import hashlib
import hmac
import os
from typing import Dict, Any, Optional

from cryptography.hazmat.primitives.asymmetric import ec
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from cryptography.hazmat.primitives.kdf.hkdf import HKDF

from openkey.exceptions import OpenKeyError

# Subcomandos do authenticatorClientPIN (CTAP2.1 §6.5.2)
PIN_SUB_GET_RETRIES = 0x01
PIN_SUB_GET_KEY_AGREEMENT = 0x02
PIN_SUB_SET_PIN = 0x03
PIN_SUB_CHANGE_PIN = 0x04
PIN_SUB_GET_PIN_TOKEN = 0x05
PIN_SUB_GET_UV_TOKEN = 0x06
PIN_SUB_GET_UV_RETRIES = 0x07

# Versões do pinUvAuthProtocol
PIN_PROTOCOL_V1 = 1
PIN_PROTOCOL_V2 = 2

# Chaves CBOR do comando authenticatorClientPIN (CTAP2.1 §6.5.2)
_CBOR_PROTOCOL = 1
_CBOR_SUBCOMMAND = 2
_CBOR_KEY_AGREEMENT = 3
_CBOR_PIN_HASH_ENC = 4
_CBOR_NEW_PIN_ENC = 5
_CBOR_PERMISSIONS = 6
_CBOR_PIN_UV_AUTH_PARAM = 7
_CBOR_PIN_UV_AUTH_TOKEN = 2  # (resposta)

# Chaves CBOR da resposta
_RESP_KEY_AGREEMENT = 0x01
_RESP_PIN_UV_AUTH_TOKEN = 0x02
_RESP_PIN_RETRIES = 0x03
_RESP_UV_RETRIES = 0x05

# COSE EC2 P-256 (RFC 9052 / CTAP2 §6.5.2)
_COSE_KTY_EC2 = 2
_COSE_ALG_ECDH_ES_HKDF_256 = -25
_COSE_CRV_P256 = 1
_COSE_X = -2
_COSE_Y = -3

_AES_KEY_SIZE = 32
_IV_SIZE = 16
_BLOCK_SIZE = 16
_TOKEN_MSG_LABEL = b"CTAP2 PIN token\x00"


class PinError(OpenKeyError):
    """Erro no protocolo authenticatorClientPIN."""


def _sha256(data: bytes) -> bytes:
    return hashlib.sha256(data).digest()


def _aes_cbc_encrypt(key: bytes, iv: bytes, data: bytes) -> bytes:
    if len(key) != _AES_KEY_SIZE:
        raise PinError("Chave AES deve ter 32 bytes")
    if len(iv) != _IV_SIZE:
        raise PinError("IV deve ter 16 bytes")
    if len(data) % _BLOCK_SIZE != 0:
        raise PinError("Dados devem ser múltiplos de 16 bytes")
    cipher = Cipher(algorithms.AES(key), modes.CBC(iv))
    encryptor = cipher.encryptor()
    return encryptor.update(data) + encryptor.finalize()


def _aes_cbc_decrypt(key: bytes, iv: bytes, data: bytes) -> bytes:
    if len(key) != _AES_KEY_SIZE:
        raise PinError("Chave AES deve ter 32 bytes")
    if len(iv) != _IV_SIZE:
        raise PinError("IV deve ter 16 bytes")
    if len(data) % _BLOCK_SIZE != 0:
        raise PinError("Dados devem ser múltiplos de 16 bytes")
    cipher = Cipher(algorithms.AES(key), modes.CBC(iv))
    decryptor = cipher.decryptor()
    return decryptor.update(data) + decryptor.finalize()


def _hmac_sha256(key: bytes, data: bytes) -> bytes:
    return hmac.new(key, data, hashlib.sha256).digest()


def _zero_iv() -> bytes:
    return b"\x00" * _IV_SIZE


def pin_hash(pin: str) -> bytes:
    """Retorna o pinHash (SHA-256 do PIN, truncado em 16 bytes)."""
    return _sha256(pin.encode("utf-8"))[:16]


def pad_pin(pin: str) -> bytes:
    """Preenche o PIN com zeros até 64 bytes, conforme a spec."""
    raw = pin.encode("utf-8")
    if not raw:
        raise PinError("PIN não pode ser vazio")
    if len(raw) > 63:
        raise PinError("PIN muito longo (máximo 63 bytes UTF-8)")
    return raw.ljust(64, b"\x00")


class CoseEc2Key:
    """Chave pública EC2 P-256 no formato COSE (RFC 9052)."""

    def __init__(self, x: bytes, y: bytes):
        if len(x) != 32 or len(y) != 32:
            raise PinError("Coordenadas EC2 P-256 devem ter 32 bytes")
        self.x = x
        self.y = y

    @classmethod
    def from_private_key(cls, private_key: ec.EllipticCurvePrivateKey) -> "CoseEc2Key":
        numbers = private_key.public_key().public_numbers()
        return cls(
            numbers.x.to_bytes(32, "big"),
            numbers.y.to_bytes(32, "big"),
        )

    def to_cose(self) -> Dict[int, Any]:
        return {
            1: _COSE_KTY_EC2,          # kty: EC2
            3: _COSE_ALG_ECDH_ES_HKDF_256,  # alg: ECDH-ES-HKDF-256
            -1: _COSE_CRV_P256,        # crv: P-256
            _COSE_X: self.x,
            _COSE_Y: self.y,
        }

    @classmethod
    def from_cose(cls, cose: Dict[int, Any]) -> "CoseEc2Key":
        kty = cose.get(1)
        crv = cose.get(-1)
        if kty != _COSE_KTY_EC2 or crv != _COSE_CRV_P256:
            raise PinError("COSE_Key não é uma EC2 P-256 válida")
        x = cose.get(_COSE_X)
        y = cose.get(_COSE_Y)
        if not isinstance(x, bytes) or not isinstance(y, bytes):
            raise PinError("COSE_Key sem coordenadas x/y")
        return cls(x, y)

    def to_ec_public_key(self) -> ec.EllipticCurvePublicKey:
        return ec.EllipticCurvePublicNumbers(
            int.from_bytes(self.x, "big"),
            int.from_bytes(self.y, "big"),
            ec.SECP256R1(),
        ).public_key()


class PinUvAuthProtocol:
    """Implementa o lado cliente do pinUvAuthProtocol (v1 ou v2).

    Fluxo:
    1. ``get_key_agreement`` envia a chave pública da plataforma (COSE).
    2. ``set_peer_public_key`` recebe a chave pública do autenticador.
    3. Métodos ``set_pin``/``change_pin``/``get_pin_token`` produzem os
       parâmetros CBOR do subcomando, já cifrados/autenticados.
    """

    def __init__(self, version: int = PIN_PROTOCOL_V1):
        if version not in (PIN_PROTOCOL_V1, PIN_PROTOCOL_V2):
            raise PinError(f"Versão de protocolo inválida: {version}")
        self.version = version
        self._private_key = ec.generate_private_key(ec.SECP256R1())
        self._platform_public = CoseEc2Key.from_private_key(self._private_key)
        self._peer_public: Optional[CoseEc2Key] = None
        self._shared_secret: Optional[bytes] = None

    @property
    def platform_public_key(self) -> CoseEc2Key:
        return self._platform_public

    @property
    def peer_public_key(self) -> Optional[CoseEc2Key]:
        return self._peer_public

    def set_peer_public_key(self, cose_key: Dict[int, Any]) -> None:
        """Registra a chave pública do autenticador e deriva o shared secret."""
        self._peer_public = CoseEc2Key.from_cose(cose_key)
        self._shared_secret = self._derive_shared_secret()

    def _derive_shared_secret(self) -> bytes:
        if self._peer_public is None:
            raise PinError("Chave pública do autenticador não definida")
        ecdh_x = self._private_key.exchange(
            ec.ECDH(), self._peer_public.to_ec_public_key()
        )

        if self.version == PIN_PROTOCOL_V1:
            # pinUvAuthProtocol 1: shared secret = X do ponto ECDH (32 bytes)
            return ecdh_x

        # pinUvAuthProtocol 2: HKDF-SHA-256 com salt derivado das chaves
        salt = (
            b"\x00"
            + self._platform_public.x
            + self._platform_public.y
            + self._peer_public.x
            + self._peer_public.y
        )
        return HKDF(
            algorithm=hashes.SHA256(),
            length=_AES_KEY_SIZE,
            salt=salt,
            info=b"CTAP2 shared secret\x00",
        ).derive(ecdh_x)

    def _enc_key(self) -> bytes:
        """Chave de encriptação (v1: shared secret; v2: HKDF 'AES key')."""
        if self._shared_secret is None:
            raise PinError("Shared secret não derivado")
        if self.version == PIN_PROTOCOL_V1:
            return self._shared_secret
        salt = self._v2_salt()
        return HKDF(
            algorithm=hashes.SHA256(),
            length=_AES_KEY_SIZE,
            salt=salt,
            info=b"CTAP2 AES key\x00",
        ).derive(self._shared_secret)

    def _hmac_key(self) -> bytes:
        """Chave HMAC (v1: shared secret; v2: HKDF 'HMAC key')."""
        if self._shared_secret is None:
            raise PinError("Shared secret não derivado")
        if self.version == PIN_PROTOCOL_V1:
            return self._shared_secret
        salt = self._v2_salt()
        return HKDF(
            algorithm=hashes.SHA256(),
            length=_AES_KEY_SIZE,
            salt=salt,
            info=b"CTAP2 HMAC key\x00",
        ).derive(self._shared_secret)

    def _v2_salt(self) -> bytes:
        if self._peer_public is None:
            raise PinError("Chave pública do autenticador não definida")
        return (
            b"\x00"
            + self._platform_public.x
            + self._platform_public.y
            + self._peer_public.x
            + self._peer_public.y
        )

    # ------------------------------------------------------------------
    # Subcomandos
    # ------------------------------------------------------------------

    def get_key_agreement_request(self) -> Dict[int, Any]:
        """Parâmetros do subcomando getKeyAgreement (0x02)."""
        return {
            _CBOR_PROTOCOL: self.version,
            _CBOR_SUBCOMMAND: PIN_SUB_GET_KEY_AGREEMENT,
            _CBOR_KEY_AGREEMENT: self._platform_public.to_cose(),
        }

    def get_pin_retries_request(self) -> Dict[int, Any]:
        """Parâmetros do subcomando getPinRetries (0x01)."""
        return {
            _CBOR_PROTOCOL: self.version,
            _CBOR_SUBCOMMAND: PIN_SUB_GET_RETRIES,
        }

    def set_pin_request(self, new_pin: str) -> Dict[int, Any]:
        """Parâmetros do subcomando setPIN (0x03)."""
        new_pin_enc = _aes_cbc_encrypt(
            self._enc_key(), _zero_iv(), pad_pin(new_pin)
        )
        pin_uv_auth_param = _hmac_sha256(
            self._hmac_key(), bytes([PIN_SUB_SET_PIN]) + new_pin_enc
        )
        return {
            _CBOR_PROTOCOL: self.version,
            _CBOR_SUBCOMMAND: PIN_SUB_SET_PIN,
            _CBOR_KEY_AGREEMENT: self._platform_public.to_cose(),
            _CBOR_NEW_PIN_ENC: new_pin_enc,
            _CBOR_PIN_UV_AUTH_PARAM: pin_uv_auth_param,
        }

    def change_pin_request(self, current_pin: str, new_pin: str) -> Dict[int, Any]:
        """Parâmetros do subcomando changePIN (0x04)."""
        pin_hash_enc = _aes_cbc_encrypt(
            self._enc_key(), _zero_iv(), pin_hash(current_pin)
        )
        new_pin_enc = _aes_cbc_encrypt(
            self._enc_key(), _zero_iv(), pad_pin(new_pin)
        )
        pin_uv_auth_param = _hmac_sha256(
            self._hmac_key(),
            bytes([PIN_SUB_CHANGE_PIN]) + pin_hash_enc + new_pin_enc,
        )
        return {
            _CBOR_PROTOCOL: self.version,
            _CBOR_SUBCOMMAND: PIN_SUB_CHANGE_PIN,
            _CBOR_KEY_AGREEMENT: self._platform_public.to_cose(),
            _CBOR_PIN_HASH_ENC: pin_hash_enc,
            _CBOR_NEW_PIN_ENC: new_pin_enc,
            _CBOR_PIN_UV_AUTH_PARAM: pin_uv_auth_param,
        }

    def get_pin_token_request(self, pin: str) -> Dict[int, Any]:
        """Parâmetros do subcomando getPINToken (0x05)."""
        pin_hash_enc = _aes_cbc_encrypt(
            self._enc_key(), _zero_iv(), pin_hash(pin)
        )
        return {
            _CBOR_PROTOCOL: self.version,
            _CBOR_SUBCOMMAND: PIN_SUB_GET_PIN_TOKEN,
            _CBOR_KEY_AGREEMENT: self._platform_public.to_cose(),
            _CBOR_PIN_HASH_ENC: pin_hash_enc,
        }

    # ------------------------------------------------------------------
    # Pós-processamento da resposta
    # ------------------------------------------------------------------

    def process_key_agreement_response(self, response: Dict[int, Any]) -> None:
        """Processa a resposta do getKeyAgreement (configura a chave do peer)."""
        peer = response.get(_RESP_KEY_AGREEMENT)
        if not isinstance(peer, dict):
            raise PinError("Resposta getKeyAgreement sem COSE_Key")
        self.set_peer_public_key(peer)

    def derive_pin_uv_auth_token(self, token_cbor: bytes) -> bytes:
        """Deriva o pinUvAuthToken a partir da resposta getPINToken.

        O autenticador retorna o token cifrado em CBOR byte string (chave 0x02);
        para pinUvAuthProtocol v1/v2 o valor já é o token de 32 bytes.
        """
        # Em v1/v2 o token na resposta é diretamente o valor (32 bytes).
        if len(token_cbor) != 32:
            raise PinError("pinUvAuthToken deve ter 32 bytes")
        return token_cbor


def parse_pin_retries(response: Dict[int, Any]) -> int:
    """Extrai pinRetries da resposta getPinRetries."""
    retries = response.get(_RESP_PIN_RETRIES)
    if not isinstance(retries, int):
        raise PinError("Resposta getPinRetries sem campo pinRetries")
    return retries
