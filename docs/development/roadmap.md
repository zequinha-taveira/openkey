# Roadmap do Projeto OpenKey (`docs/development/roadmap.md`)

## 🗺️ Metas e Milestones

### Q1/Q2 — Fase 1: Core FIDO2 & Simulador (Atual)
- [x] Definição de arquitetura e monorepo.
- [ ] Implementação completa da máquina de estados CTAP2.0 no simulador.
- [ ] Suporte a credenciais residentes (Resident Keys) e extensão `hmac-secret`.

### Q3 — Fase 2: Hardware Inicial & Biometria
- [ ] Protótipo físico de hardware baseado no chipset target em `hardware/`.
- [ ] Suporte a CTAP2.1 `authenticatorBioEnrollment` e sensores capacitivos.

### Q4 — Fase 3: Certificação FIDO Alliance & Produção
- [ ] Bateria de testes do FIDO Alliance Conformance Tool.
- [ ] Lote inicial de produção de chaves de hardware.
