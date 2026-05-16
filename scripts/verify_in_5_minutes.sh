#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
# Copyright 2025-2026 Ian Farquharson
set -euo pipefail

echo "SCC Kernel Fortress verify-in-5-minutes gate"
echo "working_dir=$(pwd)"

python3 tools/validate_vectors.py
python3 tools/forbidden_imports.py
python3 tools/validate_formal_artifacts.py

if command -v npm >/dev/null 2>&1; then
  (cd ts && npm test)
else
  echo "npm missing: cannot run TypeScript reference tests" >&2
  exit 1
fi

if command -v cargo >/dev/null 2>&1; then
  (cd rust && cargo fmt --check)
  (cd rust && cargo clippy --all-targets --all-features -- -D warnings)
  (cd rust && cargo test --all-features)
else
  echo "cargo missing: Rust source-of-truth gate not executed" >&2
  exit 1
fi

echo "verify_in_5_minutes=PASS"
