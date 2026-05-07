# Federation evidence protocol

Kernel Fortress remains single-transition first.  Federation is added as an evidence layer, not as a replacement for the single-node gate.

## Rule F1: certificate shape

A federated certificate binds `{shard, height, state_root, signer_set, descriptor_hash}`.  The signer set must have size at least `q = 2f + 1` in a committee of size `n = 3f + 1`.

## Rule F2: quorum intersection

Two certificates at the same shard and height with conflicting roots are rejected when both satisfy the quorum rule.  The arithmetic proof is included in `formal/coq/SCCKernelFortress.v`: two quorums of size `2f + 1` inside `3f + 1` nodes overlap in at least `f + 1` nodes, so at least one honest node would have signed both conflicting roots.

## Rule F3: cross-shard lineage

Cross-shard messages must carry the source shard lineage root and certificate.  The target shard must bind that root into its next event or reject the transition as a lineage gap.

## Rule F4: bounded halt propagation

A shard entering failure mode 3 must publish a halt certificate.  Dependent shards must either absorb the halt within the declared descriptor bound or reject subsequent ordinary transitions that depend on the halted shard.

## Review fixtures

The `federation/` directory includes four reviewer fixtures: valid quorum certificate, conflicting quorum certificate, cross-shard lineage break, and bounded halt propagation.  These fixtures are intentionally separate from the single-node negative corpus so the single-node checker contract stays stable while the distributed story becomes executable and reviewable.
