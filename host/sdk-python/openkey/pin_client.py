"""Cliente do protocolo authenticatorClientPIN sobre o transporte CTAP2"""

import cbor2
from typing import Any, Dict, Optional

from openkey.ctap2 import CMD_CLIENT_PIN, Ctap2Client
from openkey.exceptions import OpenKeyError
from openkey.pin import (
    PIN_PROTOCOL_V1,
    CoseEc2Key,
    PinError,
    PinUvAuthProtocol,
    parse_pin_retries,
)

# Subcomandos do authenticatorClientPIN
_SUB_GET_RETRIES = 0x01
_SUB_GET_KEY_AGREEMENT = 0x02
_SUB_SET_PIN = 0x03
_SUB_CHANGE_PIN = 0x04
_SUB_GET_PIN_TOKEN = 0x05

# Chaves CBOR da resposta
_RESP_KEY_AGREEMENT = 0x01
_RESP_PIN_UV_AUTH_TOKEN = 0x02
_RESP_PIN_RETRIES = 0x03


class PinClient:
    """Cliente de alto nível para o protocolo authenticatorClientPIN.

    Encapsula o handshake de key agreement e expõe operações de PIN usando o
    ``PinUvAuthProtocol`` (v1 ou v2) escolhido pelo autenticador.
    """

    def __init__(self, ctap2: Ctap2Client, protocol_version: int = PIN_PROTOCOL_V1):
        self._ctap2 = ctap2
        self._protocol = PinUvAuthProtocol(version=protocol_version)
        self._peer_public_key: Optional[CoseEc2Key] = None

    def _client_pin(self, params: Dict[int, Any]) -> Dict[int, Any]:
        """Envia um subcomando authenticatorClientPIN e decodifica a resposta."""
        payload = self._ctap2._call_raw(CMD_CLIENT_PIN, params)
        if not payload:
            raise PinError("Resposta authenticatorClientPIN vazia")
        response = cbor2.loads(payload)
        if not isinstance(response, dict):
            raise PinError("Resposta authenticatorClientPIN inválida")
        return response

    def get_key_agreement(self) -> CoseEc2Key:
        """Executa getKeyAgreement e armazena a chave pública do autenticador."""
        response = self._client_pin(self._protocol.get_key_agreement_request())
        key = response.get(_RESP_KEY_AGREEMENT)
        if not isinstance(key, dict):
            raise PinError("Resposta getKeyAgreement sem COSE_Key")
        peer = CoseEc2Key.from_cose(key)
        self._peer_public_key = peer
        self._protocol.set_peer_public_key(key)
        return peer

    def get_pin_retries(self) -> int:
        """Consulta as tentativas restantes de PIN (getPinRetries)."""
        response = self._client_pin(self._protocol.get_pin_retries_request())
        return parse_pin_retries(response)

    def set_pin(self, new_pin: str) -> None:
        """Define o PIN do dispositivo (setPIN). Requer key agreement prévio."""
        self._ensure_key_agreement()
        self._client_pin(self._protocol.set_pin_request(new_pin))

    def change_pin(self, current_pin: str, new_pin: str) -> None:
        """Altera o PIN (changePIN). Requer key agreement prévio."""
        self._ensure_key_agreement()
        self._client_pin(self._protocol.change_pin_request(current_pin, new_pin))

    def get_pin_token(self, pin: str) -> bytes:
        """Obtém o pinUvAuthToken (getPINToken). Requer key agreement prévio."""
        self._ensure_key_agreement()
        response = self._client_pin(self._protocol.get_pin_token_request(pin))
        token = response.get(_RESP_PIN_UV_AUTH_TOKEN)
        if not isinstance(token, bytes):
            raise PinError("Resposta getPINToken sem pinUvAuthToken")
        return token

    def _ensure_key_agreement(self) -> None:
        if self._peer_public_key is None:
            self.get_key_agreement()


def setup_pin(ctap2: Ctap2Client, new_pin: str, protocol_version: int = PIN_PROTOCOL_V1) -> PinClient:
    """Conveniência: cria um PinClient, faz key agreement e define o PIN."""
    client = PinClient(ctap2, protocol_version=protocol_version)
    client.get_key_agreement()
    client.set_pin(new_pin)
    return client


def change_device_pin(
    ctap2: Ctap2Client,
    current_pin: str,
    new_pin: str,
    protocol_version: int = PIN_PROTOCOL_V1,
) -> PinClient:
    """Conveniência: cria um PinClient, faz key agreement e altera o PIN."""
    client = PinClient(ctap2, protocol_version=protocol_version)
    client.get_key_agreement()
    client.change_pin(current_pin, new_pin)
    return client
