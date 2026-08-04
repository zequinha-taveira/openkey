"""Descoberta de dispositivos com detecção de attach/detach (sem Qt).

O ``DiscoveryService`` mantém um snapshot dos dispositivos conectados e
notifica ouvintes quando um dispositivo é conectado (attach) ou removido
(detach), permitindo que a GUI faça *auto-refresh* periódico (G10-T07).
"""

from typing import Callable, Dict, List, Optional, Tuple

from openkey_manager.core.device import DeviceBackend, DeviceCandidate

DiscoveryListener = Callable[
    [List[DeviceCandidate], List[DeviceCandidate]], None
]


def _candidate_key(candidate: DeviceCandidate) -> str:
    """Chave estável para comparar um dispositivo entre refreshs."""
    if candidate.path is not None:
        return f"path:{candidate.path.hex()}"
    return (
        f"{candidate.vid:04X}:{candidate.pid:04X}:"
        f"{candidate.serial_number}"
    )


class DiscoveryService:
    """Snapshot de dispositivos e notificação de attach/detach."""

    def __init__(self, backend: Optional[DeviceBackend] = None):
        self._backend = backend if backend is not None else DeviceBackend()
        self._known: Dict[str, DeviceCandidate] = {}
        self._listeners: List[DiscoveryListener] = []

    # ------------------------------------------------------------------
    # Estado
    # ------------------------------------------------------------------

    def refresh(self) -> List[DeviceCandidate]:
        """Reexecuta a descoberta e notifica attach/detach.

        Returns:
            Lista atual de dispositivos conectados.
        """
        current: Dict[str, DeviceCandidate] = {}
        for candidate in self._backend.discover():
            current[_candidate_key(candidate)] = candidate

        attached = [
            candidate
            for key, candidate in current.items()
            if key not in self._known
        ]
        detached = [
            candidate
            for key, candidate in self._known.items()
            if key not in current
        ]

        self._known = current
        if attached or detached:
            for listener in list(self._listeners):
                listener(attached, detached)
        return list(current.values())

    def snapshot(self) -> List[DeviceCandidate]:
        """Lista atual dos dispositivos conhecidos (sem rediscover)."""
        return list(self._known.values())

    def add_listener(self, callback: DiscoveryListener) -> None:
        """Registra um ouvinte ``(attached, detached)``."""
        self._listeners.append(callback)

    def clear(self) -> None:
        """Esquece o snapshot atual (não fecha conexões)."""
        self._known.clear()

    @property
    def backend(self) -> DeviceBackend:
        return self._backend
