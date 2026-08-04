"""Cliente do comando authenticatorCredentialManagement (FIDO CTAP2.1 Section 6.8).

Permite inspecionar e gerenciar credenciais residentes no autenticador:
metadata, enumerateRPs, enumerateCredentials e deleteCredential.

Referência: FIDO CTAP2.1 spec, seção 6.8 (authenticatorCredentialManagement).
"""

import hashlib
import hmac
from dataclasses import dataclass
from typing import Any, Dict, List, Optional

import cbor2

from openkey.ctap2 import CMD_CREDENTIAL_MGMT, Ctap2Client
from openkey.exceptions import OpenKeyError
from openkey.pin_client import PinClient

# Subcomandos do authenticatorCredentialManagement (CTAP2.1 §6.8.1)
SUB_GET_CREDS_METADATA = 0x01
SUB_ENUMERATE_RPS_BEGIN = 0x02
SUB_ENUMERATE_RPS_NEXT = 0x03
SUB_ENUMERATE_CREDENTIALS_BEGIN = 0x04
SUB_ENUMERATE_CREDENTIALS_NEXT = 0x05
SUB_DELETE_CREDENTIAL = 0x06
SUB_UPDATE_USER_INFORMATION = 0x07

# Chaves CBOR do request
_CBOR_SUBCOMMAND = 0x01
_CBOR_PIN_PROTOCOL = 0x03
_CBOR_PIN_AUTH = 0x04
_CBOR_RP_ID = 0x05
_CBOR_CREDENTIAL_ID = 0x06

# Chaves CBOR da resposta
_RESP_TOTAL = 0x01
_RESP_EXISTING_COUNT = 0x01
_RESP_MAX_COUNT = 0x02
_RESP_RP = 0x02
_RESP_CREDENTIAL_INFO = 0x02
_RESP_USER = 0x03
_RESP_RP_INFO = 0x04

# Chaves do PublicKeyCredentialDescriptor / credentialInfo
_DESC_ID = 0x01
_DESC_TYPE = 0x02
_CRED_RP_ID = 0x03

# Chaves do mapa user
_USER_ID = 0x01
_USER_NAME = 0x02
_USER_DISPLAY_NAME = 0x03
_USER_ICON = 0x04

# Chaves do mapa rp
_RP_ID = 0x01
_RP_NAME = 0x02
_RP_ICON = 0x03

_CREDENTIAL_TYPE = "public-key"


class CredentialManagementError(OpenKeyError):
    """Erro no comando authenticatorCredentialManagement."""


@dataclass
class RpInfo:
    """Relaying Party listada pelo autenticador."""

    id: str
    name: Optional[str] = None
    icon: Optional[str] = None

    @classmethod
    def from_cose_map(cls, rp: Dict[int, Any]) -> "RpInfo":
        rp_id = rp.get(_RP_ID)
        if not isinstance(rp_id, str):
            raise CredentialManagementError("RP sem campo id")
        name = rp.get(_RP_NAME)
        icon = rp.get(_RP_ICON)
        return cls(
            id=rp_id,
            name=name if isinstance(name, str) else None,
            icon=icon if isinstance(icon, str) else None,
        )


@dataclass
class UserInfo:
    """Informações de usuário associadas a uma credencial residente."""

    id: bytes
    name: Optional[str] = None
    display_name: Optional[str] = None
    icon: Optional[str] = None

    @classmethod
    def from_cose_map(cls, user: Dict[int, Any]) -> "UserInfo":
        user_id = user.get(_USER_ID)
        if not isinstance(user_id, bytes):
            raise CredentialManagementError("Usuário sem campo id")
        name = user.get(_USER_NAME)
        display_name = user.get(_USER_DISPLAY_NAME)
        icon = user.get(_USER_ICON)
        return cls(
            id=user_id,
            name=name if isinstance(name, str) else None,
            display_name=display_name if isinstance(display_name, str) else None,
            icon=icon if isinstance(icon, str) else None,
        )


@dataclass
class CredentialInfo:
    """Credencial residente (credentialInfo + user + rp)."""

    credential_id: bytes
    credential_type: str = _CREDENTIAL_TYPE
    rp_id: Optional[str] = None
    user: Optional[UserInfo] = None
    rp: Optional[RpInfo] = None

    @classmethod
    def from_response(cls, credential_info: Dict[int, Any]) -> "CredentialInfo":
        descriptor = credential_info.get(_DESC_ID)
        if not isinstance(descriptor, dict):
            raise CredentialManagementError("credentialInfo sem descriptor")
        cred_id = descriptor.get(_DESC_ID)
        if not isinstance(cred_id, bytes):
            raise CredentialManagementError("Descriptor sem credentialID")
        cred_type = descriptor.get(_DESC_TYPE)
        rp_id = credential_info.get(_CRED_RP_ID)
        return cls(
            credential_id=cred_id,
            credential_type=cred_type if isinstance(cred_type, str) else _CREDENTIAL_TYPE,
            rp_id=rp_id if isinstance(rp_id, str) else None,
        )


