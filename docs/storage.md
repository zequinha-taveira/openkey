# Armazenamento do OpenKey

## 📦 Modelo de Persistência

O OpenKey persiste os seguintes dados na memória não-volátil através da PAL:

1. **Configuração do Dispositivo**: AAGUID, chaves de atestação e flags de estado.
2. **Hash de PIN & Salt**: Derivação criptográfica protegida.
3. **Contador Monotônico Global**: Inteiro de 32 bits incrementado atomicamente.
4. **Credenciais Residentes (RK)**: Tabela de credenciais contendo RP ID Hash, User ID, Credential ID e Chave Privada associada.

## 🔒 Regras de Persistência

- **Dados sensíveis efémeros** (`pinUvAuthToken`, chaves ECDH de sessão, buffers de pacotes USB) **JAMAIS** são gravados na memória não-volátil.
- **Wear-leveling** é implementado para preservar a vida útil da Flash.
- **Power-loss recovery** garante integridade dos dados.

## 📄 Componentes

- **Storage Manager** (`storage/`): Gerenciamento de armazenamento persistente
- **Configuration Manager** (`platform/src/config.rs`): Gerencia Board Profile, Device Profile e Application Configuration
- **Flash Storage Provider** (`platform/src/hal/flash.rs`): Interface de leitura/escrita/erase da Flash

## 📖 Documentação Detalhada

Veja também: [docs/architecture/storage.md](architecture/storage.md)