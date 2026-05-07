# Memory, lineage, and compression stress protocol

Memory compression is lawful only when it preserves reconstructible lineage.  Compression is not an excuse to erase ancestry.

## Accepted compression shape

A compression event may rewrite an internal memory representation only if the candidate transition supplies:

1. the prior memory root,
2. the post-compression memory root,
3. a deterministic compression witness identifier,
4. a lineage update derived from the previous lineage root and the compression event hash,
5. a repaired audit event and proof-obligation hash set.

The TypeScript stress test `stress_mechanization.test.ts` constructs this accepted case from the golden exact vector.

## Rejected compression shape

The same transition is rejected at `LineageViolation` when lineage is reset to the previous root after all hashes are repaired.  This makes the failure semantic rather than a superficial hash mismatch.

## Rust stress targets

The Rust suite adds:

- `tests/memory_lineage_compression.rs`
- `fuzz/fuzz_targets/memory_lineage.rs`

These push the seven-stage witness boundary for memory/lineage behavior under compression.
