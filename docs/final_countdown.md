# SCC Kernel Fortress Final Countdown

Run this before every push.

```text
Does this change preserve protected isolation?
Does the checker still reject every negative fixture at the named gate?
Would a tired reviewer understand the evidence in one pass?
Is the TCB smaller or better justified?
Are we closer to mechanized soundness?
```

## Push policy

- Five yes answers: push.
- Any no answer: stop, fix, and rerun the relevant gate.
- Any unverified release claim: downgrade the claim or attach evidence.

## Release policy

A release is not v1.0-ready unless the clean runner transcript contains:

```text
stage0_prelaunch=PASS
release_gate=PASS
rust_gate=PASS
ts_gate=PASS
python_scaffold=PASS
vectors=PASS
negative_corpus=PASS
differential=PASS
manifest_sha256=PASS
signature=PASS
```
