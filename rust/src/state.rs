// SPDX-License-Identifier: Apache-2.0
// Copyright 2025-2026 Ian Farquharson
//! State data model and state-level invariants.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    canonical::{CanonicalBytes, CanonicalError, CanonicalWriter},
    env::ExecutionEnv,
    hash::{sha256_hex, HashHex, ZERO_HASH},
};

/// Concrete SCC kernel state used by the executable checker.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SCCState {
    /// State schema identifier.
    pub schema_version: String,
    /// Human-stable state identifier.
    pub state_id: String,
    /// Monotone implementation step number.
    pub step: u64,
    /// Content root for compute state.
    pub compute_root: HashHex,
    /// Content root for memory state.
    pub memory_root: HashHex,
    /// Per-producer vector clock.
    pub vector_clock: BTreeMap<String, u64>,
    /// Governance simplex coordinate.
    pub governance_weights: Vec<f64>,
    /// Failure mode in {0,1,2,3}; 3 is halt.
    pub failure_mode: u8,
    /// Audit hash coordinate.
    pub audit_hash: HashHex,
    /// Risk vector.
    pub risk_vector: Vec<f64>,
    /// Lineage content root.
    pub lineage_root: HashHex,
    /// Protected coordinate root.
    pub protected_root: HashHex,
}

impl CanonicalBytes for SCCState {
    fn canonical_bytes(&self) -> Result<Vec<u8>, CanonicalError> {
        state_bytes(self, true)
    }
}

/// Canonical state bytes, optionally including the audit-hash coordinate.
pub fn state_bytes(state: &SCCState, include_audit: bool) -> Result<Vec<u8>, CanonicalError> {
    let mut w = CanonicalWriter::new();
    w.tag("SCC-STATE-v1")?;
    w.string(&state.schema_version)?;
    w.string(&state.state_id)?;
    w.u64(state.step);
    w.hash_hex(&state.compute_root)?;
    w.hash_hex(&state.memory_root)?;
    w.vector_clock(&state.vector_clock)?;
    w.f64_vec(&state.governance_weights)?;
    w.u8(state.failure_mode);
    let audit = if include_audit {
        &state.audit_hash
    } else {
        ZERO_HASH
    };
    w.hash_hex(&audit.to_string())?;
    w.f64_vec(&state.risk_vector)?;
    w.hash_hex(&state.lineage_root)?;
    w.hash_hex(&state.protected_root)?;
    Ok(w.into_bytes())
}

/// Hash the full state.
pub fn state_hash(state: &SCCState) -> Result<HashHex, CanonicalError> {
    Ok(sha256_hex(&state_bytes(state, true)?))
}

/// Hash the state with audit_hash zeroed. Used by the audit event to avoid recursion.
pub fn state_body_hash(state: &SCCState) -> Result<HashHex, CanonicalError> {
    Ok(sha256_hex(&state_bytes(state, false)?))
}

/// Valid failure mode predicate.
#[must_use]
pub fn valid_failure_mode(mode: u8) -> bool {
    matches!(mode, 0..=3)
}

/// Check finite vector with optional non-empty requirement.
#[must_use]
pub fn finite_vector(xs: &[f64], non_empty: bool) -> bool {
    (!non_empty || !xs.is_empty()) && xs.iter().all(|x| x.is_finite())
}

/// Simplex predicate with tolerance.
#[must_use]
pub fn valid_simplex(weights: &[f64], eps: f64) -> bool {
    if !finite_vector(weights, true) {
        return false;
    }
    let sum: f64 = weights.iter().sum();
    weights.iter().all(|w| *w >= -eps) && (sum - 1.0).abs() <= eps
}

/// L2 drift between two equally sized vectors.
#[must_use]
pub fn l2_drift(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() {
        return f64::INFINITY;
    }
    a.iter()
        .zip(b)
        .map(|(x, y)| (*x - *y) * (*x - *y))
        .sum::<f64>()
        .sqrt()
}

/// Risk vector domain predicate.
#[must_use]
pub fn valid_risk_vector(xs: &[f64], max_value: f64) -> bool {
    finite_vector(xs, true) && xs.iter().all(|x| *x >= 0.0 && *x <= max_value)
}

/// Full state-domain predicate checked by `accepts`.
#[must_use]
pub fn is_admissible_domain(state: &SCCState, env: &ExecutionEnv) -> bool {
    state.schema_version == env.expected_state_schema
        && valid_failure_mode(state.failure_mode)
        && valid_simplex(&state.governance_weights, env.simplex_epsilon)
        && valid_risk_vector(&state.risk_vector, env.max_risk_value)
}

/// Protected coordinate non-interference predicate.
#[must_use]
pub fn protected_unchanged(prev: &SCCState, next: &SCCState, authorized: bool) -> bool {
    authorized || prev.protected_root == next.protected_root
}

/// Halt absorption and non-decreasing severity for ordinary execution.
#[must_use]
pub fn halt_absorption_holds(prev: &SCCState, next: &SCCState, recovery: bool) -> bool {
    if prev.failure_mode == 3 && !recovery {
        return next.failure_mode == 3;
    }
    recovery || next.failure_mode >= prev.failure_mode
}

/// Governance simplex and bounded-drift check.
#[must_use]
pub fn governance_invariant_holds(prev: &SCCState, next: &SCCState, env: &ExecutionEnv) -> bool {
    valid_simplex(&next.governance_weights, env.simplex_epsilon)
        && l2_drift(&prev.governance_weights, &next.governance_weights)
            <= env.max_governance_drift + env.simplex_epsilon
}

/// Risk-to-halt law.
#[must_use]
pub fn risk_law_holds(state: &SCCState, env: &ExecutionEnv) -> bool {
    let risk: f64 = state.risk_vector.iter().sum();
    risk < env.risk_halt_threshold || state.failure_mode == 3
}
