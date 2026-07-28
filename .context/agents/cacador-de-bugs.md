---
type: agent
name: Cacador de Bugs
description: Busca ativamente problemas e bugs nos códigos do projeto, analisando padrões, anti-padrões e vulnerabilidades
agentType: cacador-de-bugs
phases: [R, E, V]
generated: 2026-07-28
status: unfilled
scaffoldVersion: "2.0.0"
---

## Available Skills

The following skills provide detailed procedures for specific tasks. Activate them when needed:

| Skill | Description |
|-------|-------------|
| [bug-hunting](./../skills/bug-hunting/SKILL.md) | Hunt for bugs and problems in code systematically. Use when Searching for bugs proactively, Auditing code for correctness, or Finding potential issues before they manifest |
| [bug-investigation](./../skills/bug-investigation/SKILL.md) | Investigate bugs systematically and perform root cause analysis. Use when Investigating reported bugs, Diagnosing unexpected behavior, or Finding the root cause of issues |
| [code-review](./../skills/code-review/SKILL.md) | Review code quality, patterns, and best practices. Use when Reviewing code changes for quality, Checking adherence to coding standards, or Identifying potential bugs or issues |
| [security-audit](./../skills/security-audit/SKILL.md) | Review code and infrastructure for security weaknesses. Use when Reviewing code for security vulnerabilities, Assessing authentication/authorization, or Checking for OWASP top 10 issues |

## Focus Areas

This agent specializes in finding bugs in the OpenKey codebase, with particular attention to:

- **Firmware (Rust, `no_std`)**: Memory safety issues, bounds checks, `unsafe` block violations, panic paths, integer overflow/underflow, uninitialized memory
- **Protocol parsing (CBOR)**: Malformed input handling, buffer overflows, deserialization vulnerabilities, missing validation
- **Cryptographic operations**: Side-channel vulnerabilities, timing attacks, improper key handling, weak randomness
- **Host applications (Rust/CLI, Python SDK)**: Error handling gaps, resource leaks, input validation failures
- **Concurrency issues**: Race conditions, deadlocks, improper synchronization
- **State management**: Inconsistent state transitions, missing error states, resource cleanup

## Project-Specific Constraints

See [AGENTS.md](../../AGENTS.md) for global rules. Key constraints:

- Bounds checks must never be ignored in firmware crypto/protocol code
- Every `unsafe` block requires a `// SAFETY:` justification
- No `panic!`, `unwrap()`, or `expect()` in production firmware paths
- All public API/protocol changes require documentation updates