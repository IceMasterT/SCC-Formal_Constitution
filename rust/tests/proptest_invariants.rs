// SPDX-License-Identifier: Apache-2.0
// Copyright 2025-2026 Ian Farquharson
#![allow(missing_docs)]

use std::collections::BTreeMap;

use proptest::prelude::*;
use scc_kernel::{
    accepts,
    audit::{audit_event_hash, expected_audit_event},
    checker::CheckerError,
    events::{event_hash, CanonicalEvent},
    governance::project_to_simplex,
    hash::derive_root,
    po::{POBundle, StepMode},
    state::{halt_absorption_holds, protected_unchanged, state_hash, valid_simplex, SCCState},
    transition::candidate_next,
    AuditEvent, ExecutionEnv,
};

fn dummy_state(failure_mode: u8, protected_root: String) -> SCCState {
    SCCState {
        schema_version: "scc.state.v1".to_string(),
        state_id: "s".to_string(),
        step: 0,
        compute_root: "11".repeat(32),
        memory_root: "22".repeat(32),
        vector_clock: Default::default(),
        governance_weights: vec![0.5, 0.5],
        failure_mode,
        audit_hash: "33".repeat(32),
        risk_vector: vec![0.1],
        lineage_root: "44".repeat(32),
        protected_root,
    }
}

/// Build a domain-valid previous state from generated coordinates.
fn seeded_prev(step: u64, gov: &[f64], risk: Vec<f64>, seed: &str) -> SCCState {
    SCCState {
        schema_version: "scc.state.v1".to_string(),
        state_id: format!("state:{step}"),
        step,
        compute_root: derive_root("compute-seed", &[seed]).unwrap(),
        memory_root: derive_root("memory-seed", &[seed]).unwrap(),
        vector_clock: BTreeMap::from([("agent".to_string(), step)]),
        governance_weights: project_to_simplex(gov),
        failure_mode: 0,
        audit_hash: derive_root("audit-seed", &[seed]).unwrap(),
        risk_vector: risk,
        lineage_root: derive_root("lineage-seed", &[seed]).unwrap(),
        protected_root: derive_root("protected-seed", &[seed]).unwrap(),
    }
}

fn seeded_event(prev: &SCCState, kind: &str, producer: &str, seed: &str) -> CanonicalEvent {
    CanonicalEvent {
        schema_version: "scc.event.v1".to_string(),
        event_id: format!("event:{}", prev.step + 1),
        kind: kind.to_string(),
        producer: producer.to_string(),
        step: prev.step + 1,
        parent_hash: derive_root("parent", &[&prev.lineage_root]).unwrap(),
        payload_hash: derive_root("payload", &[seed]).unwrap(),
        attributes: BTreeMap::new(),
    }
}

/// Build a fully evidence-consistent transition for the given mode.
fn build_transition(
    prev: &SCCState,
    event: &CanonicalEvent,
    mode: StepMode,
    env: &ExecutionEnv,
) -> (POBundle, SCCState, AuditEvent) {
    let mut next = candidate_next(prev, event, mode, env).unwrap();
    let mut po = POBundle {
        schema_version: "scc.po.v1".to_string(),
        checker_version: env.checker_version.clone(),
        step_id: format!("step:{}", next.step),
        mode,
        dom: true,
        inv: true,
        identity: true,
        governance: true,
        risk: true,
        audit: true,
        isolation: true,
        refinement: true,
        prev_state_hash: String::new(),
        next_state_hash: String::new(),
        event_hash: String::new(),
        audit_event_hash: String::new(),
        authorized_meta_path: false,
        recovery_path: false,
        witness_refs: BTreeMap::new(),
    };
    let audit = expected_audit_event(&po, prev, &next, event, env).unwrap();
    let audit_hash = audit_event_hash(&audit).unwrap();
    next.audit_hash = audit_hash.clone();
    po.prev_state_hash = state_hash(prev).unwrap();
    po.next_state_hash = state_hash(&next).unwrap();
    po.event_hash = event_hash(event).unwrap();
    po.audit_event_hash = audit_hash;
    (po, next, audit)
}

/// Recompute all evidence after a post-construction state mutation.
fn repair_evidence(
    po: &mut POBundle,
    prev: &SCCState,
    next: &mut SCCState,
    event: &CanonicalEvent,
    env: &ExecutionEnv,
) -> AuditEvent {
    next.audit_hash = "00".repeat(32);
    let audit = expected_audit_event(po, prev, next, event, env).unwrap();
    let audit_hash = audit_event_hash(&audit).unwrap();
    next.audit_hash = audit_hash.clone();
    po.prev_state_hash = state_hash(prev).unwrap();
    po.next_state_hash = state_hash(next).unwrap();
    po.event_hash = event_hash(event).unwrap();
    po.audit_event_hash = audit_hash;
    audit
}

