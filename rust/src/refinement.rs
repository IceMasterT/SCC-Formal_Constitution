// SPDX-License-Identifier: Apache-2.0
// Copyright 2025-2026 Ian Farquharson
//! Normalized simulation projection and step classification.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    env::ExecutionEnv,
    events::{event_hash, CanonicalEvent},
    hash::{derive_root, HashHex},
    po::StepMode,
    state::SCCState,
};

/// Normalized projection used for stutter and refinement classification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NormalizedState {
    /// Compute root.
    pub compute_root: HashHex,
    /// Memory root.
    pub memory_root: HashHex,
    /// Vector clock.
    pub vector_clock: BTreeMap<String, u64>,
    /// Governance weights.
    pub governance_weights: Vec<f64>,
    /// Failure mode.
    pub failure_mode: u8,
    /// Risk vector.
    pub risk_vector: Vec<f64>,
    /// Protected coordinate root.
    pub protected_root: HashHex,
}

/// Compute the normalized projection.
#[must_use]
pub fn normalized(state: &SCCState) -> NormalizedState {
    NormalizedState {
        compute_root: state.compute_root.clone(),
        memory_root: state.memory_root.clone(),
        vector_clock: state.vector_clock.clone(),
        governance_weights: state.governance_weights.clone(),
        failure_mode: state.failure_mode,
        risk_vector: state.risk_vector.clone(),
        protected_root: state.protected_root.clone(),
    }
}

fn close_vec(a: &[f64], b: &[f64], eps: f64) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| (*x - *y).abs() <= eps)
}

/// Tolerant equality on normalized states.
#[must_use]
pub fn normalized_equal(a: &NormalizedState, b: &NormalizedState, eps: f64) -> bool {
    a.compute_root == b.compute_root
        && a.memory_root == b.memory_root
        && a.vector_clock == b.vector_clock
        && close_vec(&a.governance_weights, &b.governance_weights, eps)
        && a.failure_mode == b.failure_mode
        && close_vec(&a.risk_vector, &b.risk_vector, eps)
        && a.protected_root == b.protected_root
}

/// Induced exact successor on normalized coordinates.
pub fn expected_next_normalized(
    prev: &SCCState,
    event: &CanonicalEvent,
    env: &ExecutionEnv,
) -> Result<NormalizedState, crate::canonical::CanonicalError> {
    let mut next = normalized(prev);
    next.compute_root = derive_root("compute", &[&prev.compute_root, &event.payload_hash])?;
    *next.vector_clock.entry(event.producer.clone()).or_insert(0) += 1;
    if event.kind == "risk_signal" && !next.risk_vector.is_empty() {
        next.risk_vector[0] = (next.risk_vector[0] + 0.1).min(env.max_risk_value);
    }
    let risk: f64 = next.risk_vector.iter().sum();
    if risk >= env.risk_halt_threshold {
        next.failure_mode = 3;
    }
    Ok(next)
}

/// Declared safety preorder for v1: stable structural coordinates, no more total risk, no lower failure mode.
#[must_use]
pub fn safe_refined(next: &NormalizedState, exact: &NormalizedState, eps: f64) -> bool {
    let stable = next.compute_root == exact.compute_root
        && next.memory_root == exact.memory_root
        && next.vector_clock == exact.vector_clock
        && close_vec(&next.governance_weights, &exact.governance_weights, eps)
        && next.protected_root == exact.protected_root;
    let risk_next: f64 = next.risk_vector.iter().sum();
    let risk_exact: f64 = exact.risk_vector.iter().sum();
    stable && risk_next <= risk_exact + eps && next.failure_mode >= exact.failure_mode
}

/// Classify one step as Exact, Stutter, SafeRefined, or rejected.
pub fn classify_step(
    prev: &SCCState,
    next: &SCCState,
    event: &CanonicalEvent,
    env: &ExecutionEnv,
) -> Result<Option<StepMode>, crate::canonical::CanonicalError> {
    let p = normalized(prev);
    let n = normalized(next);
    let exact = expected_next_normalized(prev, event, env)?;
    if normalized_equal(&n, &exact, env.simplex_epsilon) {
        return Ok(Some(StepMode::Exact));
    }
    if normalized_equal(&n, &p, env.simplex_epsilon) {
        return Ok(Some(StepMode::Stutter));
    }
    if safe_refined(&n, &exact, env.simplex_epsilon) {
        return Ok(Some(StepMode::SafeRefined));
    }
    Ok(None)
}

/// Expected lineage extension for any lawful implementation step.
pub fn expected_lineage_root(
    prev: &SCCState,
    event: &CanonicalEvent,
) -> Result<HashHex, crate::canonical::CanonicalError> {
    derive_root("lineage", &[&prev.lineage_root, &event_hash(event)?])
}
