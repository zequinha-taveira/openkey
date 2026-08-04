"""Modelos e helpers WebAuthn/CTAP2 (makeCredential e getAssertion).

Define as entidades (RP, usuário, descritores de credencial), o parser do
authenticator data (CTAP2 §6.1) e as respostas tipadas de makeCredential e
getAssertion.

Referência: FIDO CTAP2.1 spec, seção 6.2 (makeCredential) e 6.3
(getAssertion); WebAuthn Level 3 (W3C).
"""

from dataclasses import dataclass, field
from typing import Any, Dict, List, Optional

from openkey.exceptions import OpenKeyError

# Chaves CBOR do request de makeCredential (CTAP2.1 §6.2)
_CBOR_CLIENT_DATA_HASH = 0x01
_CBOR_RP = 0x02
_CBOR_USER = 0x03
_CBOR_PUB_KEY_CRED_PARAMS = 0x04
_CBOR_EXCLUDE_LIST = 0x05
_CBOR_EXTENSIONS = 0x06
_CBOR_OPTIONS = 0x07
_CBOR_PIN_UV_AUTH_PARAM = 0x08
_CBOR_PIN_UV_AUTH_PROTOCOL = 0x09

# Chaves CBOR da resposta de makeCredential
_RESP_FMT = 0x01
_RESP_AUTH_DATA = 0x02
_RESP_ATT_STMT = 0x03

# Chaves CBOR do request de getAssertion (CTAP2.1 §6.3)
_CBOR_RP_ID = 0x01
_CBOR_CLIENT_DATA_HASH_ASSERTION = 0x02
_CBOR_ALLOW_LIST = 0x03
_CBOR_EXTENSIONS_ASSERTION = 0x04
_CBOR_OPTIONS_ASSERTION = 0x05
_CBOR_PIN_UV_AUTH_PARAM_ASSERTION = 0x06
_CBOR_PIN_UV_AUTH_PROTOCOL_ASSERTION = 0x07

# Chaves CBOR da resposta de getAssertion
_RESP_CREDENTIAL = 0x01
_RESP_AUTH_DATA_ASSERTION = 0x02
_RESP_SIGNATURE = 0x03
_RESP_USER = 0x04
_RESP_NUM_CREDENTIALS = 0x05
_RESP_EXTENSIONS_ASSERTION = 0x06

# Chaves do mapa rp
_RP_ID = 0x01
_RP_NAME = 0x02
_RP_ICON = 0x03

# Chaves do mapa user
_USER_ID = 0x01
_USER_NAME = 0x02
_USER_DISPLAY_NAME = 0x03
_USER_ICON = 0x04

# Chaves do PublicKeyCredentialDescriptor
_DESC_ID = 0x01
_DESC_TYPE = 0x02

# Bits do flag byte do authenticator data (CTAP2.1 §6.1)
_FLAG_UP = 0x01
_FLAG_UV = 0x04
_FLAG_AT = 0x40
_FLAG_ED = 0x80

_AUTH_DATA_FIXED_LEN = 37
_AAGUID_LEN = 16
_MIN_AUTH_DATA = _AUTH_DATA_FIXED_LEN + _AAGUID_LEN + 2


class WebAuthnError(OpenKeyError):
    """Erro nos modelos/parse de dados WebAuthn."""


@dataclass
class RpEntity:
    """Relaying Party (mapa rp do makeCredential)."""

    id: str
    name: Optional[str] = None
    icon: Optional[str] = None

    def to_cbor(self) -> Dict[int, Any]:
        result: Dict[int, Any] = {_RP_ID: self.id}
        if self.name is not None:
            result[_RP_NAME] = self.name
        if self.icon is not None:
            result[_RP_ICON] = self.icon
        return result


@dataclass
class UserEntity:
    """Entidade de usuário (mapa user do makeCredential)."""

    id: bytes
    name: Optional[str] = None
    display_name: Optional[str] = None
    icon: Optional[str] = None

    def to_cbor(self) -> Dict[int, Any]:
        result: Dict[int, Any] = {_USER_ID: self.id}
        if self.name is not None:
            result[_USER_NAME] = self.name
        if self.display_name is not None:
            result[_USER_DISPLAY_NAME] = self.display_name
        if self.icon is not None:
            result[_USER_ICON] = self.icon
        return result


@dataclass
class PublicKeyCredentialDescriptor:
    """Descritor de credencial (excludeList/allowList e respostas)."""

    id: bytes
    type: str = "public-key"

    def to_cbor(self) -> Dict[int, Any]:
        return {_DESC_ID: self.id, _DESC_TYPE: self.type}

    @classmethod
    def from_cbor(cls, descriptor: Dict[int, Any]) -> "PublicKeyCredentialDescriptor":
        cred_id = descriptor.get(_DESC_ID)
        if not isinstance(cred_id, bytes):
            raise WebAuthnError("Descriptor sem credentialID")
        cred_type = descriptor.get(_DESC_TYPE)
        return cls(
            id=cred_id,
            type=cred_type if isinstance(cred_type, str) else "public-key",
        )


