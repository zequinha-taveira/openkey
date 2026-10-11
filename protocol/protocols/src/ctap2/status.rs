//! Códigos de Status/Erro do Protocolo CTAP2 (FIDO CTAP2.1 Specification)

/// Códigos de resposta CTAP2
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctap2Status {
    Ok = 0x00,
    ErrInvalidCommand = 0x01,
    ErrInvalidParameter = 0x02,
    ErrInvalidLength = 0x03,
    ErrInvalidSeq = 0x04,
    ErrTimeout = 0x05,
    ErrChannelBusy = 0x06,
    ErrLockRequired = 0x0a,
    ErrInvalidChannel = 0x0b,
    ErrCborUnexpectedType = 0x11,
    ErrInvalidCbor = 0x12,
    ErrMissingParameter = 0x14,
    ErrLimitExceeded = 0x15,
    ErrUnsupportedExtension = 0x16,
    ErrCredentialExcluded = 0x21,
    ErrUnsupportedAlgorithm = 0x27,
    ErrOperationDenied = 0x28,
    ErrPinInvalid = 0x2e,
    ErrPinBlocked = 0x2f,
    ErrPinAuthInvalid = 0x30,
    ErrPinAuthBlocked = 0x31,
    ErrPinNotSet = 0x32,
    ErrPinRequired = 0x33,
    ErrPinPolicyViolation = 0x34,
    ErrPinTokenExpired = 0x35,
    ErrNoCredentials = 0x36,
    ErrUserActionTimeout = 0x37,
    ErrNotAllowed = 0x38,
    ErrOther = 0x7f,
}

impl Ctap2Status {
    pub fn to_u8(&self) -> u8 {
        *self as u8
    }
}
