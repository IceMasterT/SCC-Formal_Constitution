// SPDX-License-Identifier: Apache-2.0
// Copyright 2025-2026 Ian Farquharson
//! Deterministic replay of checked transition records.

use crate::{
    audit::AuditEvent,
    checker::{accepts, CheckerError},
    env::ExecutionEnv,
    events::CanonicalEvent,
    po::POBundle,
    state::SCCState,
};

/// One recorded replay step.
#[derive(Debug, Clone)]
pub struct ReplayStep {
    /// Proof-obligation bundle.
    pub po: POBundle,
    /// Previous state.
    pub prev: SCCState,
    /// Next state.
    pub next: SCCState,
    /// Canonical event.
    pub event: CanonicalEvent,
    /// Audit event.
    pub audit_event: AuditEvent,
}

/// Replay a non-empty step sequence and return the terminal state.
pub fn replay(steps: &[ReplayStep], env: &ExecutionEnv) -> Result<SCCState, CheckerError> {
    let first = steps.first().ok_or(CheckerError::DomainViolation)?;
    let mut current = first.prev.clone();
    for step in steps {
        if step.prev != current {
            return Err(CheckerError::HashVerificationFailed);
        }
        accepts(
            &step.po,
            &step.prev,
            &step.next,
            &step.event,
            &step.audit_event,
            env,
        )?;
        current = step.next.clone();
    }
    Ok(current)
}
