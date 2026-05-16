// SPDX-License-Identifier: Apache-2.0
// Copyright 2025-2026 Ian Farquharson
//! Runtime boundary wrappers. The checker itself remains pure and side-effect free.

use serde::{Deserialize, Serialize};

use crate::{
    audit::AuditEvent,
    checker::{accepts, CheckerError},
    env::ExecutionEnv,
    events::CanonicalEvent,
    po::POBundle,
    state::{state_hash, SCCState},
};

/// Deterministic acceptance summary suitable for storage after the caller commits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StepCommit {
    /// Accepted step id.
    pub step_id: String,
    /// Previous state hash.
    pub prev_state_hash: String,
    /// Next state hash.
    pub next_state_hash: String,
    /// Event hash from PO.
    pub event_hash: String,
    /// Audit event hash from PO.
    pub audit_event_hash: String,
}

/// Evaluate a candidate and return a deterministic commit summary.
pub fn evaluate_candidate(
    po: &POBundle,
    prev: &SCCState,
    next: &SCCState,
    event: &CanonicalEvent,
    audit_event: &AuditEvent,
    env: &ExecutionEnv,
) -> Result<StepCommit, CheckerError> {
    accepts(po, prev, next, event, audit_event, env)?;
    let prev_state_hash = state_hash(prev).map_err(|_| CheckerError::HashVerificationFailed)?;
    let next_state_hash = state_hash(next).map_err(|_| CheckerError::HashVerificationFailed)?;
    Ok(StepCommit {
        step_id: po.step_id.clone(),
        prev_state_hash,
        next_state_hash,
        event_hash: po.event_hash.clone(),
        audit_event_hash: po.audit_event_hash.clone(),
    })
}
