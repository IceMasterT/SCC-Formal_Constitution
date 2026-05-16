// SPDX-License-Identifier: Apache-2.0
// Copyright 2025-2026 Ian Farquharson
#![allow(missing_docs)]
mod common;

use glob::glob;
use scc_kernel::accepts;

use common::{read_fixture, repo_root};

#[test]
fn every_negative_corpus_case_rejects_at_expected_gate() {
    for path in glob(
        repo_root()
            .join("negative_corpus/cases/*.json")
            .to_str()
            .unwrap(),
    )
    .unwrap()
    {
        let fx = read_fixture(path.unwrap());
        let err = accepts(
            &fx.po,
            &fx.prev,
            &fx.next,
            &fx.event,
            &fx.audit_event,
            &fx.env,
        )
        .expect_err("negative corpus case must reject");
        assert_eq!(Some(format!("{:?}", err)), fx.expected.error, "{}", fx.name);
    }
}
