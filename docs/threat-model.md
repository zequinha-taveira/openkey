# Modelo de ameaças

Ameaças iniciais: entrada FIDO/APDU malformada, repetição, downgrade, extração de chaves, falha de RNG, corrupção de armazenamento e transporte USB hostil.

Toda implementação deve validar limites, falhar fechada e evitar expor material secreto em logs.
