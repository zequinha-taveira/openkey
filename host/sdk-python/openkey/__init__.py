"""OpenKey Python SDK"""

from openkey.client import OpenKeyDevice
from openkey.ctap2 import Ctap2Client, GetInfoResponse
from openkey.exceptions import OpenKeyError, TransportError, CtapError
from openkey.transport import CtapHidPacket, CtapHidMessageAssembler
from openkey.hid import (
    HidTransportBackend,
    OPENKEY_VID,
    OPENKEY_PID,
    discover_devices,
    open_device,
)

__version__ = "0.6.0"

__all__ = [
    "OpenKeyDevice",
    "Ctap2Client",
    "GetInfoResponse",
    "OpenKeyError",
    "TransportError",
    "CtapError",
    "CtapHidPacket",
    "CtapHidMessageAssembler",
    "HidTransportBackend",
    "OPENKEY_VID",
    "OPENKEY_PID",
    "discover_devices",
    "open_device",
]
