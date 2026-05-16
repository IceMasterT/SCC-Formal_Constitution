#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
# Copyright 2025-2026 Ian Farquharson
set -euo pipefail

python3 tools/validate_formal_artifacts.py

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
