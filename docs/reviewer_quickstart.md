# Reviewer quickstart

This supplement is an executable evaluation artifact for transition-level runtime compliance. It is not a model, training dataset, or leaderboard.

## Full release gate

Run from the repository root on a CPU-only machine with Python 3, Node 22+, and Rust 1.78+:

```bash
bash scripts/run_release_gates.sh
```

Equivalent one-command Docker path:

```bash
docker build -t scc-kernel-fortress .
docker run --rm scc-kernel-fortress
```

## Quick non-Rust inspection path

When Rust is unavailable, reviewers can still inspect the fixture corpus and TypeScript reference checker:

```bash
python3 tools/validate_vectors.py
python3 tools/forbidden_imports.py
cd ts && npm test
```

This quick path is useful for package inspection, but the release claim is locked only when the Rust release gate or included CI workflow also passes.

## What passing means

A passing full release gate means:

- three golden modes (`Exact`, `Stutter`, `SafeRefined`) accept;
- fourteen negative-corpus fixtures reject at their named first-failure gates;
- replay determinism is preserved by the TypeScript reference checker;
- forbidden runtime imports are absent from the checker boundary;
- the Rust source-of-truth tests, negative corpus, golden vectors, and property tests pass.
