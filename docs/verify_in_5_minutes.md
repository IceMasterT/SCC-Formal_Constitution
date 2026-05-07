# Verify in 5 Minutes

This is the public-repo reviewer path. It is intentionally short.

## Docker path

```bash
docker build -t scc-kernel-fortress .
docker run --rm scc-kernel-fortress
```

Expected final line on a release-ready tag:

```text
release_gate=PASS
```

## Native path

```bash
bash scripts/verify_in_5_minutes.sh
```

Expected final line on a release-ready tag:

```text
verify_in_5_minutes=PASS
```

## What this verifies

- vector manifest integrity;
- forbidden-import discipline;
- TypeScript reference checker behavior;
- Rust source-of-truth checker tests and clippy gate, when Rust is installed;
- fixture first-failure expectations;
- release-manifest regeneration;
- formal bridge static scan;
- optional proof-assistant gates when Lean/Coq are installed.

## What this does not verify

It does not prove general AI safety, compiler correctness, operating-system correctness, cryptographic primitive security, semantic harmlessness, or full mechanization of the complete SCC Canon.
