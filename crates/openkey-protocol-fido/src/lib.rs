#![no_std]

/// Comandos FIDO previstos para a primeira implementação.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FidoCommand {
    GetInfo,
    MakeCredential,
    GetAssertion,
}
