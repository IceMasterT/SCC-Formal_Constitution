# Rust contract bridge

The Rust crate is the source-of-truth checker.  The formal bridge maps each public predicate to a proof obligation.

| Rust function | Formal obligation | Stress gate |
| --- | --- | --- |
| `checker::accepts` | accepted transition implies `GateProp` in the formal model | golden vectors, negative corpus, property tests |
| `state::protected_unchanged` | protected-coordinate isolation | negative corpus and proptest |
| `state::halt_absorption_holds` | halt absorption and severity monotonicity | negative corpus and proptest |
| `state::risk_law_holds` | risk threshold forces halt | negative corpus and numerical-stability tests |
| `refinement::expected_lineage_root` | lineage continuation | lineage-erasure fixture and compression stress |
| `governance::project_to_simplex` | finite simplex projection within descriptor tolerance | numerical-stability stress |
| federation quorum tests | quorum intersection under `n = 3f + 1`, `q = 2f + 1` | `federation_quorum.rs` and Coq arithmetic proof |

Verification status in this release:

- Rust execution is validated by `cargo test --all-features` on a Rust-equipped runner.
- TypeScript reference execution is locally validated in the transcript.
- Lean/Coq bridge files are included and checked by the proof-equipped CI job when available.
- No claim is made that the compiler, operating system, SHA-256 primitive, or all SCC canon theorems are mechanically verified by this fragment.
