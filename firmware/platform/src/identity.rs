//! Device Identity - identidade do dispositivo de segurança
//!
//! Gerencia o AAGUID (Authenticator Attestation GUID), certificado de
//! atestação e validação de identidade no boot.
//!
//! ## Segurança
//!
//! - AAGUID é derivado determinísticamente a partir do Board Profile ID
//! - Certificado de atestação é lido de OTP (nunca gerado no firmware)
//! - Chave de atestação é gerenciada pelo KeyProvider (HSM/OTP)

use crate::board::BoardProfileId;
use crate::device::DeviceProfile;
use crate::hal::OtpProvider;
use openkey_crypto::keys::{
    derive_aaguid, AttestationAlgorithm, AttestationKeyProvider, AAGUID_SIZE,
};

/// AAGUID (Authenticator Attestation GUID)
///
/// Identificador único de 16 bytes que identifica o modelo do autenticador.
/// É derivado de forma determinística a partir do Board Profile ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Aaguid([u8; AAGUID_SIZE]);

impl Aaguid {
    /// Cria um AAGUID vazio
    pub const fn empty() -> Self {
        Self([0u8; AAGUID_SIZE])
    }

    /// Cria um AAGUID a partir de bytes
    pub const fn from_bytes(bytes: [u8; AAGUID_SIZE]) -> Self {
        Self(bytes)
    }

    /// Deriva um AAGUID a partir de um Board Profile ID
    pub fn from_board_id(board_id: &BoardProfileId) -> Self {
        Self(derive_aaguid(&board_id.0))
    }

    /// Retorna os bytes do AAGUID
    pub fn as_bytes(&self) -> &[u8; AAGUID_SIZE] {
        &self.0
    }
}

/// Estado de provisionamento do dispositivo
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceProvisioningState {
    /// Dispositivo não provisionado
    Unprovisioned,
    /// Dispositivo parcialmente provisionado
    Partial,
    /// Dispositivo totalmente provisionado
    Provisioned,
}

/// Identidade do dispositivo
///
/// Combina AAGUID, Device Profile e estado de provisionamento.
/// É validada no boot para garantir integridade.
#[derive(Debug, Clone)]
pub struct DeviceIdentity {
    /// AAGUID do modelo do dispositivo
    aaguid: Aaguid,
    /// Device Profile (dados do dispositivo)
    device: DeviceProfile,
    /// Estado de provisionamento
    state: DeviceProvisioningState,
    /// Algoritmo de atestação suportado
    attestation_algorithm: AttestationAlgorithm,
}

impl DeviceIdentity {
    /// Cria uma nova identidade de dispositivo
    pub fn new(
        board_id: &BoardProfileId,
        device: DeviceProfile,
        state: DeviceProvisioningState,
        algorithm: AttestationAlgorithm,
    ) -> Self {
        Self {
            aaguid: Aaguid::from_board_id(board_id),
            device,
            state,
            attestation_algorithm: algorithm,
        }
    }

    /// Retorna o AAGUID
    pub fn aaguid(&self) -> &Aaguid {
        &self.aaguid
    }

    /// Retorna o Device Profile
    pub fn device(&self) -> &DeviceProfile {
        &self.device
    }

    /// Retorna o estado de provisionamento
    pub fn state(&self) -> DeviceProvisioningState {
        self.state
    }

    /// Retorna o algoritmo de atestação
    pub fn attestation_algorithm(&self) -> AttestationAlgorithm {
        self.attestation_algorithm
    }

    /// Verifica se o dispositivo está provisionado
    pub fn is_provisioned(&self) -> bool {
        self.state == DeviceProvisioningState::Provisioned
    }

    /// Carrega a identidade do dispositivo a partir do OTP e Device Profile
    ///
    /// # Arguments
    /// * `board_id` — Board Profile ID
    /// * `device` — Device Profile carregado
    /// * `otp` — Provedor de OTP para leitura de dados de fabricação
    /// * `key_provider` — Provedor de chave de atestação
    pub fn load(
        board_id: &BoardProfileId,
        device: DeviceProfile,
        otp: &dyn OtpProvider,
        key_provider: &dyn AttestationKeyProvider,
    ) -> Result<Self, IdentityError> {
        // Verifica se a chave de atestação está disponível
        if !key_provider.is_available() {
            return Err(IdentityError::KeyNotAvailable);
        }

        // Determina o estado de provisionamento
        let state = if otp.total_size() > 0 {
            DeviceProvisioningState::Provisioned
        } else {
            DeviceProvisioningState::Unprovisioned
        };

        // Determina o algoritmo de atestação (P-256 por padrão)
        let algorithm = AttestationAlgorithm::P256;

        Ok(Self::new(board_id, device, state, algorithm))
    }
}

