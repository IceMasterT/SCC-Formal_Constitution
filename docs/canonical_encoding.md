# Canonical Encoding v1

All accepted SCC artifacts are encoded by a domain-separated binary canonical encoder. JSON files in `golden/` are human-readable carriers; the `.bin` files are the byte-level contract.

## Primitive rules

- Every record begins with a UTF-8 domain tag and a u32 byte length.
- Strings are UTF-8 with a u32 byte length.
- `u8`, `u32`, and `u64` are big-endian.
- `f64` values must be finite IEEE-754 binary64, encoded big-endian. Negative zero is normalized to positive zero.
- Hashes are exactly 32 bytes represented as 64 lowercase hexadecimal characters in JSON.
- Maps are sorted by key before encoding.

## Domains

- `SCC-STATE-v1`
- `SCC-EVENT-v1`
- `SCC-AUDIT-EVENT-v1`
- `SCC-PO-BUNDLE-v1`
- `SCC-DERIVE-v1`
- `SCC-DESCRIPTOR-v1`

## Audit body hash

`state_body_hash(next)` is computed by zeroing `next.audit_hash` before canonical state encoding. This prevents a circular audit definition while keeping the installed `next.audit_hash` checked by `audit_chain_valid`.