fn mode_from_index(i: u8) -> StepMode {
    match i {
        0 => StepMode::Exact,
        1 => StepMode::Stutter,
        _ => StepMode::SafeRefined,
    }
}

fn arbitrary_case() -> impl Strategy<Value = (SCCState, CanonicalEvent, StepMode)> {
    (
        proptest::collection::vec(0.01f64..1.0, 2..5),
        proptest::collection::vec(0.06f64..0.25, 1..4),
        0u64..1_000,
        any::<bool>(),
        0u8..3,
        "[a-z]{1,12}",
    )
        .prop_map(|(gov, risk, step, risk_signal, mode_idx, seed)| {
            let prev = seeded_prev(step, &gov, risk, &seed);
            let kind = if risk_signal {
                "risk_signal"
            } else {
                "state_update"
            };
            let event = seeded_event(&prev, kind, "agent", &seed);
            (prev, event, mode_from_index(mode_idx))
        })
}

proptest! {
    #[test]
    fn projection_lands_on_simplex(xs in proptest::collection::vec(-10.0f64..10.0, 1..32)) {
        let p = project_to_simplex(&xs);
        prop_assert!(valid_simplex(&p, 1e-9));
    }

    #[test]
    fn protected_coordinate_cannot_change_without_authorization(a in "[0-9a-f]{64}", b in "[0-9a-f]{64}") {
        let prev = dummy_state(0, a.clone());
        let next = dummy_state(0, b.clone());
        prop_assert_eq!(protected_unchanged(&prev, &next, false), a == b);
        prop_assert!(protected_unchanged(&prev, &next, true));
    }

    #[test]
    fn halt_is_absorbing_without_recovery(next_mode in 0u8..=3) {
        let prev = dummy_state(3, "aa".repeat(32));
        let next = dummy_state(next_mode, "aa".repeat(32));
        prop_assert_eq!(halt_absorption_holds(&prev, &next, false), next_mode == 3);
    }

    #[test]
    fn generated_transitions_are_accepted((prev, event, mode) in arbitrary_case()) {
        let env = ExecutionEnv::golden_v1();
        let (po, next, audit) = build_transition(&prev, &event, mode, &env);
        prop_assert_eq!(accepts(&po, &prev, &next, &event, &audit, &env), Ok(()));
    }

    #[test]
    fn corrupted_state_commitment_is_rejected((prev, event, mode) in arbitrary_case()) {
        let env = ExecutionEnv::golden_v1();
        let (mut po, next, audit) = build_transition(&prev, &event, mode, &env);
        let flipped = if po.prev_state_hash.starts_with('0') { "1" } else { "0" };
        po.prev_state_hash = format!("{}{}", flipped, &po.prev_state_hash[1..]);
        prop_assert_eq!(
            accepts(&po, &prev, &next, &event, &audit, &env),
            Err(CheckerError::HashVerificationFailed)
        );
    }

    #[test]
    fn wrong_declared_mode_is_rejected((prev, event, _mode) in arbitrary_case()) {
        let env = ExecutionEnv::golden_v1();
        let (mut po, mut next, _) = build_transition(&prev, &event, StepMode::Exact, &env);
        po.mode = StepMode::Stutter;
        let audit = repair_evidence(&mut po, &prev, &mut next, &event, &env);
        prop_assert_eq!(
            accepts(&po, &prev, &next, &event, &audit, &env),
            Err(CheckerError::RefinementViolation)
        );
    }

    #[test]
    fn step_regression_is_rejected((prev, event, mode) in arbitrary_case()) {
        let env = ExecutionEnv::golden_v1();
        let (mut po, mut next, _) = build_transition(&prev, &event, mode, &env);
        next.step = prev.step;
        let audit = repair_evidence(&mut po, &prev, &mut next, &event, &env);
        prop_assert_eq!(
            accepts(&po, &prev, &next, &event, &audit, &env),
            Err(CheckerError::LineageViolation)
        );
    }

    #[test]
    fn tampered_protected_root_is_rejected((prev, event, mode) in arbitrary_case()) {
        let env = ExecutionEnv::golden_v1();
        let (mut po, mut next, _) = build_transition(&prev, &event, mode, &env);
        next.protected_root = derive_root("tampered", &[&next.protected_root]).unwrap();
        let audit = repair_evidence(&mut po, &prev, &mut next, &event, &env);
        prop_assert_eq!(
            accepts(&po, &prev, &next, &event, &audit, &env),
            Err(CheckerError::ProtectedMutation)
        );
    }
}
