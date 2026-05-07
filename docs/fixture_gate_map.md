# Fixture-to-gate map

Each negative-corpus fixture is committed as a JSON record and is expected to reject at the listed first-failure gate. A different first-failure gate should be treated as a regression because it means the checker pipeline changed in an observable way.

| Fixture | Expected first-failure gate | Runtime interpretation |
|---|---|---|
| `schema_mismatch.json` | `schema_mismatch` | Submitted record violates the expected schema family. |
| `checker_version_mismatch.json` | `checker_version_mismatch` | Bundle targets a checker version outside the release contract. |
| `missing_required_field.json` | `missing_required_field` | Proof-obligation bundle lacks required machine-checkable evidence. |
| `prev_hash_bad.json` | `prev_hash_bad` | Previous-state commitment does not match canonical bytes. |
| `next_hash_bad.json` | `next_hash_bad` | Candidate next-state commitment does not match canonical bytes. |
| `event_hash_bad.json` | `event_hash_bad` | Canonical event commitment is corrupted or inconsistent. |
| `audit_event_hash_bad.json` | `audit_event_hash_bad` | Audit event hash does not bind the transition as claimed. |
| `descriptor_mismatch_audit.json` | `descriptor_mismatch_audit` | Execution descriptor in the audit record does not match the transition descriptor. |
| `protected_mutation.json` | `protected_mutation` | Ordinary execution mutated protected coordinates. |
| `halt_absorption_violation.json` | `halt_absorption_violation` | Halted state resumed without authorized recovery. |
| `invalid_simplex.json` | `invalid_simplex` | Governance weights left the valid simplex. |
| `risk_threshold_without_halt.json` | `risk_threshold_without_halt` | Risk threshold was crossed but failure mode did not enter halt. |
| `lineage_erasure.json` | `lineage_erasure` | Runtime history continuation was severed. |
| `wrong_step_mode.json` | `wrong_step_mode` | Declared step mode disagrees with checker classification. |
