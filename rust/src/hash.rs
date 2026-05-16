// SPDX-License-Identifier: Apache-2.0
// Copyright 2025-2026 Ian Farquharson
//! Domain-separated SHA-256 helpers.

use sha2::{Digest, Sha256};

use crate::canonical::{CanonicalError, CanonicalWriter};

/// Hex-encoded SHA-256 digest.
pub type HashHex = String;

/// The all-zero SHA-256 word used for audit-body hashing.
pub const ZERO_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

/// Validate a 32-byte SHA-256 digest written as 64 hexadecimal characters.
#[must_use]
pub fn is_hash_hex(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|b| b.is_ascii_hexdigit())
}

/// Hash bytes to lowercase hex.
#[must_use]
pub fn sha256_hex(bytes: &[u8]) -> HashHex {
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

/// Deterministically derive a content root from a label and parts.
pub fn derive_root(label: &str, parts: &[&str]) -> Result<HashHex, CanonicalError> {
    let mut w = CanonicalWriter::new();
    w.tag("SCC-DERIVE-v1")?;
    w.string(label)?;
    w.u32(u32::try_from(parts.len()).map_err(|_| CanonicalError::LengthOverflow)?);
    for part in parts {
        if is_hash_hex(part) {
            w.u8(1);
            w.hash_hex(&(*part).to_ascii_lowercase())?;
        } else {
            w.u8(0);
            w.string(part)?;
        }
    }
    Ok(sha256_hex(&w.into_bytes()))
}

/// Deterministically derive the default execution descriptor root used by tests.
pub fn default_descriptor_hash() -> Result<HashHex, CanonicalError> {
    let mut w = CanonicalWriter::new();
    w.tag("SCC-DESCRIPTOR-v1")?;
    for s in [
        "scc-kernel-fortress",
        "scc-checker-1.0.0",
        "scc.state.v1",
        "scc.event.v1",
        "scc.po.v1",
        "scc.audit.v1",
        "deterministic-f64-be-v1",
    ] {
        w.string(s)?;
    }
    Ok(sha256_hex(&w.into_bytes()))
}
