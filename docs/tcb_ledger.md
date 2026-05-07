# Trusted Computing Base Ledger - v1.0 Freeze

| TCB item | Trusted for | Boundaries | Evidence |
|---|---|---|---|
| Rust checker crate | Source-of-truth acceptance and rejection | Pure deterministic code under `rust/src`; `#![forbid(unsafe_code)]` | Cargo tests, negative corpus, golden vectors, proptest harness |
| Canonical serializer | Stable bytes for state, event, audit event, PO | Binary domain-separated encoding only | Golden `.bin` vectors, TS byte equality tests |
| SHA-256 implementation | Hash integrity | Uses `sha2` in Rust, Node crypto in TS | Golden cross-language vectors |
| TypeScript reference checker | Audit/reference parity | Not source of authority for production acceptance | Node test suite against the same vectors |
| Compiler/runtime | Executes trusted kernel code | Rust stable, Node 22 for reference tests | CI pinning and release manifest |
| Fuzz harness | Finds parser/checker crashes | Not a proof of soundness | 5-minute CI fuzz smoke, longer local campaigns recommended |

## Non-TCB

Product models, prompt layers, telemetry dashboards, UI routing, wall-clock time, network calls, and mutable storage are not allowed to authorize an SCC step. They may propose candidates only.


## Numerical stability addendum

The numerical mode is part of the TCB.  The v1 artifact uses `deterministic-f64-be-v1`: finite IEEE-754 binary64 only, big-endian canonical bytes, negative-zero normalization, and descriptor-supplied tolerances.  Production deployments must record maximum governance dimension, maximum risk-vector dimension, simplex epsilon, drift bound, and threshold margin.  See `docs/numerical_stability_tcb.md`.

## Mechanization bridge addendum

The formal bridge in `formal/` covers the implemented v1 subset and prohibits unledgered `sorry`, `admit`, `Admitted`, and `axiom` placeholders.  A proof-equipped release gate should run Lean and Coq checks; a sandbox without proof tools may report only the static placeholder scan.  See `docs/mechanization_bridge.md` and `mechanization_manifest.json`.

## Federation evidence addendum

Federation is treated as a certificate layer over single-node transition legality.  Quorum certificates, cross-shard lineage bindings, and halt propagation are separate evidence artifacts.  See `docs/federation_evidence_protocol.md` and `federation/manifest.json`.
