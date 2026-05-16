// SPDX-License-Identifier: Apache-2.0
// Copyright 2025-2026 Ian Farquharson
#![allow(missing_docs)]
mod common;

use std::fs;

use glob::glob;
use scc_kernel::{
    accepts,
    audit::audit_event_hash,
    canonical::CanonicalBytes,
    events::event_hash,
    po::po_hash,
    state::{state_hash, SCCState},
};

use common::{read_fixture, repo_root};

#[test]
fn golden_vectors_accept_and_hashes_match() {
    for path in glob(repo_root().join("golden/*.json").to_str().unwrap()).unwrap() {
        let path = path.unwrap();
        if path.file_name().unwrap() == "manifest.json" {
            continue;
        }
        let fx = read_fixture(path.clone());
        assert!(fx.expected.accepted, "golden fixture is marked accepted");
        accepts(
            &fx.po,
            &fx.prev,
            &fx.next,
            &fx.event,
            &fx.audit_event,
            &fx.env,
        )
        .unwrap_or_else(|e| panic!("{} rejected: {:?}", fx.name, e));
        assert_eq!(state_hash(&fx.prev).unwrap(), fx.hashes["prev_state_hash"]);
        assert_eq!(state_hash(&fx.next).unwrap(), fx.hashes["next_state_hash"]);
        assert_eq!(event_hash(&fx.event).unwrap(), fx.hashes["event_hash"]);
        assert_eq!(
            audit_event_hash(&fx.audit_event).unwrap(),
            fx.hashes["audit_event_hash"]
        );
        assert_eq!(po_hash(&fx.po).unwrap(), fx.hashes["po_hash"]);

        let name = path.file_stem().unwrap().to_string_lossy();
        let bin_dir = repo_root().join("golden/bin");
        assert_eq!(
            fx.prev.canonical_bytes().unwrap(),
            fs::read(bin_dir.join(format!("{name}.prev_state.bin"))).unwrap()
        );
        assert_eq!(
            fx.next.canonical_bytes().unwrap(),
            fs::read(bin_dir.join(format!("{name}.next_state.bin"))).unwrap()
        );
        assert_eq!(
            fx.event.canonical_bytes().unwrap(),
            fs::read(bin_dir.join(format!("{name}.event.bin"))).unwrap()
        );
        assert_eq!(
            fx.audit_event.canonical_bytes().unwrap(),
            fs::read(bin_dir.join(format!("{name}.audit_event.bin"))).unwrap()
        );
        assert_eq!(
            fx.po.canonical_bytes().unwrap(),
            fs::read(bin_dir.join(format!("{name}.po.bin"))).unwrap()
        );
    }
}

#[test]
fn replay_determinism_same_input_same_hash() {
    let fx = read_fixture(repo_root().join("golden/valid_exact.json"));
    let h1 = state_hash(&fx.next).unwrap();
    let cloned: SCCState = serde_json::from_str(&serde_json::to_string(&fx.next).unwrap()).unwrap();
    let h2 = state_hash(&cloned).unwrap();
    assert_eq!(h1, h2);
}
