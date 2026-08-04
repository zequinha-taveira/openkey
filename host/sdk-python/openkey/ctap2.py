"""Cliente CTAP2 para envio e recebimento de comandos CBOR"""

import time
from dataclasses import dataclass, field
from typing import Any, Callable, Dict, List, Optional

import cbor2

from openkey.exceptions import CtapError, OpenKeyError
from openkey.webauthn import (
    AssertionResponse,
    MakeCredentialResponse,
    PublicKeyCredentialDescriptor,
    RpEntity,
    UserEntity,
    normalize_cred_params,
)

# Códigos de Comando CTAP2
CMD_MAKE_CREDENTIAL = 0x01
CMD_GET_ASSERTION = 0x02
CMD_GET_INFO = 0x04
CMD_CLIENT_PIN = 0x06
CMD_RESET = 0x07
CMD_CREDENTIAL_MGMT = 0x0A

# Códigos de Resposta/Status
STATUS_OK = 0x00

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

# Chaves CBOR do request de getAssertion (CTAP2.1 §6.3)
_CBOR_RP_ID = 0x01
_CBOR_CLIENT_DATA_HASH_ASSERTION = 0x02
_CBOR_ALLOW_LIST = 0x03
_CBOR_EXTENSIONS_ASSERTION = 0x04
_CBOR_OPTIONS_ASSERTION = 0x05
_CBOR_PIN_UV_AUTH_PARAM_ASSERTION = 0x06
_CBOR_PIN_UV_AUTH_PROTOCOL_ASSERTION = 0x07

# Direções do hook de log
LOG_SEND = "send"
LOG_RECV = "recv"

# Hook de logging de pacotes CTAP: (direção, comando CTAP2, payload bruto)
CtapLogHook = Callable[[str, int, bytes], None]


@dataclass
class CtapLogEntry:
    """Uma entrada de log de pacote CTAP (para o visualizador da GUI)."""

    direction: str
    command: int
    payload: bytes
    timestamp: float = field(default_factory=time.time)

    @property
    def payload_hex(self) -> str:
        return self.payload.hex()

    @property
    def command_name(self) -> str:
        names = {
            CMD_MAKE_CREDENTIAL: "makeCredential",
            CMD_GET_ASSERTION: "getAssertion",
            CMD_GET_INFO: "getInfo",
            CMD_CLIENT_PIN: "clientPin",
            CMD_RESET: "reset",
            CMD_CREDENTIAL_MGMT: "credentialManagement",
        }
        return names.get(self.command, f"0x{self.command:02x}")


class CtapLogRecorder:
    """Coleta entradas de log de pacotes CTAP (hook pronto para a GUI)."""

    def __init__(self) -> None:
        self.entries: List[CtapLogEntry] = []

    def record(self, direction: str, command: int, payload: bytes) -> None:
        self.entries.append(CtapLogEntry(direction, command, bytes(payload)))

    def clear(self) -> None:
        self.entries.clear()

    @property
    def commands_sent(self) -> List[int]:
        return [e.command for e in self.entries if e.direction == LOG_SEND]


def _noop_log(direction: str, command: int, payload: bytes) -> None:
    pass

class GetInfoResponse:
    """Resposta do comando authenticatorGetInfo"""
    def __init__(self, raw_dict: Dict[int, Any]):
        self.raw = raw_dict
        self.versions = raw_dict.get(1, [])
        self.extensions = raw_dict.get(2, [])
        self.aaguid = raw_dict.get(3, b"")
        self.options = raw_dict.get(4, {})
        self.max_msg_size = raw_dict.get(5, 1200)
        self.pin_uv_auth_protocols = raw_dict.get(6, [])

    def __repr__(self) -> str:
        aaguid_hex = self.aaguid.hex() if isinstance(self.aaguid, bytes) else str(self.aaguid)
        return (f"<GetInfoResponse versions={self.versions} aaguid={aaguid_hex} "
                f"options={self.options}>")

