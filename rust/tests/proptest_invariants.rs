#![allow(missing_docs)]

use proptest::prelude::*;
use scc_kernel::{
    governance::project_to_simplex,
    state::{halt_absorption_holds, protected_unchanged, valid_simplex, SCCState},
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
}
