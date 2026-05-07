# SCC Kernel Fortress formal bridge

This directory gives the mechanization bridge for the v1 executable artifact.  It is not a claim that the entire SCC canon is mechanically proved.  It is the formal boundary for the implemented checker subset: a candidate transition accepted by the v1 gate implies the corresponding Prop-level v1 obligations in the small formal model.

Contents:

- `lean/SCC/KernelFortress/GateSoundness.lean` - Lean 4 Bool-to-Prop bridge for the v1 gate model.
- `lean/SCC/KernelFortress/PipelineWitness.lean` - Lean 4 seven-stage witness structure and acceptance theorem.
- `coq/SCCKernelFortress.v` - Coq proof fragment for required-bundle reflection and quorum-intersection arithmetic.
- `rust-contracts/README.md` - mapping from Rust functions to formal obligations and verification candidates.

Release discipline:

- These files must contain no `sorry`, `admit`, `Admitted`, or unledgered `axiom`.
- A Rust-equipped release gate validates the executable checker.
- A proof-equipped release gate should run `lake build` and `coqc` for the bridge when Lean/Coq are available.
- If Lean or Coq are unavailable in a local sandbox, do not report a local proof-assistant pass.  Report the static placeholder scan and attach CI evidence from a proof-equipped runner.
