# Implementação CTAP2.0 / CTAP2.1 (`docs/protocols/ctap2.md`)

## 🔌 Especificação CTAP (Client to Authenticator Protocol)

O OpenKey implementa a especificação **FIDO Alliance CTAP 2.0 / 2.1**.

## 📥 Comandos Suportados

| Código de Comando | Nome do Comando CTAP2 | Descrição |
| ----------------- | --------------------- | --------- |
| `0x01` | `authenticatorMakeCredential` | Cria uma nova credencial FIDO2 associada a um RP ID. |
| `0x02` | `authenticatorGetAssertion` | Realiza a autenticação assinando o desafio oferecido pelo RP ID. |
| `0x04` | `authenticatorGetInfo` | Retorna as capacidades, versões e extensões do autenticador. |
| `0x06` | `authenticatorClientPIN` | Gerencia PIN do usuário, obtenção de tokens `pinUvAuthToken`. |
| `0x07` | `authenticatorReset` | Restaura o dispositivo às configurações de fábrica e apaga credenciais. |
| `0x08` | `authenticatorBioEnrollment` | Cadastro e gerenciamento de impressões digitais. |
| `0x09` | `authenticatorCredentialManagement` | Gerenciamento de credenciais residentes (RK) salvas no dispositivo. |

Para detalhes da máquina de estados do protocolo, consulte [`docs/protocols/protocol-state-machine.md`](protocol-state-machine.md).
