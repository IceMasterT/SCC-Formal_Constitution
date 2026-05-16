#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
# Copyright 2025-2026 Ian Farquharson
set -euo pipefail

printf 'SCC Kernel Fortress release gate\n'
printf 'working_dir=%s\n' "$(pwd)"
printf 'python=%s\n' "$(python3 --version 2>&1)"
printf 'node=%s\n' "$(node --version 2>&1)"
printf 'npm=%s\n' "$(npm --version 2>&1)"
printf 'rustc=%s\n' "$(rustc --version 2>&1)"
printf 'cargo=%s\n' "$(cargo --version 2>&1)"
printf 'lake=%s\n' "$(lake --version 2>&1 || true)"
printf 'coqc=%s\n' "$(coqc --version 2>&1 | head -n 1 || true)"

python3 tools/validate_vectors.py
python3 tools/forbidden_imports.py
python3 tools/validate_formal_artifacts.py
(
  cd rust
  cargo test --all-features
  cargo test --test negative_corpus
  cargo test --test golden_vectors
  cargo test --test proptest_invariants
  cargo test --test memory_lineage_compression
  cargo test --test numerical_stability
  cargo test --test federation_quorum
)
(
  cd ts
  npm test
)
if command -v lake >/dev/null 2>&1; then
  (cd formal/lean && lake build)
else
  printf 'formal_lean=SKIP reason=lake_not_available\n'
fi
if command -v coqc >/dev/null 2>&1; then
  (cd formal/coq && coqc SCCKernelFortress.v)
else
  printf 'formal_coq=SKIP reason=coqc_not_available\n'
fi
printf 'release_gate=PASS\n'
