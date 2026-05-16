# SPDX-License-Identifier: Apache-2.0
# Copyright 2025-2026 Ian Farquharson
#!/usr/bin/env python3
from __future__ import annotations
from pathlib import Path
import json, sys

ROOT = Path(__file__).resolve().parents[1]
REQUIRED_STATE = {'schema_version','state_id','step','compute_root','memory_root','vector_clock','governance_weights','failure_mode','audit_hash','risk_vector','lineage_root','protected_root'}
REQUIRED_EVENT = {'schema_version','event_id','kind','producer','step','parent_hash','payload_hash','attributes'}
REQUIRED_AUDIT = {'schema_version','step_id','prev_audit_hash','event_hash','prev_state_hash','next_state_body_hash','checker_version','descriptor_hash'}
REQUIRED_PO = {'schema_version','checker_version','step_id','mode','dom','inv','identity','governance','risk','audit','isolation','refinement','prev_state_hash','next_state_hash','event_hash','audit_event_hash','authorized_meta_path','recovery_path','witness_refs'}

def assert_has(obj, keys, label, path):
    missing = keys - set(obj)
    if missing:
        raise AssertionError(f'{path}: {label} missing {sorted(missing)}')

def check_fixture(path: Path, should_accept: bool):
    fx = json.loads(path.read_text())
    assert_has(fx['prev'], REQUIRED_STATE, 'prev', path)
    assert_has(fx['next'], REQUIRED_STATE, 'next', path)
    assert_has(fx['event'], REQUIRED_EVENT, 'event', path)
    assert_has(fx['audit_event'], REQUIRED_AUDIT, 'audit_event', path)
    assert_has(fx['po'], REQUIRED_PO, 'po', path)
    if fx['expected']['accepted'] is not should_accept:
        raise AssertionError(f'{path}: expected.accepted mismatch')
    for key, value in fx['hashes'].items():
        if not isinstance(value, str) or len(value) != 64 or any(c not in '0123456789abcdef' for c in value):
            raise AssertionError(f'{path}: bad hash {key}')
    return fx

count = 0
for path in sorted((ROOT / 'golden').glob('*.json')):
    if path.name != 'manifest.json':
        fx = check_fixture(path, True); count += 1
        name = path.stem
        for suffix in ['prev_state','next_state','event','audit_event','po']:
            b = ROOT / 'golden' / 'bin' / f'{name}.{suffix}.bin'
            if not b.exists() or b.stat().st_size == 0:
                raise AssertionError(f'missing bin {b}')
for path in sorted((ROOT / 'negative_corpus' / 'cases').glob('*.json')):
    check_fixture(path, False); count += 1
print(f'validate_vectors: OK ({count} fixtures)')
