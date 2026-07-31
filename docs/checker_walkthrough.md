# Checker Walkthrough

`accepts` is intentionally linear. The order is part of the API because negative corpus cases assert the first rejection gate.

1. **Schema discipline**: PO, event, and audit schemas must match the execution descriptor.
2. **Version discipline**: the PO checker version must equal the descriptor checker version, and the descriptor numerical mode must be the supported `deterministic-f64-be-v1`.
3. **Step mode**: the declared PO mode must be a recognized mode string (`Exact`, `Stutter`, `SafeRefined`).
4. **Required witnesses**: all core PO booleans must be true.
5. **Canonical hashes**: previous state, next state, and event hashes must recompute exactly.
6. **Domain**: both states must satisfy schema, failure-mode, simplex, and risk-vector predicates.
7. **Protected coordinate**: protected root is immutable unless an authorized meta-path flag is present.
8. **Halt absorption**: failure mode 3 stays 3 absent recovery; ordinary severity cannot decrease.
9. **Governance law**: next weights remain in the simplex and within the ordinary drift bound.
10. **Risk law**: threshold breach requires halt mode.
11. **Lineage**: the step counter must advance by exactly one, the event must declare that same step, and the next lineage root must equal the deterministic extension of previous lineage and event hash.
12. **Audit chain**: audit event fields, audit event hash, and next-state audit coordinate must all match.
13. **Refinement**: the normalized classifier must equal the declared PO mode.

No branch reads time, randomness, network, environment variables, mutable global state, or product runtime outputs.
