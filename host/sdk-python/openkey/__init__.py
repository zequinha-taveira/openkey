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
from openkey.pin import PinUvAuthProtocol, CoseEc2Key, PinError, pin_hash, pad_pin
from openkey.pin_client import (
    PinClient,
    setup_pin,
    change_device_pin,
)
from openkey.credential_management import (
    CredentialManagementClient,
    CredentialManagementError,
    RpInfo,
    UserInfo,
    CredentialInfo,
)
from openkey.webauthn import (
    AuthenticatorData,
    AssertionResponse,
    MakeCredentialResponse,
    PublicKeyCredentialDescriptor,
    RpEntity,
    UserEntity,
    WebAuthnError,
)
from openkey.ctap2 import (
    CtapLogEntry,
    CtapLogRecorder,
    LOG_SEND,
    LOG_RECV,
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
    "PinUvAuthProtocol",
    "CoseEc2Key",
    "PinError",
    "pin_hash",
    "pad_pin",
    "PinClient",
    "setup_pin",
    "change_device_pin",
    "CredentialManagementClient",
    "CredentialManagementError",
    "RpInfo",
    "UserInfo",
    "CredentialInfo",
    "AuthenticatorData",
    "AssertionResponse",
    "MakeCredentialResponse",
    "PublicKeyCredentialDescriptor",
    "RpEntity",
    "UserEntity",
    "WebAuthnError",
    "CtapLogEntry",
    "CtapLogRecorder",
    "LOG_SEND",
    "LOG_RECV",
]