@dataclass
class AuthenticatorData:
    """Dados do autenticador (authenticatorData) de CTAP2 §6.1.

    Campos fixos: rpIdHash, flags (com atestação/UV/UP), signCount e, se o
    flag AT estiver presente, a credencial atestada (aaguid + credentialId).
    """

    rp_id_hash: bytes
    flags: int
    sign_count: int
    aaguid: Optional[bytes] = None
    credential_id: Optional[bytes] = None

    @property
    def user_present(self) -> bool:
        return bool(self.flags & _FLAG_UP)

    @property
    def user_verified(self) -> bool:
        return bool(self.flags & _FLAG_UV)

    @property
    def attested(self) -> bool:
        return bool(self.flags & _FLAG_AT)

    @property
    def extension_data_present(self) -> bool:
        return bool(self.flags & _FLAG_ED)

    @classmethod
    def parse(cls, data: bytes) -> "AuthenticatorData":
        if len(data) < _AUTH_DATA_FIXED_LEN:
            raise WebAuthnError(
                f"authenticatorData muito curto: {len(data)} bytes"
            )
        rp_id_hash = data[0:32]
        flags = data[32]
        sign_count = int.from_bytes(data[33:37], "big")
        offset = _AUTH_DATA_FIXED_LEN
        aaguid: Optional[bytes] = None
        credential_id: Optional[bytes] = None

        if flags & _FLAG_AT:
            if len(data) < offset + _AAGUID_LEN + 2:
                raise WebAuthnError(
                    "authenticatorData com AT sem dados de credencial atestada"
                )
            aaguid = data[offset:offset + _AAGUID_LEN]
            offset += _AAGUID_LEN
            cred_id_len = int.from_bytes(data[offset:offset + 2], "big")
            offset += 2
            credential_id = data[offset:offset + cred_id_len]

        return cls(
            rp_id_hash=rp_id_hash,
            flags=flags,
            sign_count=sign_count,
            aaguid=aaguid,
            credential_id=credential_id,
        )


@dataclass
class MakeCredentialResponse:
    """Resposta tipada do comando authenticatorMakeCredential."""

    fmt: str
    auth_data: bytes
    att_stmt: Dict[int, Any]
    auth_data_obj: Optional[AuthenticatorData] = None

    @classmethod
    def from_dict(cls, response: Dict[int, Any]) -> "MakeCredentialResponse":
        fmt = response.get(_RESP_FMT)
        auth_data = response.get(_RESP_AUTH_DATA)
        if not isinstance(fmt, str):
            raise WebAuthnError("Resposta makeCredential sem fmt")
        if not isinstance(auth_data, bytes):
            raise WebAuthnError("Resposta makeCredential sem authData")
        att_stmt = response.get(_RESP_ATT_STMT)
        if not isinstance(att_stmt, dict):
            att_stmt = {}
        obj: Optional[AuthenticatorData] = None
        try:
            obj = AuthenticatorData.parse(auth_data)
        except WebAuthnError:
            obj = None
        return cls(fmt=fmt, auth_data=auth_data, att_stmt=att_stmt, auth_data_obj=obj)


@dataclass
class AssertionResponse:
    """Resposta tipada do comando authenticatorGetAssertion."""

    auth_data: bytes
    signature: bytes
    credential: Optional[PublicKeyCredentialDescriptor] = None
    user: Optional[UserEntity] = None
    number_of_credentials: Optional[int] = None
    extensions: Optional[Dict[int, Any]] = None
    auth_data_obj: Optional[AuthenticatorData] = None

    @classmethod
    def from_dict(cls, response: Dict[int, Any]) -> "AssertionResponse":
        auth_data = response.get(_RESP_AUTH_DATA_ASSERTION)
        signature = response.get(_RESP_SIGNATURE)
        if not isinstance(auth_data, bytes):
            raise WebAuthnError("Resposta getAssertion sem authData")
        if not isinstance(signature, bytes):
            raise WebAuthnError("Resposta getAssertion sem signature")

        credential: Optional[PublicKeyCredentialDescriptor] = None
        descriptor = response.get(_RESP_CREDENTIAL)
        if isinstance(descriptor, dict):
            credential = PublicKeyCredentialDescriptor.from_cbor(descriptor)

        user: Optional[UserEntity] = None
        user_map = response.get(_RESP_USER)
        if isinstance(user_map, dict):
            user_id = user_map.get(_USER_ID)
            if isinstance(user_id, bytes):
                user = UserEntity(
                    id=user_id,
                    name=user_map.get(_USER_NAME),
                    display_name=user_map.get(_USER_DISPLAY_NAME),
                    icon=user_map.get(_USER_ICON),
                )

        num = response.get(_RESP_NUM_CREDENTIALS)
        extensions = response.get(_RESP_EXTENSIONS_ASSERTION)
        obj: Optional[AuthenticatorData] = None
        try:
            obj = AuthenticatorData.parse(auth_data)
        except WebAuthnError:
            obj = None
        return cls(
            auth_data=auth_data,
            signature=signature,
            credential=credential,
            user=user,
            number_of_credentials=num if isinstance(num, int) else None,
            extensions=extensions if isinstance(extensions, dict) else None,
            auth_data_obj=obj,
        )


def normalize_cred_params(pub_key_cred_params: List[Any]) -> List[Dict[int, Any]]:
    """Normaliza a lista de pubKeyCredParams para os mapas CBOR `{3: alg}`.

    Aceita inteiros (algoritmos COSE) ou mapas já prontos.
    """
    result: List[Dict[int, Any]] = []
    for item in pub_key_cred_params:
        if isinstance(item, int):
            result.append({3: item})
        elif isinstance(item, dict):
            result.append(dict(item))
        else:
            raise WebAuthnError(
                "pubKeyCredParams deve conter inteiros ou mapas {3: alg}"
            )
    return result
