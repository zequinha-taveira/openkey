# ADR-0002: Armazenamento Seguro e Wear-Leveling na Flash

- **Status**: Aceito
- **Data**: 2026-07-24
- **Autores**: Equipe de Firmware e Segurança

## 📌 Contexto

As chaves de segurança FIDO2 precisam armazenar credenciais residentes (Discoverable Credentials) e contadores monotônicos de assinatura na memória Flash interna do microcontrolador. O ciclo de vida limitado de apagamento de páginas de memória Flash exige estratégias para evitar o desgaste prematuro do hardware (*wear-out*).

## 💡 Decisão

Adotaremos um subsistema de armazenamento seguro com algoritmo de **Wear-Leveling circular** e criptografia **AES-256-GCM** para todos os dados mantidos na Flash.

## 🚀 Consequências

### Positivas
- Aumento significativo da vida útil física da chave de segurança.
- Proteção total contra leitura direta do conteúdo da Flash por extração física de hardware.

### Compromissos (Trade-offs)
- Ligeiro aumento na complexidade do driver de Flash e necessidade de compactação de páginas (*garbage collection* interno).
