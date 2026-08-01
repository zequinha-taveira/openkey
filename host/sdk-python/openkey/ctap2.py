"""Cliente CTAP2 para envio e recebimento de comandos CBOR"""

import cbor2
from typing import Dict, Any, Optional
from openkey.exceptions import CtapError, OpenKeyError

# Códigos de Comando CTAP2
CMD_MAKE_CREDENTIAL = 0x01
CMD_GET_ASSERTION = 0x02
CMD_GET_INFO = 0x04
CMD_CLIENT_PIN = 0x06
CMD_RESET = 0x07
CMD_CREDENTIAL_MGMT = 0x0A

# Códigos de Resposta/Status
STATUS_OK = 0x00

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

    def __init__(self, transport_send_func):
        """
        transport_send_func: função `send(cmd: int, payload: bytes) -> bytes`
        """
        self._send = transport_send_func

    def _call(self, ctap_cmd: int, payload_data: Optional[Dict[int, Any]] = None) -> bytes:
        """Envia um comando CTAP2 e valida o byte de status retornado"""
        payload_bytes = cbor2.dumps(payload_data) if payload_data is not None else b""
        response_bytes = self._send(ctap_cmd, payload_bytes)

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
