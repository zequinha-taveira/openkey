//! Tipos de erro strongly-typed para o OpenKey Core

/// Erros gerais do núcleo de execução
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreError {
    /// Falha no transporte de comunicação
    TransportError,
    /// Erro ao acessar o provedor de aleatoriedade (TRNG/RNG)
    RngFailure,
    /// Falha na operação do armazenamento persistente
    StorageFailure,
    /// Presença de usuário não detectada dentro do tempo limite
    UserPresenceTimeout,
    /// Erro de enquadramento ou parsing do protocolo
    ProtocolError,
}
