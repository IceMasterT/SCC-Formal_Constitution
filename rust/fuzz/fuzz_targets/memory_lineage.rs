// SPDX-License-Identifier: Apache-2.0
// Copyright 2025-2026 Ian Farquharson
#![no_main]

use libfuzzer_sys::fuzz_target;
use scc_kernel::{
    accepts,
    audit::{audit_event_hash, expected_audit_event},
    events::event_hash,
    hash::{derive_root, ZERO_HASH},
    state::state_hash,
    AuditEvent, CanonicalEvent, ExecutionEnv, POBundle, SCCState,
};

#[derive(serde::Deserialize)]
struct Candidate {
    po: POBundle,
    prev: SCCState,
    next: SCCState,
    event: CanonicalEvent,
    audit_event: AuditEvent,
    env: ExecutionEnv,
}

fuzz_target!(|data: &[u8]| {
    if let Ok(mut candidate) = serde_json::from_slice::<Candidate>(data) {
        let _ = accepts(
            &candidate.po,
            &candidate.prev,
            &candidate.next,
            &candidate.event,
            &candidate.audit_event,
            &candidate.env,
        );
        if let Ok(eh) = event_hash(&candidate.event) {
            if let Ok(lineage) = derive_root("lineage", &[&candidate.prev.lineage_root, &eh]) {
                candidate.next.lineage_root = lineage;
                candidate.next.audit_hash = ZERO_HASH.to_string();
                if let Ok(audit) = expected_audit_event(&candidate.po, &candidate.prev, &candidate.next, &candidate.event, &candidate.env) {
                    if let Ok(ah) = audit_event_hash(&audit) {
                        candidate.next.audit_hash = ah.clone();
                        if let Ok(prev_hash) = state_hash(&candidate.prev) { candidate.po.prev_state_hash = prev_hash; }
                        if let Ok(next_hash) = state_hash(&candidate.next) { candidate.po.next_state_hash = next_hash; }
                        candidate.po.event_hash = eh;
                        candidate.po.audit_event_hash = ah;
                        let _ = accepts(&candidate.po, &candidate.prev, &candidate.next, &candidate.event, &audit, &candidate.env);
                    }
                }
            }
        }
    }
});