class Ctap2Client:
    """Cliente CTAP2 de alto nível"""

    def __init__(self, transport_send_func, log_hook: Optional[CtapLogHook] = None):
        """
        transport_send_func: função `send(cmd: int, payload: bytes) -> bytes`
        log_hook: callable `(direction, ctap_cmd, payload) -> None` invocado
            antes de enviar (LOG_SEND) e após receber (LOG_RECV) cada comando.
        """
        self._send = transport_send_func
        self._log = log_hook if log_hook is not None else _noop_log

    def _call(self, ctap_cmd: int, payload_data: Optional[Dict[int, Any]] = None) -> bytes:
        """Envia um comando CTAP2 e valida o byte de status retornado"""
        payload_bytes = cbor2.dumps(payload_data) if payload_data is not None else b""
        self._log(LOG_SEND, ctap_cmd, payload_bytes)
        response_bytes = self._send(ctap_cmd, payload_bytes)
        self._log(LOG_RECV, ctap_cmd, response_bytes)

        if not response_bytes:
            raise OpenKeyError("Resposta CTAP2 vazia")

        status_code = response_bytes[0]
        if status_code != STATUS_OK:
            raise CtapError(status_code)

        return response_bytes[1:]

    def _call_raw(self, ctap_cmd: int, payload_data: Optional[Dict[int, Any]] = None) -> bytes:
        """Envia um comando CTAP2 e retorna o payload CBOR da resposta.

        Diferente de ``_call``, não exige que o payload seja CBOR; retorna os
        bytes após o byte de status.
        """
        return self._call(ctap_cmd, payload_data)

    def get_info(self) -> GetInfoResponse:
        """Executa authenticatorGetInfo (0x04)"""
        cbor_response = self._call(CMD_GET_INFO, None)
        if not cbor_response:
            return GetInfoResponse({})
        raw_dict = cbor2.loads(cbor_response)
        return GetInfoResponse(raw_dict)

    def reset(self) -> None:
        """Executa authenticatorReset (0x07)"""
        self._call(CMD_RESET, None)

    # ------------------------------------------------------------------
    # makeCredential / getAssertion
    # ------------------------------------------------------------------

    def make_credential(
        self,
        client_data_hash: bytes,
        rp: RpEntity,
        user: UserEntity,
        pub_key_cred_params: List[Any],
        exclude_list: Optional[List[PublicKeyCredentialDescriptor]] = None,
        extensions: Optional[Dict[int, Any]] = None,
        options: Optional[Dict[str, Any]] = None,
        pin_uv_auth_param: Optional[bytes] = None,
        pin_uv_auth_protocol: Optional[int] = None,
    ) -> MakeCredentialResponse:
        """Executa authenticatorMakeCredential (0x01).

        ``pub_key_cred_params`` aceita uma lista de algoritmos COSE (ex.:
        ``[-7, -257]``) ou de mapas CBOR ``{3: alg}``.
        """
        if len(client_data_hash) != 32:
            raise ValueError("clientDataHash deve ter 32 bytes")

        request: Dict[int, Any] = {
            _CBOR_CLIENT_DATA_HASH: client_data_hash,
            _CBOR_RP: rp.to_cbor(),
            _CBOR_USER: user.to_cbor(),
            _CBOR_PUB_KEY_CRED_PARAMS: normalize_cred_params(pub_key_cred_params),
        }
        if exclude_list is not None:
            request[_CBOR_EXCLUDE_LIST] = [d.to_cbor() for d in exclude_list]
        if extensions is not None:
            request[_CBOR_EXTENSIONS] = extensions
        if options is not None:
            request[_CBOR_OPTIONS] = options
        if pin_uv_auth_param is not None:
            request[_CBOR_PIN_UV_AUTH_PARAM] = pin_uv_auth_param
        if pin_uv_auth_protocol is not None:
            request[_CBOR_PIN_UV_AUTH_PROTOCOL] = pin_uv_auth_protocol

        cbor_response = self._call(CMD_MAKE_CREDENTIAL, request)
        raw_dict = cbor2.loads(cbor_response)
        return MakeCredentialResponse.from_dict(raw_dict)

    def get_assertion(
        self,
        rp_id: str,
        client_data_hash: bytes,
        allow_list: Optional[List[PublicKeyCredentialDescriptor]] = None,
        extensions: Optional[Dict[int, Any]] = None,
        options: Optional[Dict[str, Any]] = None,
        pin_uv_auth_param: Optional[bytes] = None,
        pin_uv_auth_protocol: Optional[int] = None,
    ) -> AssertionResponse:
        """Executa authenticatorGetAssertion (0x02)."""
        if len(client_data_hash) != 32:
            raise ValueError("clientDataHash deve ter 32 bytes")

        request: Dict[int, Any] = {
            _CBOR_RP_ID: rp_id,
            _CBOR_CLIENT_DATA_HASH_ASSERTION: client_data_hash,
        }
        if allow_list is not None:
            request[_CBOR_ALLOW_LIST] = [d.to_cbor() for d in allow_list]
        if extensions is not None:
            request[_CBOR_EXTENSIONS_ASSERTION] = extensions
        if options is not None:
            request[_CBOR_OPTIONS_ASSERTION] = options
        if pin_uv_auth_param is not None:
            request[_CBOR_PIN_UV_AUTH_PARAM_ASSERTION] = pin_uv_auth_param
        if pin_uv_auth_protocol is not None:
            request[_CBOR_PIN_UV_AUTH_PROTOCOL_ASSERTION] = pin_uv_auth_protocol

        cbor_response = self._call(CMD_GET_ASSERTION, request)
        raw_dict = cbor2.loads(cbor_response)
        return AssertionResponse.from_dict(raw_dict)
