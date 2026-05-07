#![allow(missing_docs)]
#![allow(dead_code)]
use std::{collections::BTreeMap, fs, path::PathBuf};

use serde::Deserialize;

use scc_kernel::{AuditEvent, CanonicalEvent, ExecutionEnv, POBundle, SCCState};

#[derive(Debug, Deserialize)]
pub struct Expected {
    pub accepted: bool,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Fixture {
    pub name: String,
    pub env: ExecutionEnv,
    pub prev: SCCState,
    pub event: CanonicalEvent,
    pub next: SCCState,
    pub audit_event: AuditEvent,
    pub po: POBundle,
    pub hashes: BTreeMap<String, String>,
    pub expected: Expected,
}

pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

pub fn read_fixture(path: PathBuf) -> Fixture {
    let src = fs::read_to_string(path).expect("fixture readable");
    serde_json::from_str(&src).expect("fixture parses")
}
