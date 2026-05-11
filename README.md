# SCC Kernel Fortress

<div align="center">

![Build Status](https://github.com/IceMasterT/SCC-Formal_Constitution/actions/workflows/ci.yml/badge.svg)
![Docker](https://img.shields.io/badge/Docker-Enabled-2496ED?logo=docker&logoColor=white)
![License](https://img.shields.io/github/license/IceMasterT/SCC-Formal_Constitution)
![Version](https://img.shields.io/github/v/release/IceMasterT/SCC-Formal_Constitution?label=Version)
![Stars](https://img.shields.io/github/stars/IceMasterT/SCC-Formal_Constitution?style=social)

</div>

<br>

**SCC Kernel Fortress** is an executable evaluation artifact for **runtime-compliance claims in stateful agentic systems**. It checks one candidate transition at a time and returns either acceptance or a named rejection gate. The artifact is intentionally narrow, reproducible, and evidence-first: a transition is not accepted because a model, log, prompt, runtime trace, or explanation says it is acceptable; it is accepted only when the committed state, event, audit, proof-obligation, and execution-descriptor artifacts satisfy the implemented SCC obligations.

This repository is the cleaned source repository for the SCC Kernel Fortress artifact and its companion documentation. It includes a Rust source-of-truth checker, a TypeScript reference checker, golden fixtures, negative fixtures, formal bridge files, reviewer documentation, NeurIPS paper sources, release gates, Docker support, CI workflows, and author/documentation materials.

**Author / named edition:** Ian Farquharson  
**Artifact family:** Synthetic Cognitive Coding (SCC) / Kernel Fortress  
**Primary repository:** <https://github.com/IceMasterT/SCC-Formal_Constitution>  
**Current package posture:** executable artifact package with strict release gates; Stage 0 release readiness is intentionally conservative.

---

## Table of contents

1. [What this repository is](#what-this-repository-is)
2. [What this repository is not](#what-this-repository-is-not)
3. [Core artifact claim](#core-artifact-claim)
4. [Transition acceptance contract](#transition-acceptance-contract)
5. [System model](#system-model)
6. [Repository layout](#repository-layout)
7. [Quickstart](#quickstart)
8. [Full validation and release gates](#full-validation-and-release-gates)
9. [Docker verification path](#docker-verification-path)
10. [Rust checker](#rust-checker)
11. [TypeScript reference checker](#typescript-reference-checker)
12. [Golden vectors](#golden-vectors)
13. [Negative corpus](#negative-corpus)
14. [Canonical encoding and hashing](#canonical-encoding-and-hashing)
15. [Proof-obligation bundles](#proof-obligation-bundles)
16. [Audit-chain determinism](#audit-chain-determinism)
17. [Step classification](#step-classification)
18. [Formal bridge and mechanization status](#formal-bridge-and-mechanization-status)
19. [Federation, numerical stability, and compression hardening](#federation-numerical-stability-and-compression-hardening)
20. [Documentation map](#documentation-map)
21. [Paper and author-identified copies](#paper-and-author-identified-copies)
22. [CI, release workflows, and evidence policy](#ci-release-workflows-and-evidence-policy)
23. [Stage-gate roadmap](#stage-gate-roadmap)
24. [Security and trusted computing base](#security-and-trusted-computing-base)
25. [How to extend the artifact](#how-to-extend-the-artifact)
26. [Troubleshooting](#troubleshooting)
27. [Reviewer reading order](#reviewer-reading-order)
28. [Current limitations](#current-limitations)

---

## What this repository is

SCC Kernel Fortress is a **pre-acceptance transition checker**. It evaluates whether a proposed next state may lawfully enter an accepted state trajectory under a selected subset of Synthetic Cognitive Coding obligations.

The artifact is designed for systems that maintain state across tool calls, memory updates, policy gates, recovery paths, risk signals, audit events, governance metadata, or distributed/federated execution. In those systems, output inspection alone is not enough. A visible response may look acceptable while the runtime has already corrupted lineage, bypassed a halt, mutated protected state, broken audit continuity, invalidated governance bounds, or accepted evidence that does not match committed canonical bytes.

This repository provides:

- a **Rust source-of-truth checker**;
- a **TypeScript reference checker** for auditability and differential inspection;
- committed **golden vectors** for accepted transitions;
- committed **negative fixtures** for named first-failure gates;
- binary canonical vectors for byte-level reproducibility;
- schema carriers for state, event, and proof-obligation data;
- validation tools for vectors, formal artifacts, and forbidden imports;
- release scripts for local and CI validation;
- Docker-based reproducibility support;
- a Lean/Coq bridge for the implemented v1 subset;
- federation, numerical-stability, and memory-lineage hardening materials;
- formal canon and engineering handbook companion materials;
- paper sources and named/author-identified copies for non-anonymous use.

The guiding idea is simple:

> A transition is not part of the accepted execution history until the checker says it is.

---

## What this repository is not

This artifact is deliberately scoped. It should not be overread.

It does **not** claim:

- general AI safety;
- semantic harmlessness of model outputs;
- correctness of every possible SCC implementation;
- correctness of compilers, operating systems, kernels, CPUs, containers, filesystems, or CI providers;
- cryptographic proof of SHA-256 or any other primitive;
- complete theorem mechanization of the entire SCC canon;
- proof that every deployment substrate satisfies the assumptions used by the checker;
- proof that arbitrary product runtime behavior is safe;
- proof that logs, screenshots, telemetry, or model explanations are trustworthy;
- proof that all possible state schemas are correct;
- proof that future extensions preserve soundness unless they pass the release gates and documentation requirements.

Instead, the claim is intentionally smaller and testable:

> For the implemented v1 subset, the artifact checks selected SCC transition obligations before state acceptance and rejects committed negative fixtures at named first-failure gates.

---

## Core artifact claim

Given:

1. a previous SCC state;
2. a candidate next SCC state;
3. a canonical event;
4. a canonical audit event;
5. a proof-obligation bundle;
6. an execution descriptor/environment;

SCC Kernel Fortress deterministically returns either:

- `Ok`, meaning the candidate transition satisfies the implemented acceptance gates; or
- an explicit named rejection gate, meaning the first detected failure belongs to a known validation class.

The artifact is therefore useful as:

- an executable benchmark for transition-legality checking;
- a fixture corpus for differential checker implementations;
- a reproducibility package for reviewer evaluation;
- a foundation for building stricter SCC-governed runtimes;
- a bridge between formal SCC documentation and concrete runtime enforcement.

---

## Transition acceptance contract

The release contract is the single most important invariant in this repository:

```text
accepts(po, prev, next, event, audit_event, env) = Ok
```

Only this result admits a candidate transition. Everything else rejects.

The checker enforces the following classes of obligations:

1. **Schema discipline** — state, event, audit event, and proof-obligation records must match expected schema/version fields.
2. **Checker-version discipline** — the proof-obligation bundle must target the active checker version.
3. **Required proof-obligation booleans** — required boolean claims must be true and must participate in the committed bundle.
4. **Canonical hash agreement** — previous state, next state, event, audit event, and proof-obligation hashes must recompute exactly.
5. **Admissible state domain** — governance vectors, risk vectors, failure modes, and bounded fields must remain in their declared domains.
6. **Protected-coordinate isolation** — protected roots/coordinates cannot be mutated by ordinary transitions.
7. **Halt absorption** — halted states remain halted unless a valid recovery path is explicitly modeled.
8. **Severity monotonicity** — failure-mode weakening is rejected when it violates the implemented law.
9. **Governance simplex validity** — governance weights must remain a valid simplex vector within descriptor tolerance.
10. **Governance drift bounds** — ordinary governance updates must stay within configured drift bounds.
11. **Risk-threshold-to-halt law** — risk above the threshold must imply halt escalation.
12. **Lineage continuation** — state lineage must extend rather than erase accepted history.
13. **Audit-chain determinism** — audit events and installed audit hashes must match deterministic recomputation.
14. **Step classification** — accepted transitions must classify as `Exact`, `Stutter`, or `SafeRefined`.

---

## System model

SCC Kernel Fortress treats a runtime transition as a structured evidence problem.

The checker does not ask whether an output is persuasive. It asks whether the proposed state change is admissible under the implemented rules.

### Inputs

The acceptance function consumes:

- `prev`: previous accepted SCC state;
- `next`: candidate next SCC state;
- `event`: canonical event describing the input/update being applied;
- `audit_event`: audit event that should deterministically bind the transition;
- `po`: proof-obligation bundle carrying committed evidence fields;
- `env`: execution descriptor containing schema IDs, checker versions, numeric tolerances, drift bounds, risk thresholds, and descriptor hashes.

### Outputs

The checker returns:

- `Ok`, if all gates pass; or
- a named error string, if a gate fails.

### Why named rejection gates matter

Named gates are reviewer-facing evidence. They allow the corpus to specify not only that a fixture must fail, but **where it must fail first**. This protects against weak checkers that reject for the wrong reason, accept after a bypass, or accidentally change gate ordering in a way that hides the intended obligation.

---

## Repository layout

```text
.
├── .github/workflows/
│   ├── ci.yml                         # CI validation workflow
│   └── release.yml                    # Release artifact workflow
├── docs/
│   ├── canonical_encoding.md          # Binary canonical encoding contract
│   ├── checker_walkthrough.md         # Gate-by-gate checker explanation
│   ├── ci_evidence_lock.md            # CI evidence policy
│   ├── federation_evidence_protocol.md
│   ├── final_countdown.md             # Pre-push/release checklist
│   ├── fixture_gate_map.md            # Negative fixture -> first-failure gate map
│   ├── local_validation_transcript.txt
│   ├── mechanization_bridge.md        # Bool-to-Prop / proof bridge notes
│   ├── memory_lineage_compression_stress.md
│   ├── numerical_stability_tcb.md
│   ├── orbital_launch_checklist.md
│   ├── proof_mapping.md
│   ├── release_manifest_signing.md
│   ├── reproducibility_card.md
│   ├── reviewer_quickstart.md
│   ├── sandbox_validation.txt
│   ├── scaffold_migration.md
│   ├── SCC_Engineering_Handbook_Kernel_Fortress_MECH_10_10.*
│   ├── SCC_Formal_Canon_Kernel_Fortress_MECH_10_10.*
│   ├── SCC_SYSTEM_OVERVIEW_ONE_PAGER.md
│   ├── SCC_System_Overview_OnePager_MECH_10_10.*
│   ├── stage_gate_matrix.csv
│   ├── stage1_v1_release_plan.md
│   ├── stride_security_review.md
│   ├── tcb_ledger.md
│   ├── validation_report.md
│   └── verify_in_5_minutes.md
├── federation/
│   ├── manifest.json
│   └── cases/                         # Federation/quorum fixtures
├── formal/
│   ├── coq/SCCKernelFortress.v
│   ├── lean/lakefile.lean
│   ├── lean/lean-toolchain
│   ├── lean/SCC/KernelFortress/*.lean
│   └── rust-contracts/README.md
├── golden/
│   ├── manifest.json
│   ├── valid_exact.json
│   ├── valid_stutter.json
│   ├── valid_safe_refined.json
│   └── bin/                            # Binary canonical vectors
├── negative_corpus/
│   ├── manifest.json
│   └── cases/*.json                    # Rejection fixtures
├── paper/
│   ├── main.tex                        # NeurIPS-style paper source
│   ├── main.pdf                        # Compiled paper
│   ├── checklist.tex
│   ├── checklist_filled.tex
│   ├── neurips_2026.sty
│   ├── references.bib
│   └── Ian_Farquharson_*               # Author-identified copies where applicable
├── releases/                           # Local release artifacts; gitignored
├── rust/
│   ├── Cargo.toml
│   ├── Cargo.lock
│   ├── rust-toolchain.toml
│   ├── src/*.rs
│   ├── tests/*.rs
│   └── fuzz/fuzz_targets/*.rs
├── schemas/
│   ├── event.schema.json
│   ├── po.schema.json
│   └── state.schema.json
├── scripts/
│   ├── collect_ci_evidence.sh
│   ├── run_formal_gates.sh
│   ├── run_release_gates.sh
│   ├── stage0_prelaunch_gate.sh
│   └── verify_in_5_minutes.sh
├── tools/
│   ├── forbidden_imports.py
│   ├── make_release_manifest.py
│   ├── validate_formal_artifacts.py
│   └── validate_vectors.py
├── ts/
│   ├── package.json
│   ├── pnpm-lock.yaml
│   ├── src/index.ts
│   └── test/*.ts
├── Dockerfile
├── Makefile
├── mechanization_manifest.json
├── release_manifest.json
└── README.md
```

---

## Quickstart

The quickest non-Rust inspection path validates fixture consistency, forbidden imports, and the TypeScript reference suite:

```bash
python3 tools/validate_vectors.py
python3 tools/forbidden_imports.py
cd ts && npm test
```

Expected high-level result:

```text
validate_vectors: OK
forbidden_imports: OK
TypeScript tests pass
```

This path is useful when a reviewer wants fast confidence without compiling Rust. It is **not** the full release claim. The full artifact claim is locked by the Rust-equipped release gate or CI workflow.

---

## Full validation and release gates

Run the full release gate on a machine with:

- Python 3;
- Node 22+;
- Rust 1.78+ or compatible toolchain;
- standard POSIX shell utilities.

Command:

```bash
bash scripts/run_release_gates.sh
```

Equivalent Make target:

```bash
make test
```

The release gate is expected to cover:

```bash
python3 tools/validate_vectors.py
python3 tools/forbidden_imports.py
cd rust && cargo test --all-features
cd rust && cargo test --test negative_corpus
cd rust && cargo test --test golden_vectors
cd rust && cargo test --test proptest_invariants
cd rust && cargo test --test memory_lineage_compression
cd rust && cargo test --test numerical_stability
cd rust && cargo test --test federation_quorum
cd ts && npm test
bash scripts/run_formal_gates.sh
```

Useful Make targets:

```bash
make quick          # vector validation + forbidden imports + TS tests + formal static gate
make rust-test      # Rust test suite
make ts-test        # TypeScript reference test suite
make validate       # vector validation
make forbidden      # forbidden import scan
make formal         # formal artifact static checks
make paper          # build paper through latexmk, if installed
make manifest       # regenerate release manifest
make ci-evidence    # collect CI-style evidence bundle
make docker-build   # build reproducibility container
make docker-test    # run release gates in container
make clean          # remove generated local validation/build artifacts
```

---

## Docker verification path

For clean-run review, build and run the CPU-only container:

```bash
docker build -t scc-kernel-fortress .
docker run --rm scc-kernel-fortress
```

Docker is useful because it reduces host-machine drift, but it does not prove Docker, the host kernel, the container runtime, the compiler toolchain, or the operating system correct. Those remain trusted computing base assumptions.

---

## Rust checker

The Rust crate is the source-of-truth implementation.

Important paths:

- `rust/src/checker.rs` — central acceptance gate;
- `rust/src/canonical.rs` — binary canonical encoding support;
- `rust/src/hash.rs` — hash utility layer;
- `rust/src/state.rs` — SCC state representation;
- `rust/src/events.rs` — event representation;
- `rust/src/audit.rs` — audit event and audit-chain logic;
- `rust/src/po.rs` — proof-obligation bundle representation;
- `rust/src/governance.rs` — governance simplex and drift rules;
- `rust/src/refinement.rs` — `Exact`, `Stutter`, `SafeRefined` classification;
- `rust/src/replay.rs` — replay continuity support;
- `rust/tests/negative_corpus.rs` — negative fixture gate tests;
- `rust/tests/golden_vectors.rs` — accepted vector tests;
- `rust/tests/proptest_invariants.rs` — property-test scaffolding;
- `rust/tests/memory_lineage_compression.rs` — compression/lineage tests;
- `rust/tests/numerical_stability.rs` — numerical tolerance tests;
- `rust/tests/federation_quorum.rs` — federation/quorum checks;
- `rust/fuzz/fuzz_targets/` — fuzz target scaffolding.

Run Rust tests:

```bash
cd rust
cargo test --all-features
cargo test --test negative_corpus
cargo test --test golden_vectors
cargo test --test proptest_invariants
cargo test --test memory_lineage_compression
cargo test --test numerical_stability
cargo test --test federation_quorum
```

---

## TypeScript reference checker

The TypeScript implementation is a reference/audit checker. It is useful for:

- independent inspection of the gate logic;
- fixture debugging;
- cross-language comparisons;
- reviewer-friendly reading;
- validating golden and negative fixture expectations;
- testing stress cases for federation, numerical handling, and compression.

Important paths:

- `ts/src/index.ts` — TypeScript acceptance and replay implementation;
- `ts/test/golden.test.ts` — positive fixture tests;
- `ts/test/negative_corpus.test.ts` — negative fixture tests;
- `ts/test/replay.test.ts` — replay continuity tests;
- `ts/test/stress_mechanization.test.ts` — stress/mechanization-adjacent tests;
- `ts/test/_util.ts` — test utilities.

Run:

```bash
cd ts
npm install
npm test
```

---

## Golden vectors

Golden vectors are accepted fixtures. They define lawful positive controls.

Current golden JSON carriers:

- `golden/valid_exact.json`
- `golden/valid_stutter.json`
- `golden/valid_safe_refined.json`

Binary canonical vectors live under:

```text
golden/bin/
```

They bind the byte-level encoding contract for:

- previous state;
- next state;
- event;
- audit event;
- proof-obligation bundle.

The vector validator checks that JSON carriers and binary canonical forms agree:

```bash
python3 tools/validate_vectors.py
```

---

## Negative corpus

The negative corpus contains committed rejection fixtures. Each fixture is expected to fail at a named gate.

Current fixture directory:

```text
negative_corpus/cases/
```

Current manifest:

```text
negative_corpus/manifest.json
```

Representative gate classes:

- bad previous-state hash;
- bad next-state hash;
- bad event hash;
- bad audit-event hash;
- checker-version mismatch;
- descriptor mismatch;
- halt absorption violation;
- invalid simplex;
- lineage erasure;
- missing required field;
- protected mutation;
- risk threshold without halt;
- schema mismatch;
- wrong step mode.

The reviewer-facing mapping is documented in:

```text
docs/fixture_gate_map.md
```

Run negative corpus tests:

```bash
cd rust && cargo test --test negative_corpus
cd ../ts && npm test -- negative_corpus
```

---

## Canonical encoding and hashing

Canonical encoding is the byte-level contract. JSON carriers are for human readability; canonical `.bin` vectors are the replayable bytes.

See:

```text
docs/canonical_encoding.md
```

Important principles:

- every encoded object is domain-separated;
- schema/version fields participate in the encoded bytes;
- floating-point fields are deterministic under the declared numerical mode;
- negative zero is normalized where required;
- byte order is fixed;
- map iteration order is stable;
- state body hashing avoids circular audit definitions by zeroing `next.audit_hash` before computing the candidate body hash;
- audit hashes are installed only after deterministic audit-event recomputation.

---

## Proof-obligation bundles

The proof-obligation bundle is the transition evidence carrier. It records the fields the checker must bind before acceptance.

The checker treats the bundle as committed evidence, not as a trusted witness by itself. Bundle fields must match recomputed canonical hashes and required booleans must be present and true.

The bundle is represented in:

- `rust/src/po.rs`
- `schemas/po.schema.json`
- `ts/src/index.ts`

Typical bundle commitments include:

- checker version;
- step ID;
- previous state hash;
- next state hash;
- event hash;
- audit event hash;
- required boolean obligations;
- declared step mode;
- lineage/audit/governance/risk commitments.

---

## Audit-chain determinism

Audit-chain determinism is the rule that the audit event for a transition must be reconstructible from the previous state, candidate next state body, event, proof-obligation bundle, and execution descriptor.

The checker rejects if:

- the audit event hash does not match the proof-obligation bundle;
- the installed next-state audit hash does not match the audit event;
- the audit event contains the wrong previous audit hash;
- the event hash inside the audit event differs from the canonical event hash;
- the descriptor hash or checker version does not bind correctly;
- the next-state body hash is inconsistent.

This makes audit continuity a pre-acceptance condition rather than a post-hoc log convention.

---

## Step classification

Accepted transitions must classify as one of three normalized modes:

### `Exact`

The candidate next state matches the deterministic expected update.

### `Stutter`

The candidate next state is equivalent to the previous normalized state. Stutter is useful for lawful no-op or wait transitions.

### `SafeRefined`

The candidate next state is a safe refinement of the deterministic expected state. It may be more conservative, but must preserve stability conditions and avoid risk-increasing illegal behavior.

If a candidate transition does not classify as any accepted mode, the checker rejects it.

---

## Formal bridge and mechanization status

The formal bridge is intentionally scoped to the implemented v1 subset.

Important paths:

- `formal/lean/SCC/KernelFortress/GateSoundness.lean`
- `formal/lean/SCC/KernelFortress/PipelineWitness.lean`
- `formal/coq/SCCKernelFortress.v`
- `formal/README.md`
- `mechanization_manifest.json`
- `docs/mechanization_bridge.md`

The bridge is designed to show how accepted boolean gates correspond to Prop-level obligations in a small formal model. It does not prove the entire SCC canon, the Rust compiler, the operating system, SHA-256, or all deployment assumptions.

Run the formal static gate:

```bash
python3 tools/validate_formal_artifacts.py
bash scripts/run_formal_gates.sh
```

The validation tool checks for unresolved placeholders such as `sorry`, `admit`, `Admitted`, and unledgered proof gaps.

---

## Federation, numerical stability, and compression hardening

The package includes additional hardening surfaces beyond the minimal single-transition gate.

### Federation

Paths:

- `federation/manifest.json`
- `federation/cases/*.json`
- `docs/federation_evidence_protocol.md`
- `rust/tests/federation_quorum.rs`

The federation fixtures exercise quorum-certificate and cross-shard lineage concepts. They are evidence fixtures, not a full distributed-systems proof.

### Numerical stability

Paths:

- `docs/numerical_stability_tcb.md`
- `rust/tests/numerical_stability.rs`

The v1 numerical mode uses deterministic finite `f64` discipline with explicit tolerance handling. Production-critical deployments should consider fixed-point arithmetic for governance and risk-critical paths.

### Memory lineage compression

Paths:

- `docs/memory_lineage_compression_stress.md`
- `rust/tests/memory_lineage_compression.rs`
- `rust/fuzz/fuzz_targets/memory_lineage.rs`

Compression is acceptable only when lineage remains reconstructible enough for audit and review. Compression that erases evidence is a compliance failure.

---

## Documentation map

Start here:

- `docs/SCC_SYSTEM_OVERVIEW_ONE_PAGER.md` — one-page system overview;
- `docs/reviewer_quickstart.md` — reviewer-first quickstart;
- `docs/checker_walkthrough.md` — gate walkthrough;
- `docs/fixture_gate_map.md` — fixture-to-gate evidence map;
- `docs/reproducibility_card.md` — reproducibility summary;
- `docs/validation_report.md` — validation notes;
- `docs/tcb_ledger.md` — trusted computing base ledger;
- `docs/stride_security_review.md` — STRIDE review;
- `docs/proof_mapping.md` — canon/test/proof correspondence;
- `docs/mechanization_bridge.md` — proof bridge notes;
- `docs/orbital_launch_checklist.md` — launch checklist;
- `docs/stage_gate_matrix.csv` — stage-gate roadmap;
- `docs/final_countdown.md` — final push checklist.

Companion long-form documents:

- `docs/SCC_Formal_Canon_Kernel_Fortress_MECH_10_10.pdf`
- `docs/SCC_Formal_Canon_Kernel_Fortress_MECH_10_10.tex`
- `docs/SCC_Engineering_Handbook_Kernel_Fortress_MECH_10_10.pdf`
- `docs/SCC_Engineering_Handbook_Kernel_Fortress_MECH_10_10.tex`

---

## Paper and author-identified copies

The `paper/` directory contains NeurIPS-style paper sources and checklist material. Some files originated as anonymous or anonymized submission artifacts. Per the current repository cleanup, author-identified copies are kept alongside the original anonymous/submission-form files where applicable.

Primary paper files:

- `paper/main.tex`
- `paper/main.pdf`
- `paper/checklist.tex`
- `paper/checklist_filled.tex`
- `paper/neurips_2026.sty`
- `paper/references.bib`

Author-identified copies use `Ian_Farquharson` in the filename. These are intended for non-anonymous publication, archival, or public-repository use. The anonymous/submission versions are retained where useful so that the repository preserves both review and author-identified forms.

---

## CI, release workflows, and evidence policy

CI workflows live in:

```text
.github/workflows/ci.yml
.github/workflows/release.yml
```

The evidence policy is described in:

```text
docs/ci_evidence_lock.md
docs/release_manifest_signing.md
```

A release should not be considered v1.0-ready unless clean-run evidence includes all of the following:

```text
stage0_prelaunch=PASS
release_gate=PASS
rust_gate=PASS
ts_gate=PASS
python_scaffold=PASS
vectors=PASS
negative_corpus=PASS
differential=PASS
manifest_sha256=PASS
signature=PASS
```

Release claims that lack evidence should be downgraded rather than asserted.

---

## Stage-gate roadmap

The stage-gate roadmap is intentionally strict. See:

- `docs/stage_gate_matrix.csv`
- `docs/orbital_launch_checklist.md`
- `docs/stage1_v1_release_plan.md`

High-level stages:

### Stage 0 — prelaunch evidence

Requires clean validation, manifest integrity, Docker/CI evidence, repository readiness, and release-signing posture.

### Stage 1 — v1 hardening

Requires stronger Rust evidence, expanded negative corpus, expanded golden vectors, differential testing, fuzz/proptest duration, TCB review, STRIDE review, documentation, and tag artifact publication.

### Stage 2 — mechanization and advanced hardening

Targets deeper Lean/Coq mechanization, Prusti/Kani-style Rust verification, numerical stability formalization, federation expansion, and memory-compression proof expansion.

### Stage 3 — external adoption

Targets external certification and independent reproductions.

### Stage 4 — nightly constellation

Targets continuous fuzzing, live governance updates, and integration with broader frameworks.

---

## Security and trusted computing base

The checker is intentionally small and fail-closed, but it still depends on a trusted computing base.

See:

- `docs/tcb_ledger.md`
- `docs/stride_security_review.md`
- `docs/numerical_stability_tcb.md`

TCB surfaces include:

- Rust compiler and standard library;
- TypeScript/Node runtime for reference tests;
- Python runtime for validation scripts;
- SHA-256 implementation;
- canonical serializer implementation;
- JSON parser behavior for carrier files;
- operating system and filesystem behavior;
- CI runner behavior;
- Docker runtime behavior;
- proof-assistant tooling for formal bridge checks;
- reviewer trust in committed source and manifests.

The artifact reduces the trusted boundary by making transition evidence explicit, but it does not eliminate the boundary.

---

## How to extend the artifact

When adding a new obligation, fixture, or gate:

1. Define the obligation in documentation.
2. Add or update schema fields if needed.
3. Extend the Rust checker first.
4. Extend TypeScript reference behavior.
5. Add at least one accepted fixture if the obligation affects lawful behavior.
6. Add at least one negative fixture that fails at the intended first-failure gate.
7. Update `negative_corpus/manifest.json`.
8. Update `docs/fixture_gate_map.md`.
9. Regenerate canonical vectors if byte-level objects changed.
10. Update formal bridge notes if the obligation touches the proved subset.
11. Run the full release gate.
12. Update the release manifest.
13. Downgrade any claim that lacks evidence.

Do not add broad claims without tests, manifests, and reviewer-visible evidence.

---

## Troubleshooting

### `cargo` is unavailable

Install Rust 1.78+ or use the Docker path. The quick non-Rust path can still validate vectors, forbidden imports, and TypeScript tests, but it does not establish the full release claim.

### `npm test` fails because dependencies are missing

Run:

```bash
cd ts
npm install
npm test
```

### Vector validation fails

Check whether JSON carriers, binary `.bin` vectors, or canonical encoding code changed. Regenerate intentionally only when the encoding contract changes, and update the release manifest afterward.

### A negative fixture fails at the wrong gate

Inspect:

- `negative_corpus/manifest.json`
- `docs/fixture_gate_map.md`
- Rust test output;
- TypeScript test output.

Wrong-gate rejection is a real regression unless the gate ordering was intentionally changed and documented.

### Formal gate reports placeholders

Remove or ledger proof placeholders. The formal static gate is intended to prevent accidental release of `sorry`, `admit`, `Admitted`, or untracked axioms.

### Docker passes but local tests fail

Treat this as an environment drift issue until proven otherwise. Compare Rust, Node, Python, and OS/toolchain versions.

---

## Reviewer reading order

Recommended reviewer path:

1. `docs/SCC_SYSTEM_OVERVIEW_ONE_PAGER.md`
2. `README.md`
3. `docs/reviewer_quickstart.md`
4. `docs/checker_walkthrough.md`
5. `docs/fixture_gate_map.md`
6. `docs/canonical_encoding.md`
7. `docs/tcb_ledger.md`
8. `docs/mechanization_bridge.md`
9. `paper/main.pdf`
10. `docs/SCC_Formal_Canon_Kernel_Fortress_MECH_10_10.pdf`
11. `docs/SCC_Engineering_Handbook_Kernel_Fortress_MECH_10_10.pdf`

Short evidence chain:

```text
Canon clause -> Handbook rule -> Kernel Fortress gate -> Fixture -> Release transcript -> Reviewer claim
```

---

## Current limitations

The package is already useful as an executable artifact, but the stage matrix records remaining work before broad release claims should be made.

Known limitations include:

- negative corpus count is currently smaller than the Stage 1 target;
- golden vector count is currently smaller than the Stage 1 target;
- full Rust/Docker/CI evidence must be observed on clean equipped runners for strict release claims;
- release signatures require maintainer key material;
- full differential testing across Rust, TypeScript, and Python scaffolds is not yet a completed release claim;
- one-hour fuzz/proptest evidence is not assumed unless a transcript is attached;
- Lean/Coq bridge covers the implemented v1 subset, not the entire SCC canon;
- federation fixtures are evidence fixtures, not a full distributed fault-tolerance proof;
- numerical stability discipline is documented and tested, but full formalized bounds remain future work.

The project policy is to keep these limitations visible. If evidence is absent, downgrade the claim or attach evidence before release.

---

## Final countdown before pushing or tagging

Before pushing or tagging a release, answer:

```text
Does this change preserve protected isolation?
Does the checker still reject every negative fixture at the named gate?
Would a tired reviewer understand the evidence in one pass?
Is the TCB smaller or better justified?
Are we closer to mechanized soundness?
```

Five yes answers means proceed. Any no means stop, fix, and rerun the relevant gate.