/// Erro de identidade do dispositivo
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentityError {
    /// Chave de atestação não disponível
    KeyNotAvailable,
    /// OTP não programada
    OtpNotProgrammed,
    /// Dados de identidade inválidos
    InvalidIdentity,
    /// Falha de hardware
    HardwareFailure,
}

/// Valida a identidade do dispositivo
///
/// Verifica se a identidade carregada é consistente e válida.
pub fn validate_identity(identity: &DeviceIdentity) -> Result<(), IdentityError> {
    // Verifica se o AAGUID não é vazio
    if identity.aaguid() == &Aaguid::empty() {
        return Err(IdentityError::InvalidIdentity);
    }

    // Verifica se o dispositivo está provisionado
    if !identity.is_provisioned() {
        return Err(IdentityError::InvalidIdentity);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::{DeviceText, UsbIdentity};
    use crate::hal::OtpError;

    /// Mock OTP para testes
    struct MockOtp;

    impl OtpProvider for MockOtp {
        fn read(&self, _offset: u32, buf: &mut [u8]) -> Result<(), OtpError> {
            buf.fill(0);
            Ok(())
        }

        fn is_programmed(&self, _offset: u32, _len: usize) -> Result<bool, OtpError> {
            Ok(true)
        }

        fn total_size(&self) -> u32 {
            1024
        }
    }

    /// Mock key provider para testes
    struct MockKeyProvider;

    impl AttestationKeyProvider for MockKeyProvider {
        fn sign(
            &mut self,
            _algorithm: AttestationAlgorithm,
            _message: &[u8],
        ) -> Result<openkey_crypto::keys::AttestationSignature, openkey_crypto::keys::KeyError> {
            Err(openkey_crypto::keys::KeyError::Unsupported)
        }

        fn public_key(
            &self,
            _algorithm: AttestationAlgorithm,
        ) -> Result<openkey_crypto::keys::AttestationPublicKey, openkey_crypto::keys::KeyError> {
            Err(openkey_crypto::keys::KeyError::Unsupported)
        }

        fn is_available(&self) -> bool {
            true
        }
    }

    fn test_device() -> DeviceProfile {
        DeviceProfile::new(
            DeviceText::from_static("device-1"),
            UsbIdentity {
                vid: 1,
                pid: 2,
                serial_number: DeviceText::from_static("usb-1"),
                product_name: DeviceText::from_static("OpenKey"),
                manufacturer_name: DeviceText::from_static("OpenKey"),
            },
            None,
            None,
        )
    }

    #[test]
    fn test_aaguid_derivation() {
        let board_id = BoardProfileId(*b"openkey-rp23xx01");
        let aaguid = Aaguid::from_board_id(&board_id);
        assert_ne!(aaguid, Aaguid::empty());

        // AAGUID deve ser determinístico
        let aaguid2 = Aaguid::from_board_id(&board_id);
        assert_eq!(aaguid, aaguid2);
    }

    #[test]
    fn test_device_identity_creation() {
        let board_id = BoardProfileId(*b"openkey-rp23xx01");
        let device = test_device();
        let identity = DeviceIdentity::new(
            &board_id,
            device.clone(),
            DeviceProvisioningState::Provisioned,
            AttestationAlgorithm::P256,
        );

        assert_eq!(identity.state(), DeviceProvisioningState::Provisioned);
        assert!(identity.is_provisioned());
        assert_eq!(identity.attestation_algorithm(), AttestationAlgorithm::P256);
        assert_ne!(identity.aaguid(), &Aaguid::empty());
    }

    #[test]
    fn test_device_identity_load() {
        let board_id = BoardProfileId(*b"openkey-rp23xx01");
        let device = test_device();
        let otp = MockOtp;
        let key_provider = MockKeyProvider;

        let identity = DeviceIdentity::load(&board_id, device, &otp, &key_provider).unwrap();
        assert!(identity.is_provisioned());
    }

    #[test]
    fn test_validate_identity() {
        let board_id = BoardProfileId(*b"openkey-rp23xx01");
        let device = test_device();
        let identity = DeviceIdentity::new(
            &board_id,
            device,
            DeviceProvisioningState::Provisioned,
            AttestationAlgorithm::P256,
        );

        assert!(validate_identity(&identity).is_ok());
    }

    #[test]
    fn test_validate_identity_unprovisioned() {
        let board_id = BoardProfileId(*b"openkey-rp23xx01");
        let device = test_device();
        let identity = DeviceIdentity::new(
            &board_id,
            device,
            DeviceProvisioningState::Unprovisioned,
            AttestationAlgorithm::P256,
        );

        assert_eq!(
            validate_identity(&identity),
            Err(IdentityError::InvalidIdentity)
        );
    }
}
