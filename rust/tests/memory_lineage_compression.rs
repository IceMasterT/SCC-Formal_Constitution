// SPDX-License-Identifier: Apache-2.0
// Copyright 2025-2026 Ian Farquharson
#![allow(missing_docs)]
mod common;

use scc_kernel::{
    accepts,
    audit::{audit_event_hash, expected_audit_event},
    checker::CheckerError,
    events::event_hash,
    hash::{derive_root, ZERO_HASH},
    state::state_hash,
};

use common::{read_fixture, repo_root, Fixture};

fn repair_hashes(mut fx: Fixture) -> Fixture {
    fx.next.audit_hash = ZERO_HASH.to_string();
    let audit_event = expected_audit_event(&fx.po, &fx.prev, &fx.next, &fx.event, &fx.env).unwrap();
    let audit_hash = audit_event_hash(&audit_event).unwrap();
    fx.next.audit_hash = audit_hash.clone();
    fx.po.prev_state_hash = state_hash(&fx.prev).unwrap();
    fx.po.next_state_hash = state_hash(&fx.next).unwrap();
    fx.po.event_hash = event_hash(&fx.event).unwrap();
    fx.po.audit_event_hash = audit_hash;
    fx.audit_event = audit_event;
    fx
}

#[test]
fn memory_compression_step_accepts_when_lineage_is_extended() {
    let mut fx = read_fixture(repo_root().join("golden/valid_exact.json"));
    fx.event.kind = "memory_compress".to_string();
    fx.event
        .attributes
        .insert("compression".to_string(), "lossless-dedup-v1".to_string());
    fx.event.attributes.insert(
        "memory_root_before".to_string(),
        fx.prev.memory_root.clone(),
    );
    fx.event
        .attributes
        .insert("memory_root_after".to_string(), fx.next.memory_root.clone());
    let event_hash = event_hash(&fx.event).unwrap();
    fx.next.lineage_root = derive_root("lineage", &[&fx.prev.lineage_root, &event_hash]).unwrap();
    let fx = repair_hashes(fx);
    accepts(
        &fx.po,
        &fx.prev,
        &fx.next,
        &fx.event,
        &fx.audit_event,
        &fx.env,
    )
    .unwrap();
}

#[test]
fn memory_compression_lineage_erasure_rejects_after_hashes_repaired() {
    let mut fx = read_fixture(repo_root().join("golden/valid_exact.json"));
    fx.event.kind = "memory_compress".to_string();
    fx.event
        .attributes
        .insert("compression".to_string(), "lossless-dedup-v1".to_string());
    fx.next.lineage_root = fx.prev.lineage_root.clone();
    let fx = repair_hashes(fx);
    let err = accepts(
        &fx.po,
        &fx.prev,
        &fx.next,
        &fx.event,
        &fx.audit_event,
        &fx.env,
    )
    .expect_err("lineage erasure must be the first semantic failure");
    assert_eq!(err, CheckerError::LineageViolation);
}
