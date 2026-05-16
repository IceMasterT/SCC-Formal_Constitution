// SPDX-License-Identifier: Apache-2.0
// Copyright 2025-2026 Ian Farquharson
//! Canonical event model.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    canonical::{CanonicalBytes, CanonicalError, CanonicalWriter},
    hash::{sha256_hex, HashHex},
};

/// Canonical event supplied to an SCC transition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalEvent {
    /// Event schema identifier.
    pub schema_version: String,
    /// Event id.
    pub event_id: String,
    /// Event kind.
    pub kind: String,
    /// Producer identifier.
    pub producer: String,
    /// Declared step.
    pub step: u64,
    /// Parent event or run hash.
    pub parent_hash: HashHex,
    /// Payload content root.
    pub payload_hash: HashHex,
    /// Stable string attributes.
    pub attributes: BTreeMap<String, String>,
}

impl CanonicalBytes for CanonicalEvent {
    fn canonical_bytes(&self) -> Result<Vec<u8>, CanonicalError> {
        let mut w = CanonicalWriter::new();
        w.tag("SCC-EVENT-v1")?;
        w.string(&self.schema_version)?;
        w.string(&self.event_id)?;
        w.string(&self.kind)?;
        w.string(&self.producer)?;
        w.u64(self.step);
        w.hash_hex(&self.parent_hash)?;
        w.hash_hex(&self.payload_hash)?;
        w.string_map(&self.attributes)?;
        Ok(w.into_bytes())
    }
}

/// Hash a canonical event.
pub fn event_hash(event: &CanonicalEvent) -> Result<HashHex, CanonicalError> {
    Ok(sha256_hex(&event.canonical_bytes()?))
}
