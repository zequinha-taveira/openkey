# Primeira Chave de Segurança

## Passos

1. **Provisionar o dispositivo** (ver `first-provisioning.md`)

2. **Criar credencial WebAuthn**
   ```bash
   openkey-cli make-credential --rp-id example.com
   ```

3. **Autenticar**
   ```bash
   openkey-cli get-assertion --rp-id example.com
   ```

4. **Verificar no navegador**
   - Abrir https://example.com/login
   - Selecionar chave de segurança
   - Confirmar presença do usuário