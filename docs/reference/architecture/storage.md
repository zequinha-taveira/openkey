# Armazenamento

## Arquitetura

O OpenKey persiste dados na Flash não-volátil com wear-leveling.

## Dados Armazenados

1. **Configuração do Dispositivo** - AAGUID, chaves de atestação
2. **Hash de PIN & Salt** - Derivação criptográfica
3. **Contador Monotônico Global** - Previne replay
4. **Credenciais Residentes (RK)** - Credenciais descobríveis

## Regras

- Dados sensíveis efémeros **NUNCA** são gravados
- Wear-leveling preserva vida útil da Flash
- Power-loss recovery garante integridade

## Componentes

- **Storage Manager** (`storage/`) - Gerenciamento de armazenamento
- **Flash Storage Provider** (`platform/src/hal/flash.rs`) - Interface Flash