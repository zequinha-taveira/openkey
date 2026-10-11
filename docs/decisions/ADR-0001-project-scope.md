# ADR-0001: escopo do MVP

- **Status:** aceito
- **Decisão:** priorizar um autenticador FIDO2/CTAP e manter OpenPGP/APDU e HSM como módulos independentes posteriores.
- **Motivo:** reduzir a superfície inicial e preservar fronteiras de portabilidade e testes no host.
- **Consequência:** o scaffold não implementa ainda um autenticador completo nem certificação.
