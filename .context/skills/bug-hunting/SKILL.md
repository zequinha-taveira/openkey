---
type: skill
name: Bug Hunting
description: Hunt for bugs and problems in code systematically. Use when Searching for bugs proactively, Auditing code for correctness, or Finding potential issues before they manifest
skillSlug: bug-hunting
phases: [R, E, V]
generated: 2026-07-28
status: unfilled
scaffoldVersion: "2.0.0"
---
## Workflow

1. **Map the attack surface** — Identify all entry points: API boundaries, protocol parsers, user input handlers, FFI boundaries, file I/O, network interfaces
2. **Trace data flow** — Follow untrusted input from entry point to sink; note every transformation, validation, and trust boundary crossed
3. **Pattern-based scan** — Search for known bug patterns:
   - **Rust**: `unwrap()`, `expect()`, `panic!`, missing `Some`/`None` handling, integer overflow, `unsafe` without `SAFETY` comment, missing bounds checks
   - **CBOR/protocol**: Unbounded allocation, missing length validation, type confusion, trailing data ignored
   - **Crypto**: Non-constant-time comparisons, hardcoded keys, weak RNG, missing domain separation
   - **General**: Resource leaks, error swallowing, TOCTOU, race conditions
4. **Edge case analysis** — For each function, enumerate: empty input, max-length input, malformed input, concurrent access, resource exhaustion
5. **State machine review** — Verify all states are reachable, transitions are valid, error states are handled, state is cleaned up on failure
6. **Cross-reference with AGENTS.md** — Flag violations of project-specific rules (bounds checks, unsafe blocks, panic paths)
7. **Prioritize findings** — Rank by severity: security-critical > crash/data-loss > logic error > code smell

## Examples

**Bug hunting report:**
```
## Bug Hunt: CBOR Protocol Parser

### Entry Point: `parse_command(data: &[u8])`

### Data Flow:
1. Raw bytes → `cbor::from_slice(data)` — no length pre-check
2. Result → `Command::from_cbor()` — missing field validation
3. Command → `execute()` — assumes valid key handle

### Findings:

**CRITICAL: Unbounded allocation in CBOR decode**
- File: `firmware/protocols/src/parser.rs:42`
- `cbor::from_slice` allocates based on input without size limit
- Attacker can send 1MB CBOR with nested arrays → OOM on device
- Fix: Pre-validate data length, use `from_slice_max` with sane limit

**HIGH: Missing bounds check on key handle**
- File: `firmware/protocols/src/parser.rs:67`
- `key_handle[0..4]` accessed without checking length
- Short input → panic → device crash
- Fix: Validate `key_handle.len() >= 4` before indexing

**MEDIUM: Integer overflow in payload length**
- File: `firmware/protocols/src/parser.rs:89`
- `total_len = header.len + payload.len` can overflow
- On overflow, wraps to small value → buffer over-read
- Fix: Use `checked_add`, return error on overflow
```

## Quality Bar

- Always trace from untrusted input to sensitive operations
- Verify every `unsafe` block has a `// SAFETY:` comment
- Check that all `Result` returns are handled (no `.unwrap()` in firmware)
- Confirm bounds checks exist before array/slice access
- Look for error paths that skip cleanup or leave state inconsistent
- Consider resource exhaustion (memory, file handles, crypto operations)
- Flag any deviation from AGENTS.md rules

## Resource Strategy

- Add `scripts/` only when the task is fragile, repetitive, or benefits from deterministic execution (e.g., a grep-based pattern scanner)
- Add `references/` only when details are too large or too variant-specific to keep in `SKILL.md`
- Add `assets/` only for files that will be consumed in the final output
- Keep extra docs out of the skill folder; prefer `SKILL.md` plus only the resources that materially help