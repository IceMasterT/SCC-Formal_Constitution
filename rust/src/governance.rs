//! Governance simplex predicates and projection.

use crate::state::l2_drift;

/// Projects a finite vector onto the probability simplex in O(d log d).
///
/// This is the standard Euclidean projection: sort descending, find the threshold,
/// threshold at zero, then renormalize against numerical roundoff.
#[must_use]
pub fn project_to_simplex(weights: &[f64]) -> Vec<f64> {
    if weights.iter().any(|x| !x.is_finite()) {
        return Vec::new();
    }
    let d = weights.len();
    if d == 0 {
        return Vec::new();
    }
    let mut sorted = weights.to_vec();
    sorted.sort_by(|a, b| b.total_cmp(a));

    let mut cssv = 0.0;
    let mut rho = 0usize;
    for (i, value) in sorted.iter().enumerate() {
        cssv += *value;
        let theta = (cssv - 1.0) / (i as f64 + 1.0);
        if i == d - 1 || theta >= sorted[i + 1] {
            rho = i;
            break;
        }
    }
    let theta = (sorted.iter().take(rho + 1).sum::<f64>() - 1.0) / (rho as f64 + 1.0);
    let mut out: Vec<f64> = weights.iter().map(|w| (*w - theta).max(0.0)).collect();
    let total: f64 = out.iter().sum();
    if total > 1e-12 {
        for w in &mut out {
            *w /= total;
        }
    } else {
        let uniform = 1.0 / d as f64;
        out.fill(uniform);
    }
    out
}

/// Project a proposal and return the L2 drift from the previous governance vector.
#[must_use]
pub fn project_to_simplex_with_drift(prev: &[f64], proposal: &[f64]) -> (Vec<f64>, f64) {
    let projected = project_to_simplex(proposal);
    let drift = l2_drift(prev, &projected);
    (projected, drift)
}
