//! Audit event model and deterministic audit-chain verification.

use serde::{Deserialize, Serialize};

use crate::{
    canonical::{CanonicalBytes, CanonicalError, CanonicalWriter},
    env::ExecutionEnv,
    events::{event_hash, CanonicalEvent},
    hash::{sha256_hex, HashHex, ZERO_HASH},
    po::POBundle,
    state::{state_body_hash, state_hash, SCCState},
};

/// Canonical audit event for one accepted step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditEvent {
    /// Audit schema identifier.
    pub schema_version: String,
    /// Step identifier.
    pub step_id: String,
    /// Previous audit hash coordinate.
    pub prev_audit_hash: HashHex,
    /// Hash of the canonical event.
    pub event_hash: HashHex,
    /// Hash of previous full state.
    pub prev_state_hash: HashHex,
    /// Hash of next state with audit hash zeroed.
    pub next_state_body_hash: HashHex,
    /// Checker version.
    pub checker_version: String,
    /// Execution descriptor hash.
    pub descriptor_hash: HashHex,
}

impl CanonicalBytes for AuditEvent {
    fn canonical_bytes(&self) -> Result<Vec<u8>, CanonicalError> {
        let mut w = CanonicalWriter::new();
        w.tag("SCC-AUDIT-EVENT-v1")?;
        w.string(&self.schema_version)?;
        w.string(&self.step_id)?;
        w.hash_hex(&self.prev_audit_hash)?;
        w.hash_hex(&self.event_hash)?;
        w.hash_hex(&self.prev_state_hash)?;
        w.hash_hex(&self.next_state_body_hash)?;
        w.string(&self.checker_version)?;
        w.hash_hex(&self.descriptor_hash)?;
        Ok(w.into_bytes())
    }
}

/// Hash an audit event.
pub fn audit_event_hash(event: &AuditEvent) -> Result<HashHex, CanonicalError> {
    Ok(sha256_hex(&event.canonical_bytes()?))
}

/// Build the canonical audit event the checker expects.
pub fn expected_audit_event(
    po: &POBundle,
    prev: &SCCState,
    next: &SCCState,
    event: &CanonicalEvent,
    env: &ExecutionEnv,
) -> Result<AuditEvent, CanonicalError> {
    let mut next_body = next.clone();
    next_body.audit_hash = ZERO_HASH.to_string();
    Ok(AuditEvent {
        schema_version: env.expected_audit_schema.clone(),
        step_id: po.step_id.clone(),
        prev_audit_hash: prev.audit_hash.clone(),
        event_hash: event_hash(event)?,
        prev_state_hash: state_hash(prev)?,
        next_state_body_hash: state_body_hash(&next_body)?,
        checker_version: po.checker_version.clone(),
        descriptor_hash: env.descriptor_hash.clone(),
    })
}

/// Verify the audit event and next-state audit hash.
pub fn audit_chain_valid(
    po: &POBundle,
    prev: &SCCState,
    next: &SCCState,
    event: &CanonicalEvent,
    audit_event: &AuditEvent,
    env: &ExecutionEnv,
) -> Result<bool, CanonicalError> {
    let expected = expected_audit_event(po, prev, next, event, env)?;
    if audit_event != &expected {
        return Ok(false);
    }
    let audit_hash = audit_event_hash(audit_event)?;
    Ok(po.audit_event_hash == audit_hash && next.audit_hash == audit_hash)
}
