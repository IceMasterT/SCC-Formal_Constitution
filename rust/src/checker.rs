// SPDX-License-Identifier: Apache-2.0
// Copyright 2025-2026 Ian Farquharson
//! The SCC proof-obligation acceptance gate.

use serde::{Deserialize, Serialize};

use crate::{
    audit::{audit_chain_valid, AuditEvent},
    env::ExecutionEnv,
    events::{event_hash, CanonicalEvent},
    po::{POBundle, StepMode},
    refinement::{classify_step, expected_lineage_root},
    state::{
        governance_invariant_holds, halt_absorption_holds, is_admissible_domain,
        protected_unchanged, risk_law_holds, state_hash, SCCState,
    },
};

/// Rejection reason. Variants are ordered to match the checker pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckerError {
    /// Schema mismatch for PO, event, or audit event.
    SchemaMismatch,
    /// Checker version mismatch.
    VersionMismatch,
    /// A required boolean PO field was false.
    MissingRequiredField,
    /// Hash verification failed or canonical bytes were malformed.
    HashVerificationFailed,
    /// Unknown step mode.
    InvalidStepMode,
    /// Protected coordinate changed without authorization.
    ProtectedMutation,
    /// Halt absorption or severity monotonicity failed.
    HaltViolation,
    /// Governance was outside simplex or drift bounds.
    GovernanceSimplexViolation,
    /// Risk threshold required halt but next state was not halted.
    RiskThresholdBreach,
    /// Audit event, audit hash, or audit descriptor did not match.
    AuditChainBroken,
    /// Declared step-mode witness did not match the normalized classifier.
    RefinementViolation,
    /// State domain predicate failed.
    DomainViolation,
    /// Lineage was not lawfully extended.
    LineageViolation,
}

/// Pure deterministic acceptance gate for one SCC transition candidate.
pub fn accepts(
    po: &POBundle,
    prev: &SCCState,
    next: &SCCState,
    event: &CanonicalEvent,
    audit_event: &AuditEvent,
    env: &ExecutionEnv,
) -> Result<(), CheckerError> {
    if po.schema_version != env.expected_po_schema
        || event.schema_version != env.expected_event_schema
        || audit_event.schema_version != env.expected_audit_schema
    {
        return Err(CheckerError::SchemaMismatch);
    }
    if po.checker_version != env.checker_version {
        return Err(CheckerError::VersionMismatch);
    }
    if !matches!(
        po.mode,
        StepMode::Exact | StepMode::Stutter | StepMode::SafeRefined
    ) {
        return Err(CheckerError::InvalidStepMode);
    }
    if !po.all_required_true() {
        return Err(CheckerError::MissingRequiredField);
    }

    let prev_hash = state_hash(prev).map_err(|_| CheckerError::HashVerificationFailed)?;
    let next_hash = state_hash(next).map_err(|_| CheckerError::HashVerificationFailed)?;
    let ev_hash = event_hash(event).map_err(|_| CheckerError::HashVerificationFailed)?;
    if po.prev_state_hash != prev_hash
        || po.next_state_hash != next_hash
        || po.event_hash != ev_hash
    {
        return Err(CheckerError::HashVerificationFailed);
    }

    if !is_admissible_domain(prev, env) || !is_admissible_domain(next, env) {
        return Err(CheckerError::DomainViolation);
    }
    if !protected_unchanged(prev, next, po.is_authorized_meta_path()) {
        return Err(CheckerError::ProtectedMutation);
    }
    if !halt_absorption_holds(prev, next, po.is_recovery_path()) {
        return Err(CheckerError::HaltViolation);
    }
    if !governance_invariant_holds(prev, next, env) {
        return Err(CheckerError::GovernanceSimplexViolation);
    }
    if !risk_law_holds(next, env) {
        return Err(CheckerError::RiskThresholdBreach);
    }

    let expected_lineage =
        expected_lineage_root(prev, event).map_err(|_| CheckerError::HashVerificationFailed)?;
    if next.lineage_root != expected_lineage {
        return Err(CheckerError::LineageViolation);
    }

    let audit_ok = audit_chain_valid(po, prev, next, event, audit_event, env)
        .map_err(|_| CheckerError::HashVerificationFailed)?;
    if !audit_ok {
        return Err(CheckerError::AuditChainBroken);
    }

    let classified =
        classify_step(prev, next, event, env).map_err(|_| CheckerError::HashVerificationFailed)?;
    if classified != Some(po.mode) {
        return Err(CheckerError::RefinementViolation);
    }
    Ok(())
}
