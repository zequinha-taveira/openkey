# Submissão ao pid.codes (VID 0x1209, PID 0x4F4B)

## Resumo

O OpenKey registra um par VID/PID gratuito via [pid.codes](https://pid.codes)
para hardware customizado (PCBs que rodam o firmware OpenKey).

- **VID**: `0x1209` (comunidade pid.codes)
- **PID**: `0x4F4B` (hex de "OK" — OpenKey)
- **Repo**: https://github.com/zequinha-taveira/openkey
- **Licença**: MIT / Apache 2.0

## Como submeter

1. Fork o repo [`pidcodes/pidcodes.github.com`](https://github.com/pidcodes/pidcodes.github.com)
2. Copie a estrutura abaixo para o fork:
   - `org/openkey-team/index.md`
   - `VID/1209/4F4B/index.md`
3. Abra um Pull Request para o repo upstream
4. Aguarde review dos mantenedores do pid.codes

## Estrutura de arquivos

```
pidcodes.github.com/
├── org/
│   └── openkey-team/
│       └── index.md          # Perfil da organização
└── VID/
    └── 1209/
        └── 4F4B/
            └── index.md      # Registro do PID
```
