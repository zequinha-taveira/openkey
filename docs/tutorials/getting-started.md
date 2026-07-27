# Primeiros Passos

## 1. Instalar Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 2. Clonar o Repositório

```bash
git clone https://github.com/openkey/openkey.git
cd openkey
```

## 3. Build

```bash
cargo build --workspace
```

## 4. Testar

```bash
cargo test --workspace
```

## Próximos Passos

- [Primeiro Build](tutorials/first-build.md)
- [Primeiro Provisionamento](tutorials/first-provisioning.md)
- [Primeira Chave de Segurança](tutorials/first-security-key.md)