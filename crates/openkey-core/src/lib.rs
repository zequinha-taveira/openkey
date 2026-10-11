#![no_std]

/// Estado de execução do autenticador.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthenticatorState {
    Uninitialized,
    Ready,
    Locked,
}

impl AuthenticatorState {
    pub const fn is_ready(self) -> bool {
        matches!(self, Self::Ready)
    }
}

/// Erros de domínio que não dependem de transporte ou MCU.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreError {
    NotReady,
    Locked,
    InvalidState,
}

/// Máquina de estados mínima para ser usada por targets e pelo simulador.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Authenticator {
    state: AuthenticatorState,
}

impl Authenticator {
    pub const fn new() -> Self {
        Self {
            state: AuthenticatorState::Uninitialized,
        }
    }

    pub const fn state(self) -> AuthenticatorState {
        self.state
    }

    pub fn provision(&mut self) -> Result<(), CoreError> {
        if !matches!(self.state, AuthenticatorState::Uninitialized) {
            return Err(CoreError::InvalidState);
        }
        self.state = AuthenticatorState::Ready;
        Ok(())
    }

    pub fn lock(&mut self) {
        self.state = AuthenticatorState::Locked;
    }
}

impl Default for Authenticator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{Authenticator, AuthenticatorState, CoreError};

    #[test]
    fn provisiona_apenas_uma_vez() {
        let mut authenticator = Authenticator::new();
        assert_eq!(authenticator.provision(), Ok(()));
        assert_eq!(authenticator.state(), AuthenticatorState::Ready);
        assert_eq!(authenticator.provision(), Err(CoreError::InvalidState));
    }

    #[test]
    fn pode_ser_bloqueado() {
        let mut authenticator = Authenticator::new();
        authenticator.lock();
        assert_eq!(authenticator.state(), AuthenticatorState::Locked);
    }
}
