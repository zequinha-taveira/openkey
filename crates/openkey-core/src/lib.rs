#![no_std]

//! Máquina de estados e políticas independentes de hardware.

use openkey_vault::KeyId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum State {
    Locked,
    Ready,
    Authenticated,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoreError {
    InvalidState,
    VaultUnavailable,
}

#[derive(Debug)]
pub struct Authenticator {
    state: State,
    active_key: Option<KeyId>,
}

impl Authenticator {
    pub const fn new() -> Self {
        Self {
            state: State::Locked,
            active_key: None,
        }
    }

    pub const fn state(&self) -> State {
        self.state
    }

    pub fn unlock(&mut self) -> Result<(), CoreError> {
        if self.state != State::Locked {
            return Err(CoreError::InvalidState);
        }
        self.state = State::Ready;
        Ok(())
    }

    pub fn select_key(&mut self, key: KeyId) -> Result<(), CoreError> {
        if self.state != State::Ready {
            return Err(CoreError::InvalidState);
        }
        self.active_key = Some(key);
        self.state = State::Authenticated;
        Ok(())
    }

    pub fn reset(&mut self) {
        self.state = State::Locked;
        self.active_key = None;
    }
}

impl Default for Authenticator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fluxo_exige_ordem_segura() {
        let mut auth = Authenticator::new();
        assert_eq!(auth.select_key(KeyId::new(1)), Err(CoreError::InvalidState));
        auth.unlock().unwrap();
        auth.select_key(KeyId::new(1)).unwrap();
        assert_eq!(auth.state(), State::Authenticated);
        auth.reset();
        assert_eq!(auth.state(), State::Locked);
    }
}
