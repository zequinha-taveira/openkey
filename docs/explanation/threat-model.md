# Modelo de Ameaças

## Ameaças Identificadas

| Categoria | Ameaça | Defesa |
|-----------|--------|--------|
| Spoofing | Engenharia social | Botão físico dedicado |
| Tampering | Modificação firmware | Assinaturas digitais |
| Repudiation | Negação de operações | Contador de assinatura |
| Information Disclosure | Exfiltração de chaves | Armazenamento seguro |
| Denial of Service | Bloqueio do dispositivo | Watchdog, timeout PIN |
| Elevation of Privilege | Escalonamento | Políticas de segurança |

## Defesas Implementadas

- TRNG para geração de números aleatórios
- Watchdog para detecção de travamentos
- Contador monotônico para prevenção de replay
- Timeout de PIN com bloqueio temporário