# CLI (openkey-cli)

## Instalação

```bash
cargo install openkey-cli
```

## Comandos

```bash
# Informações do dispositivo
openkey-cli info

# Gerenciar PIN
openkey-cli pin set
openkey-cli pin change

# Gerenciar credenciais
openkey-cli credentials list
openkey-cli credentials delete <id>

# Reset
openkey-cli reset

# Atualizar firmware
openkey-cli update <firmware.bin>

# Diagnósticos
openkey-cli diagnostics
```

## Opções

- `--device` - Seleciona dispositivo específico
- `--verbose` - Saída verbosa
- `--json` - Saída em JSON