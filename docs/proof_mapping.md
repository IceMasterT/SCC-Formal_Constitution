# Canon-to-Test Proof Mapping

| Canon item | Executable obligation | Rust/TS gate | Corpus evidence |
|---|---|---|---|
| F9 Proof-Obligation Non-Optionality | No valid bundle, no lawful step | `POBundle::all_required_true`, `accepts` | `missing_required_field.json` |
| C11 Action Admissibility | Schema, hashes, domain, audit, witnesses must all hold | `checker::accepts` | all golden and negative vectors |
| Article 29 Governance State | Governance weights remain in simplex | `valid_simplex`, `governance_invariant_holds` | `invalid_simplex.json`, proptest projection |
| Article 30 Risk Law | Risk threshold forces halt | `risk_law_holds` | `risk_threshold_without_halt.json` |
| Article 32 Halt Absorption | Halt cannot silently revert | `halt_absorption_holds` | `halt_absorption_violation.json`, proptest halt |
| Article 35 Audit Determinism | Audit hash chain is deterministic | `audit_chain_valid`, `replay` | `audit_event_hash_bad.json`, `replay.test.ts` |
| Article 36 Lineage Continuation | Lineage must extend lawfully | `expected_lineage_root` | `lineage_erasure.json` |
| Articles 52-53 Normalized simulation | Step modes are Exact/Stutter/SafeRefined | `classify_step` | `valid_exact`, `valid_stutter`, `valid_safe_refined`, `wrong_step_mode.json` |
| Article 73 TCB Ledger | Trusted assumptions are explicit | `docs/tcb_ledger.md`, release manifest | `release_manifest.json` |
| Article 80 External Assumptions | Crypto, serialization, checker soundness remain named | TCB ledger and limitations | docs and paper limitations |
