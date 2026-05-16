// SPDX-License-Identifier: Apache-2.0
// Copyright 2025-2026 Ian Farquharson
#![allow(missing_docs)]

fn quorum_min_intersection(n: usize, q: usize) -> usize {
    2 * q - n
}

#[test]
fn byzantine_quorum_intersection_has_honest_overlap() {
    for f in 0..=64usize {
        let n = 3 * f + 1;
        let q = 2 * f + 1;
        let overlap = quorum_min_intersection(n, q);
        assert_eq!(overlap, f + 1);
        assert!(overlap > f);
    }
}

#[test]
fn insufficient_quorum_can_be_disjoint_at_the_honest_boundary() {
    for f in 1..=64usize {
        let n = 3 * f + 1;
        let q = 2 * f;
        let overlap = quorum_min_intersection(n, q);
        assert!(overlap <= f);
    }
}
