// SPDX-License-Identifier: Apache-2.0
// Copyright 2025-2026 Ian Farquharson
//! Canonical binary encoding primitives.
//!
//! The encoding is domain-separated, deterministic, and independent of JSON key order.
//! Floating point values are encoded as finite IEEE-754 binary64 in big-endian order;
//! negative zero is normalized to positive zero before encoding.

use std::collections::BTreeMap;

use thiserror::Error;

use crate::hash::HashHex;

/// Error returned by canonical encoders.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CanonicalError {
    /// A length did not fit into the declared canonical width.
    #[error("length exceeds u32 range")]
    LengthOverflow,
    /// A floating point field was NaN or infinite.
    #[error("non-finite f64 encountered")]
    NonFiniteFloat,
    /// A purported SHA-256 hex string was malformed.
    #[error("invalid hash hex")]
    InvalidHashHex,
}

/// Trait for deterministic canonical byte encoders.
pub trait CanonicalBytes {
    /// Returns the canonical byte representation.
    fn canonical_bytes(&self) -> Result<Vec<u8>, CanonicalError>;
}

/// Append-only canonical writer.
#[derive(Debug, Default, Clone)]
pub struct CanonicalWriter {
    bytes: Vec<u8>,
}

impl CanonicalWriter {
    /// Create an empty writer.
    #[must_use]
    pub fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    /// Return the written bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Append raw bytes.
    pub fn raw(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }

    /// Append a u8.
    pub fn u8(&mut self, x: u8) {
        self.bytes.push(x);
    }

    /// Append a canonical bool.
    pub fn bool(&mut self, x: bool) {
        self.u8(u8::from(x));
    }

    /// Append a u32 in network byte order.
    pub fn u32(&mut self, x: u32) {
        self.bytes.extend_from_slice(&x.to_be_bytes());
    }

    /// Append a u64 in network byte order.
    pub fn u64(&mut self, x: u64) {
        self.bytes.extend_from_slice(&x.to_be_bytes());
    }

    /// Append a UTF-8 string with a u32 length prefix.
    pub fn string(&mut self, s: &str) -> Result<(), CanonicalError> {
        let len = u32::try_from(s.len()).map_err(|_| CanonicalError::LengthOverflow)?;
        self.u32(len);
        self.raw(s.as_bytes());
        Ok(())
    }

    /// Append a domain tag.
    pub fn tag(&mut self, tag: &str) -> Result<(), CanonicalError> {
        self.string(tag)
    }

    /// Append a 32-byte hash decoded from hex.
    pub fn hash_hex(&mut self, h: &HashHex) -> Result<(), CanonicalError> {
        if h.len() != 64 || !h.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(CanonicalError::InvalidHashHex);
        }
        let bytes = hex::decode(h).map_err(|_| CanonicalError::InvalidHashHex)?;
        if bytes.len() != 32 {
            return Err(CanonicalError::InvalidHashHex);
        }
        self.raw(&bytes);
        Ok(())
    }

    /// Append finite f64 in canonical big-endian representation.
    pub fn f64(&mut self, x: f64) -> Result<(), CanonicalError> {
        if !x.is_finite() {
            return Err(CanonicalError::NonFiniteFloat);
        }
        let y = if x == 0.0 { 0.0 } else { x };
        self.raw(&y.to_bits().to_be_bytes());
        Ok(())
    }

    /// Append a vector of finite f64 values.
    pub fn f64_vec(&mut self, xs: &[f64]) -> Result<(), CanonicalError> {
        let len = u32::try_from(xs.len()).map_err(|_| CanonicalError::LengthOverflow)?;
        self.u32(len);
        for x in xs {
            self.f64(*x)?;
        }
        Ok(())
    }

    /// Append a sorted string map.
    pub fn string_map(&mut self, m: &BTreeMap<String, String>) -> Result<(), CanonicalError> {
        let len = u32::try_from(m.len()).map_err(|_| CanonicalError::LengthOverflow)?;
        self.u32(len);
        for (k, v) in m {
            self.string(k)?;
            self.string(v)?;
        }
        Ok(())
    }

    /// Append a vector clock in sorted key order.
    pub fn vector_clock(&mut self, m: &BTreeMap<String, u64>) -> Result<(), CanonicalError> {
        let len = u32::try_from(m.len()).map_err(|_| CanonicalError::LengthOverflow)?;
        self.u32(len);
        for (k, v) in m {
            self.string(k)?;
            self.u64(*v);
        }
        Ok(())
    }
}
