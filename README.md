# SCC Kernel Fortress v1.0 - anonymized NeurIPS 2026 E&D supplement

This package is a reusable executable evaluation artifact for runtime-compliance claims in stateful agentic systems. It evaluates one candidate transition at a time and returns either acceptance or a named rejection gate.

The artifact is intentionally narrow: it does not claim general AI safety, semantic harmlessness, compiler correctness, operating-system correctness, cryptographic primitive security, or mechanically proven checker soundness. It tests whether selected SCC transition obligations are enforced before state acceptance.

## What ships

- `rust/`: source-of-truth checker crate. `src/checker.rs` contains the single acceptance gate.
- `ts/`: TypeScript reference checker for audits, fixture inspection, and differential testing.
- `golden/`: accepted `Exact`, `Stutter`, and `SafeRefined` golden vectors, with JSON and binary canonical forms.
- `negative_corpus/`: fourteen committed negative fixtures with expected first-failure gates.
- `schemas/`: carrier schemas for state, event, and proof-obligation records.
- `docs/`: canonical encoding, checker walkthrough, fixture-to-gate map, TCB ledger, reproducibility card, CI evidence policy, validation reports, and proof mapping.
- `tools/`: vector validation, forbidden-import scanning, and release-manifest generation.
- `scripts/`: full release-gate runner and CI evidence collector.
- `.github/workflows/ci.yml`: Rust/Node/Python CI workflow that runs the release gate and uploads validation evidence.
- `Dockerfile`: CPU-only reproducibility container for reviewers who prefer a clean runner.
- `paper/`: anonymized NeurIPS 2026 E&D paper source, PDF, style file, and checklist.

## Reviewer quickstart

Full release gate on a machine with Python 3, Node 22+, and Rust 1.78+:

```bash
bash scripts/run_release_gates.sh
```

Equivalent Docker path:

```bash
docker build -t scc-kernel-fortress .
docker run --rm scc-kernel-fortress
```

Quick non-Rust inspection path:

```bash
python3 tools/validate_vectors.py
python3 tools/forbidden_imports.py
cd ts && npm test
```

The quick path validates fixture consistency, forbidden imports, and the TypeScript reference suite. The full artifact claim is release-locked only by the Rust-equipped release gate or the included CI workflow.

## Release contract

A transition is accepted only if:

```text
accepts(po, prev, next, event, audit_event, env) = Ok
```

Logs, comments, unverified witnesses, wall-clock data, random seeds, telemetry, and product runtime output cannot bypass the checker.

The checker enforces:

1. schema and checker-version discipline;
2. all required proof-obligation booleans true;
3. canonical hashes for previous state, next state, event, audit event, and proof-obligation vectors;
4. admissible state domain;
5. protected-coordinate isolation;
6. halt absorption and severity monotonicity;
7. governance simplex and bounded drift;
8. risk-threshold-to-halt law;
9. lineage continuation;
10. audit-chain determinism;
11. `Exact` / `Stutter` / `SafeRefined` normalized step classification.

## Expected passing evidence

A release is v1.0-ready only when these pass:

```bash
python3 tools/validate_vectors.py
python3 tools/forbidden_imports.py
cd rust && cargo test --all-features
cd rust && cargo test --test negative_corpus
cd rust && cargo test --test golden_vectors
cd rust && cargo test --test proptest_invariants
cd ts && npm test
```

The CI evidence lock is described in `docs/ci_evidence_lock.md`. The fixture-to-gate mapping is in `docs/fixture_gate_map.md`. The threat model and evidence policy are in `docs/reproducibility_card.md` and `docs/validation_report.md`.


## Reviewer navigation one-pager

Start with `docs/SCC_SYSTEM_OVERVIEW_ONE_PAGER.md`.  The short map is:

`Canon clause -> Handbook rule -> Kernel Fortress gate -> Fixture -> Release transcript -> Reviewer claim`.

## Mechanization, federation, and numerical hardening

This package now includes a formal bridge and hardening docs:

- `formal/` - Lean/Coq bridge for the implemented v1 subset.
- `mechanization_manifest.json` - proof targets and non-claims.
- `docs/mechanization_bridge.md` - Bool-to-Prop boundary and proof-gate instructions.
- `docs/federation_evidence_protocol.md` and `federation/` - distributed certificate fixtures and quorum-intersection evidence.
- `docs/numerical_stability_tcb.md` - float/fixed-point error-bound discipline.
- `docs/memory_lineage_compression_stress.md` - compression and lineage stress protocol.

Formal static check:

```bash
python3 tools/validate_formal_artifacts.py
bash scripts/run_formal_gates.sh
```

The TypeScript reference suite includes 22 tests after adding compression, numerical, and federation stress cases.

## Orbital launch control

The release roadmap and go/no-go gates are in:

- `docs/orbital_launch_checklist.md`
- `docs/stage_gate_matrix.csv`
- `docs/final_countdown.md`
- `docs/verify_in_5_minutes.md`
- `docs/release_manifest_signing.md`

The intentionally strict Stage 0 gate is:

```bash
bash scripts/stage0_prelaunch_gate.sh
```

A release should not be tagged until Stage 0 prints:

```text
stage0_prelaunch=PASS
```