class CredentialManagementClient:
    """Cliente de alto nível do authenticatorCredentialManagement.

    Requer um ``PinClient`` autenticado (key agreement já realizado) e o PIN do
    dispositivo. O ``pinUvAuthToken`` é obtido via getPINToken e usado para
    calcular o ``pinAuth`` de cada subcomando.
    """

    def __init__(self, ctap2: Ctap2Client, pin_client: PinClient, pin: str):
        self._ctap2 = ctap2
        self._pin_client = pin_client
        self._pin = pin
        self._protocol_version = pin_client._protocol.version
        self._token: Optional[bytes] = None

    # ------------------------------------------------------------------
    # Operações
    # ------------------------------------------------------------------

    def get_metadata(self) -> Dict[str, int]:
        """Executa getCredsMetadata (0x01)."""
        response = self._cmd(SUB_GET_CREDS_METADATA, message=b"\x01")
        existing = response.get(_RESP_EXISTING_COUNT)
        max_count = response.get(_RESP_MAX_COUNT)
        if not isinstance(existing, int) or not isinstance(max_count, int):
            raise CredentialManagementError(
                "Resposta getCredsMetadata sem contadores"
            )
        return {
            "existing_count": existing,
            "max_count": max_count,
        }

    def enumerate_rps(self) -> List[RpInfo]:
        """Enumera todas as Relaying Parties (enumerateRPs + paginação)."""
        self._ensure_token()
        response = self._cmd(SUB_ENUMERATE_RPS_BEGIN, message=b"\x02")
        total = response.get(_RESP_TOTAL, 0)
        rps: List[RpInfo] = []
        first = response.get(_RESP_RP)
        if isinstance(first, dict):
            rps.append(RpInfo.from_cose_map(first))
        while len(rps) < total:
            response = self._cmd(SUB_ENUMERATE_RPS_NEXT, message=b"\x03")
            rp = response.get(_RESP_RP)
            if not isinstance(rp, dict):
                break
            rps.append(RpInfo.from_cose_map(rp))
        return rps

    def enumerate_credentials(self, rp_id: str) -> List[CredentialInfo]:
        """Enumera as credenciais residentes de uma RP (enumerateCredentials).

        ``rp_id`` é o identificador (ex.: "example.com") cujo hash SHA-256
        identifica a RP no dispositivo.
        """
        self._ensure_token()
        rp_id_hash = hashlib.sha256(rp_id.encode("utf-8")).digest()
        message = bytes([SUB_ENUMERATE_CREDENTIALS_BEGIN]) + rp_id_hash
        response = self._cmd(
            SUB_ENUMERATE_CREDENTIALS_BEGIN,
            message=message,
            rp_id=rp_id,
        )
        total = response.get(_RESP_TOTAL, 0)
        credentials: List[CredentialInfo] = []
        first = response.get(_RESP_CREDENTIAL_INFO)
        if isinstance(first, dict):
            credentials.append(self._parse_credential(response))
        while len(credentials) < total:
            response = self._cmd(
                SUB_ENUMERATE_CREDENTIALS_NEXT,
                message=b"\x05",
                rp_id=rp_id,
            )
            credential = response.get(_RESP_CREDENTIAL_INFO)
            if not isinstance(credential, dict):
                break
            credentials.append(self._parse_credential(response))
        return credentials

    def delete_credential(self, credential_id: bytes, rp_id: str) -> None:
        """Remove uma credencial residente (deleteCredential)."""
        self._ensure_token()
        message = bytes([SUB_DELETE_CREDENTIAL]) + credential_id
        descriptor = {_DESC_ID: credential_id, _DESC_TYPE: _CREDENTIAL_TYPE}
        self._cmd(
            SUB_DELETE_CREDENTIAL,
            message=message,
            rp_id=rp_id,
            credential_id=descriptor,
        )

    # ------------------------------------------------------------------
    # Internos
    # ------------------------------------------------------------------

    def _parse_credential(self, response: Dict[int, Any]) -> CredentialInfo:
        info = CredentialInfo.from_response(response[_RESP_CREDENTIAL_INFO])
        user = response.get(_RESP_USER)
        if isinstance(user, dict):
            info.user = UserInfo.from_cose_map(user)
        rp = response.get(_RESP_RP_INFO)
        if isinstance(rp, dict):
            info.rp = RpInfo.from_cose_map(rp)
        return info

    def _ensure_token(self) -> bytes:
        if self._token is None:
            self._token = self._pin_client.get_pin_token(self._pin)
        return self._token

    def _cmd(
        self,
        subcommand: int,
        message: bytes,
        rp_id: Optional[str] = None,
        credential_id: Optional[Dict[int, Any]] = None,
    ) -> Dict[int, Any]:
        pin_auth = hmac.new(self._ensure_token(), message, hashlib.sha256).digest()
        request: Dict[int, Any] = {
            _CBOR_SUBCOMMAND: subcommand,
            _CBOR_PIN_PROTOCOL: self._protocol_version,
            _CBOR_PIN_AUTH: pin_auth,
        }
        if rp_id is not None:
            request[_CBOR_RP_ID] = rp_id
        if credential_id is not None:
            request[_CBOR_CREDENTIAL_ID] = credential_id
        payload = self._ctap2._call_raw(CMD_CREDENTIAL_MGMT, request)
        if not payload:
            raise CredentialManagementError(
                "Resposta authenticatorCredentialManagement vazia"
            )
        response = cbor2.loads(payload)
        if not isinstance(response, dict):
            raise CredentialManagementError(
                "Resposta authenticatorCredentialManagement inválida"
            )
        return response


def cred_management_supported(ctap2: Ctap2Client) -> bool:
    """Verifica se o autenticador suporta o comando de gestão de credenciais.

    O bit correspondente da opção ``credentialMgmt``/``credentialManagement``
    no authenticatorGetInfo indica o suporte.
    """
    from openkey.ctap2 import GetInfoResponse

    info: GetInfoResponse = ctap2.get_info()
    options = info.options
    return bool(
        options.get("credentialMgmt")
        or options.get("credentialManagement")
        or options.get("credential_mgmt")
    )
