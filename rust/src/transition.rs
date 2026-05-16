// SPDX-License-Identifier: Apache-2.0
// Copyright 2025-2026 Ian Farquharson
//! Deterministic transition helpers for tests and fixture generation.

use crate::{
    env::ExecutionEnv,
    events::{event_hash, CanonicalEvent},
    hash::{derive_root, ZERO_HASH},
    po::StepMode,
    state::SCCState,
};

/// Compute a candidate next state before the audit hash is installed.
pub fn candidate_next(
    prev: &SCCState,
    event: &CanonicalEvent,
    mode: StepMode,
    env: &ExecutionEnv,
) -> Result<SCCState, crate::canonical::CanonicalError> {
    let mut next = prev.clone();
    next.state_id = format!(
        "state:{}:{}",
        prev.step + 1,
        mode_name(mode).to_ascii_lowercase()
    );
    next.step = prev.step + 1;
    if mode != StepMode::Stutter {
        next.compute_root = derive_root("compute", &[&prev.compute_root, &event.payload_hash])?;
        *next.vector_clock.entry(event.producer.clone()).or_insert(0) += 1;
        if event.kind == "risk_signal" && !next.risk_vector.is_empty() {
            next.risk_vector[0] = (next.risk_vector[0] + 0.1).min(env.max_risk_value);
        }
        if mode == StepMode::SafeRefined {
            for risk in &mut next.risk_vector {
                *risk = (*risk - 0.05).max(0.0);
            }
        }
    }
    if next.risk_vector.iter().sum::<f64>() >= env.risk_halt_threshold {
        next.failure_mode = 3;
    }
    next.lineage_root = derive_root("lineage", &[&prev.lineage_root, &event_hash(event)?])?;
    next.audit_hash = ZERO_HASH.to_string();
    Ok(next)
}

fn mode_name(mode: StepMode) -> &'static str {
    match mode {
        StepMode::Exact => "exact",
        StepMode::Stutter => "stutter",
        StepMode::SafeRefined => "safe_refined",
    }
}
