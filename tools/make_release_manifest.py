#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
# Copyright 2025-2026 Ian Farquharson
from __future__ import annotations
from pathlib import Path
import hashlib, json
ROOT = Path(__file__).resolve().parents[1]
include_dirs = ['rust/src','rust/tests','rust/fuzz','ts/src','ts/test','golden','negative_corpus','schemas','docs','tools','scripts','.github','paper','formal','federation']
items = []
root_files = ['README.md','Makefile','Dockerfile','release_manifest.json','mechanization_manifest.json','rust/Cargo.toml','rust/rust-toolchain.toml','ts/package.json']
for rel in root_files:
    p = ROOT / rel
    if p.exists() and p.is_file():
        items.append({'path': rel, 'sha256': hashlib.sha256(p.read_bytes()).hexdigest(), 'bytes': p.stat().st_size})
for d in include_dirs:
    for p in sorted((ROOT/d).rglob('*')):
        if p.is_file() and not p.name.endswith(('.aux','.log','.out','.fls','.fdb_latexmk','.bbl','.blg')):
            items.append({'path': str(p.relative_to(ROOT)), 'sha256': hashlib.sha256(p.read_bytes()).hexdigest(), 'bytes': p.stat().st_size})
manifest = {'schema': 'scc.release_manifest.v1', 'version': '1.0.0', 'items': items}
out = ROOT / 'release_manifest.json'
out.write_text(json.dumps(manifest, indent=2) + '\n')
print(out)
