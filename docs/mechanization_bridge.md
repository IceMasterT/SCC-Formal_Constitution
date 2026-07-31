# Mechanization bridge

This release moves mechanization from roadmap-only to artifact-backed.

## What is mechanized

The formal bridge covers the implemented v1 subset:

- required proof-obligation Boolean fields,
- protected-coordinate preservation,
- halt absorption,
- risk-to-halt implication,
- lineage continuation,
- `Exact`/`Stutter`/`SafeRefined` step-mode obligations against a normalized
  projection and induced abstract successor (`ModeSound`),
- first-failure gate naming: the modeled `Check` pipeline rejects with the name
  of the first failed gate, and naming soundness is proved for representative
  gates (`reject_protected_names_violation`, `reject_refinement_names_violation`),
- seven-stage witness completeness,
- quorum-intersection arithmetic for the federation evidence layer.

The key proof shape is Bool-to-Prop reflection: if the executable gate accepts in the small formal model, the Prop-level obligation holds (`check_accept_sound`, with `check_accept_iff_gateBool` tying the pipeline to the decision procedure).

## What remains outside the proof

The bridge does not prove SHA-256 cryptographic security, compiler correctness, operating-system correctness, hardware correctness, or full mechanization of every SCC canon theorem.  Those remain explicit TCB or future-work boundaries.

## Commands

```bash
python3 tools/validate_formal_artifacts.py
bash scripts/run_formal_gates.sh
```

On a proof-equipped runner, `run_formal_gates.sh` executes `lake build` and `coqc`.  In sandboxes without Lean/Coq, it performs the placeholder/axiom scan and reports the proof-assistant checks as skipped rather than fabricating proof evidence.
