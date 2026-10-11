#![no_std]

/// Estado mínimo do autenticador enquanto os contratos são implementados.
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

#[cfg(test)]
mod tests {
    use super::AuthenticatorState;

    #[test]
    fn estado_inicial_nao_esta_pronto() {
        assert!(!AuthenticatorState::Uninitialized.is_ready());
    }
}
