# Numerical stability and TCB error bounds

Kernel Fortress v1 uses deterministic finite `f64` encodings for governance weights and risk values.  This is a TCB item, not a mathematical theorem about real arithmetic.

## Encoding discipline

- All floats must be finite IEEE-754 binary64 values.
- NaN and infinity are rejected by the domain predicates.
- Negative zero is canonicalized to positive zero before hashing.
- Canonical bytes use big-endian binary64.

## Simplex tolerance

The checker accepts a governance vector only when every coordinate is at least `-epsilon` and the sum is within `epsilon` of one.  The descriptor supplies `simplex_epsilon`; the default artifact uses `1e-9`, deliberately larger than ordinary summation roundoff for the fixture sizes.

For a sum of `n` finite binary64 values with unit roundoff `u = 2^-53`, the standard first-order bound is approximately

```text
gamma_n = n*u/(1 - n*u)
```

For `n <= 1024`, `gamma_n` is about `1.14e-13`, so `1e-9` leaves more than four orders of magnitude of slack for fixture-scale vectors.  Production deployments should set `epsilon >= 8*gamma_n + serialization_margin` and record `n_max` in the descriptor.

## Risk threshold margin

Risk-to-halt checks should be paranoid near the threshold.  Production descriptors should treat any risk sum within `epsilon` of the halt threshold as halt-required unless a fixed-point risk representation is used.  The TCB ledger must name which mode is active:

- `deterministic-f64-be-v1`: portable canonical f64 with explicit tolerance.
- `fixed-point-u64-v1`: recommended for production critical deployments.

## Reviewer rule

If a claim depends on real-valued governance or risk arithmetic, the paper must cite the descriptor tolerance and the TCB ledger.  A checker pass without a numerical-mode declaration is incomplete evidence.
