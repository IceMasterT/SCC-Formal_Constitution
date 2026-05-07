# STRIDE Security Review Matrix

This is the Stage 1 security-review template. A final v1.0 tag must fill the mitigation and evidence columns.

| Threat class | Kernel Fortress concern | Required mitigation | Evidence |
|---|---|---|---|
| Spoofing | forged audit event, fake descriptor, wrong checker version | canonical hashes, descriptor binding, version gate | negative corpus + Rust tests |
| Tampering | mutated protected coordinates, altered proof bundle | protected-coordinate isolation and PO hash gate | negative corpus + fuzz |
| Repudiation | transition accepted without replayable evidence | audit chain and release transcript | replay test + transcript |
| Information disclosure | checker imports or exposes unauthorized runtime channels | forbidden-import scan and small TCB | tools/forbidden_imports.py |
| Denial of service | malformed fixture triggers panic or nontermination | total parse discipline, fuzz, proptest | fuzz/proptest logs |
| Elevation of privilege | product runtime bypasses checker or exits halt | checker-first boundary and halt absorption | halt fixtures + CI |

## Release rule

Every new externally callable feature must add at least one abuse case to this matrix or explicitly justify why the existing threat row covers it.
