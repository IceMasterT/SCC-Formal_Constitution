// SPDX-License-Identifier: Apache-2.0
// Copyright 2025-2026 Ian Farquharson
#![allow(missing_docs)]

use scc_kernel::{
    governance::project_to_simplex,
    state::{l2_drift, valid_simplex},
};

#[test]
fn simplex_projection_stress_respects_error_envelope() {
    for n in 1..=128usize {
        let xs: Vec<f64> = (0..n)
            .map(|i| {
                ((i as f64 + 1.0) * 1.618_033_988_75).sin() * 1.0e6 - ((i + 3) as f64).cos() * 1.0e3
            })
            .collect();
        let projected = project_to_simplex(&xs);
        assert_eq!(projected.len(), n);
        assert!(projected.iter().all(|x| x.is_finite()));
        assert!(projected.iter().all(|x| *x >= -1e-12));
        assert!(
            valid_simplex(&projected, 1e-9),
            "n={n:?} projected={projected:?}"
        );
    }
}

#[test]
fn l2_drift_is_infinite_for_dimension_mismatch() {
    assert!(l2_drift(&[0.5, 0.5], &[1.0]).is_infinite());
}
