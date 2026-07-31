// SPDX-License-Identifier: Apache-2.0
// Copyright 2025-2026 Ian Farquharson
//! Execution descriptor and checker environment.

use serde::{Deserialize, Serialize};

use crate::hash::{default_descriptor_hash, HashHex};

/// The only numerical mode implemented by this checker version.
pub const SUPPORTED_NUMERICAL_MODE: &str = "deterministic-f64-be-v1";

/// Explicit environment against which proof-obligation bundles are interpreted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionEnv {
    /// Required PO schema.
    pub expected_po_schema: String,
    /// Required state schema.
    pub expected_state_schema: String,
    /// Required event schema.
    pub expected_event_schema: String,
    /// Required audit schema.
    pub expected_audit_schema: String,
    /// Required checker version.
    pub checker_version: String,
    /// Content root of the execution descriptor.
    pub descriptor_hash: HashHex,
    /// Declared numerical encoding mode.
    pub numerical_mode: String,
    /// Simplex and refinement tolerance.
    pub simplex_epsilon: f64,
    /// Sum risk threshold that mandates halt.
    pub risk_halt_threshold: f64,
    /// Maximum ordinary L2 governance drift.
    pub max_governance_drift: f64,
    /// Maximum coordinate risk value.
    pub max_risk_value: f64,
}

impl ExecutionEnv {
    /// Return the v1 golden-vector descriptor.
    #[must_use]
    pub fn golden_v1() -> Self {
        Self {
            expected_po_schema: "scc.po.v1".to_string(),
            expected_state_schema: "scc.state.v1".to_string(),
            expected_event_schema: "scc.event.v1".to_string(),
            expected_audit_schema: "scc.audit.v1".to_string(),
            checker_version: "scc-checker-1.0.0".to_string(),
            descriptor_hash: default_descriptor_hash().expect("static descriptor hash encodes"),
            numerical_mode: SUPPORTED_NUMERICAL_MODE.to_string(),
            simplex_epsilon: 1e-9,
            risk_halt_threshold: 1.0,
            max_governance_drift: 0.25,
            max_risk_value: 1.0,
        }
    }
}
