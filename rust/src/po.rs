// SPDX-License-Identifier: Apache-2.0
// Copyright 2025-2026 Ian Farquharson
//! Proof-obligation bundle model.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    canonical::{CanonicalBytes, CanonicalError, CanonicalWriter},
    hash::{sha256_hex, HashHex},
};

/// Checked step-mode witness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum StepMode {
    /// Normalized next state equals the induced abstract successor.
    Exact,
    /// Normalized state is unchanged while administrative coordinates advance.
    Stutter,
    /// Normalized next state is no less safe than the induced abstract successor.
    SafeRefined,
}

/// Typed proof-obligation bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct POBundle {
    /// PO schema identifier.
    pub schema_version: String,
    /// Checker version expected by this bundle.
    pub checker_version: String,
    /// Step identifier.
    pub step_id: String,
    /// Step classification.
    pub mode: StepMode,
    /// Domain witness.
    pub dom: bool,
    /// Hard invariant witness.
    pub inv: bool,
    /// Identity continuity witness.
    pub identity: bool,
    /// Governance witness.
    pub governance: bool,
    /// Risk witness.
    pub risk: bool,
    /// Audit witness.
    pub audit: bool,
    /// Protected-coordinate isolation witness.
    pub isolation: bool,
    /// Refinement witness.
    pub refinement: bool,
    /// Previous state hash.
    pub prev_state_hash: HashHex,
    /// Next state hash.
    pub next_state_hash: HashHex,
    /// Event hash.
    pub event_hash: HashHex,
    /// Audit event hash.
    pub audit_event_hash: HashHex,
    /// Authorized protected-coordinate meta-path flag.
    pub authorized_meta_path: bool,
    /// Authorized recovery path flag.
    pub recovery_path: bool,
    /// Stable witness references.
    pub witness_refs: BTreeMap<String, String>,
}

impl POBundle {
    /// Required PO booleans must all be true.
    #[must_use]
    pub fn all_required_true(&self) -> bool {
        self.dom
            && self.inv
            && self.identity
            && self.governance
            && self.risk
            && self.audit
            && self.isolation
            && self.refinement
    }

    /// Whether the bundle declares a protected-coordinate meta-path.
    #[must_use]
    pub fn is_authorized_meta_path(&self) -> bool {
        self.authorized_meta_path
    }

    /// Whether the bundle declares an authorized halt-recovery path.
    #[must_use]
    pub fn is_recovery_path(&self) -> bool {
        self.recovery_path
    }
}

impl CanonicalBytes for POBundle {
    fn canonical_bytes(&self) -> Result<Vec<u8>, CanonicalError> {
        let mut w = CanonicalWriter::new();
        w.tag("SCC-PO-BUNDLE-v1")?;
        w.string(&self.schema_version)?;
        w.string(&self.checker_version)?;
        w.string(&self.step_id)?;
        let mode = match self.mode {
            StepMode::Exact => "Exact",
            StepMode::Stutter => "Stutter",
            StepMode::SafeRefined => "SafeRefined",
        };
        w.string(mode)?;
        for b in [
            self.dom,
            self.inv,
            self.identity,
            self.governance,
            self.risk,
            self.audit,
            self.isolation,
            self.refinement,
        ] {
            w.bool(b);
        }
        w.hash_hex(&self.prev_state_hash)?;
        w.hash_hex(&self.next_state_hash)?;
        w.hash_hex(&self.event_hash)?;
        w.hash_hex(&self.audit_event_hash)?;
        w.bool(self.authorized_meta_path);
        w.bool(self.recovery_path);
        w.string_map(&self.witness_refs)?;
        Ok(w.into_bytes())
    }
}

/// Hash a proof-obligation bundle.
pub fn po_hash(po: &POBundle) -> Result<HashHex, CanonicalError> {
    Ok(sha256_hex(&po.canonical_bytes()?))
}
