# Política de Segurança (SECURITY.md)

A equipe do OpenKey leva a segurança do firmware e dos componentes do ecossistema com extrema gravidade. Agradecemos à comunidade de pesquisa de segurança por relatar vulnerabilidades de forma responsável.

## 🛡️ Versões Suportadas

Apenas as seguintes versões recebem atualizações de segurança e patches ativos:

| Versão | Suportada |
| ------ | --------- |
| 1.x.x  | ✅ Sim    |
| < 1.0  | ❌ Não    |

## 📩 Relando uma Vulnerabilidade

**NÃO crie um Issue público no GitHub para reportar uma vulnerabilidade de segurança.**

Se você descobriu uma falha de segurança no firmware do OpenKey, SDKs, parsers CBOR ou no simulador, envie um e-mail criptografado para:

- **E-mail de Segurança**: `security@openkey.org`
- **Chave PGP**: `0x123456789ABCDEF0` (disponível em servidores de chaves públicos)

### Informações Recomendadas no Relatório

- Tipo de vulnerabilidade (ex: estouro de buffer, falha de canal lateral, bypass de validação de PIN, vazamento de credencial).
- Componente afetado (`firmware/`, `host/sdk`, `fuzz/`, `host/simulator`).
- Passos detalhados para reprodução ou Proof-of-Concept (PoC).
- Impacto potencial de exploração (local, USB HID físico, ataque de proximidade NFC).

## ⏱️ Processo e Prazos de Resposta

1. **Confirmação**: Responderemos ao seu e-mail em até **48 horas** confirmando o recebimento do relatório.
2. **Avaliação**: Avaliaremos o impacto e triagem em até **7 dias úteis**.
3. **Divulgação Coordenada**: Trabalharemos com o relator para definir uma janela de divulgação coordenada (normalmente 90 dias após a confirmação da correção).

Para obter detalhes completos sobre o fluxo de gerenciamento de CVEs e divulgações, consulte [`docs/security/vulnerability-management.md`](docs/security/vulnerability-management.md).
