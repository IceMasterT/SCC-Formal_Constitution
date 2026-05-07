# SCC Kernel Fortress Orbital Launch Checklist

**Launch doctrine:** legality is a runtime invariant. A system that cannot prove the legality of its next step has no right to take it.

**Operating rule:** if any required gate is red, do not ship. Fix the gate, rerun the transcript, update the release manifest, and only then tag the release.

## Current launch status

**Current state:** Stage 0 is **NO-GO** until Rust-equipped validation, Docker/CI clean-run evidence, and signed SHA256 release evidence are attached.

This is not a weakness to hide. It is the correct release posture. The artifact already has strong local evidence for Python validators, TypeScript reference tests, formal static checks, vector validation, and Builder Scaffold tests. It must not claim full v1.0 orbital insertion until Rust, CI, Docker, and signing evidence are green on clean runners.

## Stage 0: Pre-launch - current state to ignition

| Gate | Evidence required | Current status | Decision |
|---|---|---:|---:|
| Existing validation transcripts green | Rust, TypeScript, Python scaffold, golden vectors, negative corpus | Partial: TS/Python/vector/formal static green; Rust not observed locally | NO-GO |
| Release manifest verified and signed | SHA256 file, manifest, detached signature, public verification command | SHA256 present; signature not attached | NO-GO |
| Docker and CI gates pass on clean runners | Fresh runner transcript, artifact upload, CI job URL or exported log | Scripts/workflow present; clean runner not observed here | NO-GO |
| NeurIPS E&D PDF/checklist/supplement ready | PDF, checklist, supplement zip, anonymized package | Ready in package | GO |
| Public repo initialized | CC-BY-SA-4.0 LICENSE, README, Verify in 5 Minutes command | Package docs present; public repo not verified here | NO-GO |

**Stage 0 launch criterion:** run `scripts/stage0_prelaunch_gate.sh` on a Rust + Docker equipped clean runner and obtain `stage0_prelaunch=PASS` with SHA256/signature evidence attached.

## Stage 1: v1.0 Kernel Fortress - First Orbital Insertion

**Goal:** make reviewers say, "this is real." The reviewer must verify the core artifact in under five minutes and see a complete release transcript.

| Requirement | Acceptance criterion | Evidence artifact |
|---|---|---|
| Rust checker impeccable | `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, full test suite, coverage report | `ci_evidence/rust_gate.log`, coverage HTML/LCOV |
| Negative corpus >= 30 | Every fixture has `fixture`, `expected_first_failure_gate`, and `canon_article` | `negative_corpus/manifest.json` |
| Golden vectors >= 8 | Exact, Stutter x2, SafeRefined x2, governance update, recovery, memory compression, long stutter chain | `golden/manifest.json` |
| Differential testing | Rust == TypeScript == Python on every fixture and first-failure gate | `ci_evidence/differential_matrix.json` |
| Proptest + fuzz | At least one hour, zero failures, seed log retained | `ci_evidence/fuzz_1h.log` |
| TCB ledger | Every crate/version justified with replacement plan | `docs/tcb_ledger.md` |
| Security review | STRIDE model plus mitigation matrix | `docs/stride_security_review.md` |
| Documentation | One-pager overview, interactive TS demo, reviewer quickstart video outline/script | `docs/`, `demo/`, `media/` |
| Tag release | CI publishes artifacts automatically on tag | `.github/workflows/release.yml` |

**Stage 1 launch criterion:** `bash scripts/run_release_gates.sh` prints `release_gate=PASS`, CI publishes release artifacts on a tag, and the transcript contains no missing Rust, Docker, fixture, signing, or coverage evidence.

## Stage 2: v2.0 Mechanized Beast - Second Stage Burn

**Goal:** turn executable evidence into formally grounded evidence.

| Requirement | Acceptance criterion | Evidence artifact |
|---|---|---|
| Lean 4 mechanization | Checker soundness, PO bundle reflection, Step Simulation Law; at least 70 percent of critical path | `formal/lean/`, `mechanization_manifest.json` |
| Rust verification fragment | Partial Prusti/Kani verification on checker core | `formal/rust-contracts/` |
| Numerical stability report | Error bounds for simplex projection, drift, risk vectors | `docs/numerical_stability_tcb.md` |
| Federation expansion | Quorum safety gates and cross-shard lineage tests | `federation/`, `rust/tests/federation_quorum.rs` |
| Memory compression proofs | Entropy monotonicity proof notes and reconstruction tests | `docs/memory_lineage_compression_stress.md` |
| Public corpus | Negative corpus repo and fixture classification explorer | `negative_corpus/`, `tools/fixture_explorer.*` |
| Certification templates | Compliance self-assessment for adopters | `certification/` |
| Independent ports | Zig and Go reference implementations pass differential tests | `ports/zig/`, `ports/go/` |

**Stage 2 launch criterion:** mechanization paper draft exists, proof-equipped gates run green, and all new fixtures pass Rust/TS/Python differential testing.

## Stage 3: v3.0 Ecosystem Domination - Third Stage and Orbital Refueling

**Goal:** make SCC a practical standard that external teams can adopt.

| Requirement | Acceptance criterion | Evidence artifact |
|---|---|---|
| Full proof direction | Lean/Coq checker soundness and extraction path to verified Rust fragments | `formal/` |
| Production-grade ports | Rust FFI, WASM, embedded profile | `ports/` |
| BOTG adversarial suite | 10k+ step traces, gradient-hacking attempts, Byzantine shard scenarios | `adversarial/` |
| Treaty and amendment framework | Live governance simulation | `governance_sim/` |
| Third-party audit | Published independent security review | `audits/` |
| Certification program | Badge criteria and public registry | `certification/` |
| Conference path | NeurIPS E&D plus systems/security submissions | `papers/` |
| Community | Discord/forum, roadmap, contribution guide, SCC in 100 lines | `COMMUNITY.md`, `CONTRIBUTING.md`, `examples/` |

**Stage 3 launch criterion:** multiple external teams can run the framework, pass the certification checklist, and reproduce the evidence without private context.

## Stage 4+: Constellation Mode - We Keep Firing

- Continuous fuzzing and nightly regression runs.
- Live governance on a reference agent.
- Integration with major agent frameworks.
- Formal verification of the full seven-stage pipeline.
- Global negative-corpus crowdsourcing.
- Annual SCC Fortress Day with new fixtures and adversarial challenges.

## 10/10 success metrics

- A reviewer can verify the artifact claim in under five minutes.
- There are zero open issues on the critical path.
- A downstream project can pass certification without private help.
- The TCB is smaller, clearer, or better justified every release.
- The repo feels complete enough that a tired reviewer understands the evidence in one pass.

## Final countdown command

Before every push, answer all five:

1. Does this change preserve protected isolation?
2. Does the checker still reject every negative fixture at the named first-failure gate?
3. Would a tired reviewer understand the evidence in one pass?
4. Is the TCB smaller or better justified?
5. Are we closer to mechanized soundness?

If yes to all five, light the engines.
