# Arquitetura

O monorepo separa domínio, criptografia, vault, protocolos, transporte USB e atualização. Crates de protocolo não dependem de um MCU específico; targets futuros implementam os contratos de transporte e armazenamento.

Fluxo inicial: `openkey-core` → protocolos FIDO/APDU → USB/vault/update → target de placa.
