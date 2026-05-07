import test from 'node:test';
import assert from 'node:assert/strict';
import {
  accepts,
  auditEventHash,
  deriveRoot,
  eventHash,
  projectToSimplex,
  stateBodyHash,
  stateHash,
  ZERO,
  type AuditEvent,
  type CanonicalEvent,
  type Fixture,
  type POBundle,
  type SCCState,
} from '../src/index.ts';
import { readFixture } from './_util.ts';

function repairHashes(prev: SCCState, next0: SCCState, event: CanonicalEvent, po0: POBundle, env: Fixture['env']) {
  const next: SCCState = { ...next0, audit_hash: ZERO };
  const evh = eventHash(event);
  const audit_event: AuditEvent = {
    schema_version: env.expected_audit_schema,
    step_id: po0.step_id,
    prev_audit_hash: prev.audit_hash,
    event_hash: evh,
    prev_state_hash: stateHash(prev),
    next_state_body_hash: stateBodyHash(next),
    checker_version: po0.checker_version,
    descriptor_hash: env.descriptor_hash,
  };
  const auditHash = auditEventHash(audit_event);
  next.audit_hash = auditHash;
  const po: POBundle = {
    ...po0,
    prev_state_hash: stateHash(prev),
    next_state_hash: stateHash(next),
    event_hash: evh,
    audit_event_hash: auditHash,
    witness_refs: {
      ...po0.witness_refs,
      memory: 'compression-witness:lossless-dedup-v1',
      mechanization: 'formal/lean/SCC/KernelFortress/GateSoundness.lean',
    },
  };
  return { next, audit_event, po };
}

test('memory compression transition accepts when lineage is recomputed and hashes are repaired', () => {
  const fx = readFixture('golden/valid_exact.json');
  const event: CanonicalEvent = {
    ...fx.event,
    kind: 'memory_compress',
    attributes: {
      ...fx.event.attributes,
      compression: 'lossless-dedup-v1',
      memory_root_before: fx.prev.memory_root,
      memory_root_after: fx.next.memory_root,
    },
  };
  const event_hash = eventHash(event);
  const next0: SCCState = {
    ...fx.next,
    lineage_root: deriveRoot('lineage', fx.prev.lineage_root, event_hash),
  };
  const repaired = repairHashes(fx.prev, next0, event, fx.po, fx.env);
  const r = accepts(repaired.po, fx.prev, repaired.next, event, repaired.audit_event, fx.env);
  assert.equal(r.ok, true);
  assert.equal(r.error, null);
});

test('memory compression transition rejects lineage erasure after all hashes are repaired', () => {
  const fx = readFixture('golden/valid_exact.json');
  const event: CanonicalEvent = {
    ...fx.event,
    kind: 'memory_compress',
    attributes: {
      ...fx.event.attributes,
      compression: 'lossless-dedup-v1',
      memory_root_before: fx.prev.memory_root,
      memory_root_after: fx.next.memory_root,
    },
  };
  const next0: SCCState = {
    ...fx.next,
    lineage_root: fx.prev.lineage_root,
  };
  const repaired = repairHashes(fx.prev, next0, event, fx.po, fx.env);
  const r = accepts(repaired.po, fx.prev, repaired.next, event, repaired.audit_event, fx.env);
  assert.equal(r.ok, false);
  assert.equal(r.error, 'LineageViolation');
});

test('simplex projection remains finite and within the documented numerical envelope', () => {
  for (let n = 1; n <= 128; n += 1) {
    const xs = Array.from({ length: n }, (_, i) => Math.sin((i + 1) * 1.61803398875) * 1e6 - Math.cos(i + 3) * 1e3);
    const p = projectToSimplex(xs);
    assert.equal(p.length, n);
    assert.ok(p.every(Number.isFinite));
    assert.ok(p.every((x) => x >= -1e-12));
    const sum = p.reduce((a, b) => a + b, 0);
    assert.ok(Math.abs(sum - 1) <= 1e-9, `n=${n} sum=${sum}`);
  }
});

test('federation quorum arithmetic gives honest overlap under n=3f+1 q=2f+1', () => {
  for (let f = 0; f <= 64; f += 1) {
    const n = 3 * f + 1;
    const q = 2 * f + 1;
    const minIntersection = 2 * q - n;
    assert.equal(minIntersection, f + 1);
    assert.ok(minIntersection > f);
  }
});
