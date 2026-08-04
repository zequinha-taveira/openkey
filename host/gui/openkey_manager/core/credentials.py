"""Serviço de credenciais residentes (sem Qt).

Encapsula a enumeração (RPs → credenciais), a visualização de detalhes e a
remoção de credenciais residentes, traduzindo os objetos do
``openkey-sdk`` (``RpInfo``/``CredentialInfo``/``UserInfo``) para o modelo de
domínio ``Credential`` usado pela GUI.
"""

from typing import Callable, List, Optional

from openkey_manager.core.device import DeviceController
from openkey_manager.core.models import Credential


class CredentialError(Exception):
    """Erro nas operações de credenciais residentes."""


PinProvider = Callable[[], Optional[str]]


class CredentialService:
    """Serviço de alto nível para credenciais residentes.

    O ``pin_provider`` é invocado na primeira operação para obter o PIN do
    dispositivo (usado apenas para derivar o ``pinUvAuthToken`` efêmero).
    """

    def __init__(
        self,
        controller: DeviceController,
        pin_provider: Optional[PinProvider] = None,
    ):
        self._controller = controller
        self._pin_provider = pin_provider
        self._client = None

    # ------------------------------------------------------------------
    # Operações
    # ------------------------------------------------------------------

    def list_credentials(self) -> List[Credential]:
        """Enumera todas as credenciais residentes de todas as RPs."""
        client = self._ensure_client()
        credentials: List[Credential] = []
        for rp in client.enumerate_rps():
            for info in client.enumerate_credentials(rp.id):
                credentials.append(self._to_model(rp, info))
        return credentials

    def delete_credential(self, credential_id: bytes, rp_id: str) -> None:
        """Remove uma credencial residente específica."""
        client = self._ensure_client()
        client.delete_credential(credential_id, rp_id)

    def reset_session(self) -> None:
        """Descarta o cliente e o token obtidos (após mudança de PIN/desconexão)."""
        self._client = None

    # ------------------------------------------------------------------
    # Internos
    # ------------------------------------------------------------------

    def _ensure_client(self):
        if self._client is None:
            pin = self._request_pin()
            self._client = self._controller.credential_manager(pin)
        return self._client

    def _request_pin(self) -> str:
        if self._pin_provider is not None:
            pin = self._pin_provider()
            if pin:
                return pin
        raise CredentialError("PIN necessário para acessar credenciais residentes")

    @staticmethod
    def _to_model(rp, info) -> Credential:
        user = getattr(info, "user", None)
        user_id = getattr(user, "id", None) if user is not None else None
        user_name = getattr(user, "name", None) if user is not None else None
        user_display = (
            getattr(user, "display_name", None) if user is not None else None
        )
        rp_name = getattr(rp, "name", None)
        return Credential(
            rp_id=info.rp_id or rp.id,
            rp_name=rp_name,
            credential_id=info.credential_id,
            user_id=user_id,
            user_name=user_name,
            user_display_name=user_display,
        )
