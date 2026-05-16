#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
# Copyright 2025-2026 Ian Farquharson
set -euo pipefail
mkdir -p validation
{
  date -u +"generated_at=%Y-%m-%dT%H:%M:%SZ"
  bash scripts/run_release_gates.sh
} | tee validation/release_gate_transcript.txt
python3 tools/make_release_manifest.py
cp release_manifest.json validation/release_manifest.json
sha256sum validation/release_gate_transcript.txt validation/release_manifest.json | tee validation/sha256sums.txt
