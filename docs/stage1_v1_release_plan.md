# Stage 1 v1.0 Release Plan - First Orbital Insertion

## Definition of done

The v1.0 release is done only when all of the following are true:

- `scripts/run_release_gates.sh` prints `release_gate=PASS` on a clean Rust-equipped runner.
- Rust has zero warnings under `cargo clippy --all-targets --all-features -- -D warnings`.
- Rust tests, TypeScript tests, vector validation, negative corpus validation, formal static scan, and Builder Scaffold validation pass.
- Negative corpus contains at least 30 committed fixtures with expected first-failure gates and canon article links.
- Golden corpus contains at least 8 committed vectors: Exact, Stutter x2, SafeRefined x2, governance update, recovery, memory compression, and long stutter chain.
- Rust, TypeScript, and Python agree on every fixture decision and every first-failure gate.
- Fuzz/proptest runs for at least 1 hour with retained seeds and zero failures.
- STRIDE review and mitigation matrix are complete.
- TCB ledger lists each crate/version with justification and replacement plan.
- Tag CI publishes release artifacts and evidence logs automatically.

## Reviewer promise

A reviewer should be able to verify the core artifact claim in under five minutes with either Docker or `scripts/verify_in_5_minutes.sh`.
