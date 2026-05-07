#![no_main]

use libfuzzer_sys::fuzz_target;
use scc_kernel::{accepts, AuditEvent, CanonicalEvent, ExecutionEnv, POBundle, SCCState};

#[derive(serde::Deserialize)]
struct Candidate {
    po: POBundle,
    prev: SCCState,
    next: SCCState,
    event: CanonicalEvent,
    audit_event: AuditEvent,
    env: ExecutionEnv,
}

fuzz_target!(|data: &[u8]| {
    if let Ok(candidate) = serde_json::from_slice::<Candidate>(data) {
        let _ = accepts(
            &candidate.po,
            &candidate.prev,
            &candidate.next,
            &candidate.event,
            &candidate.audit_event,
            &candidate.env,
        );
    }
});
