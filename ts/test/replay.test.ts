// SPDX-License-Identifier: Apache-2.0
// Copyright 2025-2026 Ian Farquharson
import test from 'node:test'; import assert from 'node:assert/strict'; import { replay,stateHash } from '../src/index.ts'; import { readFixture } from './_util.ts'; test('replay determinism',()=>{const fx=readFixture('golden/valid_exact.json'); const out1=replay([{po:fx.po,prev:fx.prev,next:fx.next,event:fx.event,audit_event:fx.audit_event}],fx.env); const out2=replay([{po:fx.po,prev:fx.prev,next:fx.next,event:fx.event,audit_event:fx.audit_event}],fx.env); assert.equal(stateHash(out1),stateHash(out2));});
