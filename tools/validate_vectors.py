#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
# Copyright 2025-2026 Ian Farquharson
from __future__ import annotations
from pathlib import Path
import json, re, sys

ROOT = Path(__file__).resolve().parents[1]
REQUIRED_STATE = {'schema_version','state_id','step','compute_root','memory_root','vector_clock','governance_weights','failure_mode','audit_hash','risk_vector','lineage_root','protected_root'}
REQUIRED_EVENT = {'schema_version','event_id','kind','producer','step','parent_hash','payload_hash','attributes'}
REQUIRED_AUDIT = {'schema_version','step_id','prev_audit_hash','event_hash','prev_state_hash','next_state_body_hash','checker_version','descriptor_hash'}
REQUIRED_PO = {'schema_version','checker_version','step_id','mode','dom','inv','identity','governance','risk','audit','isolation','refinement','prev_state_hash','next_state_hash','event_hash','audit_event_hash','authorized_meta_path','recovery_path','witness_refs'}


def schema_errors(obj, schema, path):
    """Minimal JSON Schema validator for the subset used by schemas/*.json."""
    errs = []
    t = schema.get('type')
    if t == 'object':
        if not isinstance(obj, dict):
            return [f'{path}: expected object']
        for key in schema.get('required', []):
            if key not in obj:
                errs.append(f'{path}.{key}: missing required field')
        props = schema.get('properties', {})
        extra = schema.get('additionalProperties')
        for key, value in obj.items():
            if key in props:
                errs.extend(schema_errors(value, props[key], f'{path}.{key}'))
            elif isinstance(extra, dict):
                errs.extend(schema_errors(value, extra, f'{path}.{key}'))
            elif extra is False:
                errs.append(f'{path}.{key}: unexpected field')
        return errs
    if 'enum' in schema:
        if obj not in schema['enum']:
            errs.append(f'{path}: {obj!r} not in enum {schema["enum"]}')
        return errs
    if t == 'string':
        if not isinstance(obj, str):
            return [f'{path}: expected string']
        pattern = schema.get('pattern')
        if pattern and not re.fullmatch(pattern, obj):
            errs.append(f'{path}: does not match {pattern}')
        return errs
    if t == 'integer':
        if not isinstance(obj, int) or isinstance(obj, bool):
            return [f'{path}: expected integer']
        if 'minimum' in schema and obj < schema['minimum']:
            errs.append(f'{path}: below minimum {schema["minimum"]}')
        if 'maximum' in schema and obj > schema['maximum']:
            errs.append(f'{path}: above maximum {schema["maximum"]}')
        return errs
    if t == 'number':
        if not isinstance(obj, (int, float)) or isinstance(obj, bool):
            return [f'{path}: expected number']
        return errs
    if t == 'boolean':
        if not isinstance(obj, bool):
            return [f'{path}: expected boolean']
        return errs
    if t == 'array':
        if not isinstance(obj, list):
            return [f'{path}: expected array']
        if 'minItems' in schema and len(obj) < schema['minItems']:
            errs.append(f'{path}: fewer than {schema["minItems"]} items')
        items = schema.get('items')
        if isinstance(items, dict):
            for i, value in enumerate(obj):
                errs.extend(schema_errors(value, items, f'{path}[{i}]'))
        return errs
    return errs


SCHEMAS = {
    'state': json.loads((ROOT / 'schemas' / 'state.schema.json').read_text()),
    'event': json.loads((ROOT / 'schemas' / 'event.schema.json').read_text()),
    'po': json.loads((ROOT / 'schemas' / 'po.schema.json').read_text()),
}


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
    if should_accept:
        # Golden fixtures must satisfy the carrier schemas exactly. Negative
        # fixtures intentionally violate value constraints, so they only get
        # the structural checks above.
        errs = (
            schema_errors(fx['prev'], SCHEMAS['state'], 'prev')
            + schema_errors(fx['next'], SCHEMAS['state'], 'next')
            + schema_errors(fx['event'], SCHEMAS['event'], 'event')
            + schema_errors(fx['po'], SCHEMAS['po'], 'po')
        )
        if errs:
            raise AssertionError(f'{path}: schema violations:\n  ' + '\n  '.join(errs))
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
print(f'validate_vectors: OK ({count} fixtures, golden schema-validated)')
