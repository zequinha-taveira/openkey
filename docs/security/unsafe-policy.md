# Política de Código `unsafe`

O firmware OpenKey segue o ADR-0004: não é permitido código `unsafe` arbitrário.
Todo bloco `unsafe` deve ser indispensável, encapsulado por uma API segura,
acompanhado de comentário `// SAFETY:` que descreva as invariantes e revisado
formalmente. Alterações que introduzam `unsafe` exigem ADR ou atualização do ADR
aplicável antes de serem aceitas.
