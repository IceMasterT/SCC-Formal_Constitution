// SPDX-License-Identifier: Apache-2.0
// Copyright 2025-2026 Ian Farquharson
#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! SCC executable verification kernel.
//!
//! The library is intentionally small: canonical encoders, typed state/event/PO records,
//! deterministic hash functions, a pure checker, replay, and deterministic transition helpers.

pub mod audit;
pub mod canonical;
pub mod checker;
pub mod env;
pub mod events;
pub mod governance;
pub mod hash;
pub mod po;
pub mod refinement;
pub mod replay;
pub mod runtime;
pub mod state;
pub mod transition;

pub use audit::AuditEvent;
pub use checker::{accepts, CheckerError};
pub use env::ExecutionEnv;
pub use events::CanonicalEvent;
pub use po::{POBundle, StepMode};
pub use state::SCCState;
