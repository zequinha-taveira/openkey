# Subsistema de Armazenamento Flash (`docs/architecture/storage.md`)

## 💾 Armazenamento de Credenciais e Wear-Leveling

O OpenKey utiliza a memória Flash interna do microcontrolador organizada em páginas dedicadas para persistência de dados.

## 🔐 Camadas de Armazenamento

1. **Chave Mestra do Dispositivo (Master Encryption Key)**:
   - Armazenada em região de memória protegida por leitura / Flash Read-Out Protection (RDP Level 2).
2. **Tabela de Credenciais Residentes (Discoverable Credentials)**:
   - Chaves privadas salvas criptografadas sob demanda com AES-256-GCM.
   - Algoritmo de Wear-Leveling rotaciona gravações entre páginas Flash para preservar vida útil física.
3. **Contadores Globais (Monotonic Signature Counters)**:
   - Mantidos de forma atômica para impedir ataques de clonagem ou replay.
