use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostError<E> {
    Transport(E),
    InvalidResponse,
}

pub trait HostTransport {
    type Error;

    fn exchange(&mut self, request: &[u8], response: &mut [u8]) -> Result<usize, Self::Error>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceInfo {
    pub protocol_version: u8,
    pub firmware_version: u32,
    pub provisioned: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Diagnostics {
    pub transport_ok: bool,
    pub storage_ok: bool,
    pub firmware_version: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProvisioningRequest<'a> {
    pub label: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdatePackage<'a> {
    pub version: u32,
    pub image: &'a [u8],
    /// Apenas um contrato de teste; verificação criptográfica real virá com COSE.
    pub signature_valid: bool,
}

pub struct DeviceClient<T> {
    transport: T,
    _sdk: PhantomData<fn()>,
}

impl<T: HostTransport> DeviceClient<T> {
    pub fn new(transport: T) -> Self {
        Self {
            transport,
            _sdk: PhantomData,
        }
    }

    pub fn into_inner(self) -> T {
        self.transport
    }

    pub fn get_info(&mut self) -> Result<DeviceInfo, HostError<T::Error>> {
        let mut response = [0; 7];
        let size = self
            .transport
            .exchange(&[0x04], &mut response)
            .map_err(HostError::Transport)?;
        if size != 7 || response[0] != 0 {
            return Err(HostError::InvalidResponse);
        }
        Ok(DeviceInfo {
            protocol_version: response[1],
            firmware_version: u32::from_be_bytes([
                response[3],
                response[4],
                response[5],
                response[6],
            ]),
            provisioned: response[2] != 0,
        })
    }

    pub fn provision(
        &mut self,
        request: ProvisioningRequest<'_>,
    ) -> Result<(), HostError<T::Error>> {
        if request.label.is_empty() || request.label.len() > 64 {
            return Err(HostError::InvalidResponse);
        }
        let mut response = [0; 1];
        let mut command = [0; 65];
        command[0] = 0x10;
        command[1..=request.label.len()].copy_from_slice(request.label.as_bytes());
        let size = self
            .transport
            .exchange(&command[..=request.label.len()], &mut response)
            .map_err(HostError::Transport)?;
        if size != 1 || response[0] != 0 {
            return Err(HostError::InvalidResponse);
        }
        Ok(())
    }

    pub fn update(&mut self, package: UpdatePackage<'_>) -> Result<(), HostError<T::Error>> {
        if package.image.is_empty() || !package.signature_valid {
            return Err(HostError::InvalidResponse);
        }
        let mut command = [0; 9];
        command[0] = 0x20;
        command[1..5].copy_from_slice(&package.version.to_be_bytes());
        command[5..9].copy_from_slice(&(package.image.len() as u32).to_be_bytes());
        let mut response = [0; 1];
        let size = self
            .transport
            .exchange(&command, &mut response)
            .map_err(HostError::Transport)?;
        if size != 1 || response[0] != 0 {
            return Err(HostError::InvalidResponse);
        }
        Ok(())
    }

    pub fn diagnostics(&mut self) -> Result<Diagnostics, HostError<T::Error>> {
        let mut response = [0; 7];
        let size = self
            .transport
            .exchange(&[0x30], &mut response)
            .map_err(HostError::Transport)?;
        if size != 7 || response[0] != 0 {
            return Err(HostError::InvalidResponse);
        }
        Ok(Diagnostics {
            transport_ok: response[1] != 0,
            storage_ok: response[2] != 0,
            firmware_version: u32::from_be_bytes([
                response[3],
                response[4],
                response[5],
                response[6],
            ]),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineError {
    UnsupportedCommand,
    OutputTooSmall,
    Rollback,
}

#[derive(Debug, Default)]
pub struct OfflineTransport {
    pub firmware_version: u32,
    pub provisioned: bool,
}

impl HostTransport for OfflineTransport {
    type Error = OfflineError;

    fn exchange(&mut self, request: &[u8], response: &mut [u8]) -> Result<usize, Self::Error> {
        let command = *request.first().ok_or(OfflineError::UnsupportedCommand)?;
        match command {
            0x04 => {
                if response.len() < 7 {
                    return Err(OfflineError::OutputTooSmall);
                }
                response[..7].copy_from_slice(&[
                    0,
                    1,
                    u8::from(self.provisioned),
                    (self.firmware_version >> 24) as u8,
                    (self.firmware_version >> 16) as u8,
                    (self.firmware_version >> 8) as u8,
                    self.firmware_version as u8,
                ]);
                Ok(7)
            }
            0x10 => {
                self.provisioned = true;
                response
                    .first_mut()
                    .ok_or(OfflineError::OutputTooSmall)
                    .map(|status| {
                        *status = 0;
                        1
                    })
            }
            0x20 => {
                if request.len() != 9 {
                    return Err(OfflineError::UnsupportedCommand);
                }
                let version = u32::from_be_bytes([request[1], request[2], request[3], request[4]]);
                if version <= self.firmware_version {
                    return Err(OfflineError::Rollback);
                }
                self.firmware_version = version;
                response
                    .first_mut()
                    .ok_or(OfflineError::OutputTooSmall)
                    .map(|status| {
                        *status = 0;
                        1
                    })
            }
            0x30 => {
                if response.len() < 7 {
                    return Err(OfflineError::OutputTooSmall);
                }
                response[..7].copy_from_slice(&[
                    0,
                    1,
                    1,
                    (self.firmware_version >> 24) as u8,
                    (self.firmware_version >> 16) as u8,
                    (self.firmware_version >> 8) as u8,
                    self.firmware_version as u8,
                ]);
                Ok(7)
            }
            _ => Err(OfflineError::UnsupportedCommand),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DeviceClient, HostError, OfflineTransport, ProvisioningRequest, UpdatePackage};

    #[test]
    fn sdk_executa_provisionamento_diagnostico_e_update() {
        let mut client = DeviceClient::new(OfflineTransport::default());
        client
            .provision(ProvisioningRequest { label: "test" })
            .unwrap();
        client
            .update(UpdatePackage {
                version: 2,
                image: b"firmware",
                signature_valid: true,
            })
            .unwrap();
        let diagnostics = client.diagnostics().unwrap();
        assert!(diagnostics.transport_ok && diagnostics.storage_ok);
        assert_eq!(diagnostics.firmware_version, 2);
    }

    #[test]
    fn sdk_rejeita_update_sem_assinatura() {
        let mut client = DeviceClient::new(OfflineTransport::default());
        assert_eq!(
            client.update(UpdatePackage {
                version: 1,
                image: b"firmware",
                signature_valid: false,
            }),
            Err(HostError::InvalidResponse)
        );
    }
}
