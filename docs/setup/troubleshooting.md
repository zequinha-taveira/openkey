# Solução de Problemas e Diagnósticos (`docs/setup/troubleshooting.md`)

## 📌 Objetivo

Fornecer procedimentos passo-a-passo para identificar, diagnosticar e resolver problemas comuns relacionados a permissões de USB, falhas de enumeração, comunicação CTAPHID e execução do Simulador de Software.

---

## 🚨 1. Problemas no Linux

### Erro: `Permission Denied` ao Acessar `/dev/hidraw*`

#### Causa
O usuário atual não possui permissões de leitura/escrita no nó de dispositivo `/dev/hidraw` do OpenKey.

#### Solução
1. Verifique se o seu usuário pertence ao grupo `plugdev`:
   ```bash
   groups $USER
   ```
2. Caso não pertença, adicione o usuário ao grupo:
   ```bash
   sudo usermod -aG plugdev $USER
   ```
3. Instale a regra udev oficial em `/etc/udev/rules.d/70-openkey.rules`:
   ```udev
   KERNEL=="hidraw*", SUBSYSTEM=="hidraw", ATTRS{idVendor}=="1209", ATTRS{idProduct}=="0001", MODE="0660", GROUP="plugdev", TAG+="uaccess"
   ```
4. Recarreague as regras udev e reconecte a chave USB:
   ```bash
   sudo udevadm control --reload-rules && sudo udevadm trigger
   ```

---

### Erro: `pcsc_scan` não detecta o leitor CCID

#### Solução
1. Certifique-se de que o daemon `pcscd` está em execução:
   ```bash
   sudo systemctl status pcscd
   ```
2. Caso o daemon esteja inativo ou travado, reinicie-o:
   ```bash
   sudo systemctl restart pcscd
   ```

---

## 🚨 2. Problemas no Windows

### Abertura do Dispositivo Bloqueada por Aplicações Exclusivas

#### Causa
A API nativa `Windows WebAuthn` restringe o acesso direto via `hidapi` a dispositivos FIDO HID para processos que não estejam sendo executados como Administrador ou integrados à API oficial do Windows.

#### Solução
- Para executar scripts Python ou utilitários CLI que leiam relatórios HID brutos diretamente no Windows durante o desenvolvimento, execute o prompt de comando ou PowerShell com permissões de **Administrador**.

---

## 🚨 3. Problemas com o Simulador de Software

### Erro: Socket Local de Comunicação não Encontrado

#### Causa
O Simulador de Software (`targets/simulator`) não está em execução ou o caminho do soquete IPC temporário não pôde ser criado.

#### Solução
1. Certifique-se de que o simulador está rodando em um terminal separado:
   ```bash
   cargo run --package openkey-simulator
   ```
2. No Linux/macOS, verifique se o arquivo de soquete `/tmp/openkey-sim.sock` existe e possui permissões de escrita.

---

## 🔍 4. Ferramentas Globais de Diagnóstico

### Checagem de Conexão FIDO2 via `fido2-token`
Para verificar se a pilha CTAP2 está respondendo corretamente via `libfido2`:

```bash
# Listar dispositivos FIDO2 conectados
fido2-token -L

# Solicitante de informações do autenticador (GetInfo)
fido2-token -I /dev/hidrawX
```
