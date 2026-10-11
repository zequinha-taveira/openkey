# Arquitetura

O workspace é dividido em camadas: domínio (`openkey-core`), contratos criptográficos e de cofre, protocolos independentes de transporte e adaptadores de hardware. As crates centrais usam `#![no_std]` e não importam HALs, SDKs ou registradores de MCU.

A lógica de CTAP e APDU recebe bytes por APIs próprias; USB é apenas uma implementação de transporte.
