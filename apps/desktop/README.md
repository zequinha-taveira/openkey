# OpenKey Desktop

Aplicação desktop planejada para configuração e diagnóstico.

A GUI deve consumir `openkey-host` e não acessar diretamente USB, flash, credenciais ou protocolos internos. As operações compartilhadas com a CLI são:

- descoberta e informação do dispositivo;
- provisionamento;
- atualização assinada;
- diagnósticos;
- tratamento uniforme de erros.

O backend offline do SDK permite desenvolver a interface sem hardware real. Um adaptador de transporte real será conectado posteriormente, sem alterar a GUI.
