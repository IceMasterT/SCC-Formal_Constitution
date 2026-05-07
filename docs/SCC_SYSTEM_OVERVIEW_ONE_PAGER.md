# SCC System Overview - one page

**Canon:** the formal authority.  It defines typed state, constitutional clauses, admissible transitions, proof obligations, audit determinism, lineage, refinement, federation assumptions, mechanization targets, and claim boundaries.

**Engineering Handbook:** the builder discipline.  It tells implementers how to build the small deterministic kernel first, keep product logic outside the checker, generate proof-obligation bundles, preserve audit/lineage, harden the TCB, and ship only with release evidence.

**Kernel Fortress:** the executable boundary artifact.  It checks one candidate transition before state acceptance.  It accepts only when the previous state, candidate next state, canonical event, audit event, proof-obligation bundle, and execution descriptor satisfy the implemented SCC obligations.

**Builder Scaffold:** the starting repo shape.  It gives implementers a minimal SCC runtime skeleton, tests, fixtures, runbooks, manifests, and handoff docs.

**Evidence flow:** Canon clause -> Handbook rule -> Kernel Fortress gate -> Fixture -> Release transcript -> Reviewer claim.

**Non-claim:** SCC artifacts do not by themselves prove general AI safety, compiler correctness, operating-system correctness, SHA-256 security, semantic harmlessness, or every possible implementation's correctness.

**Core sentence:** A system that cannot prove the legality of its next step has no right to take it.
