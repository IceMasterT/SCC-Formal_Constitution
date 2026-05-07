# Python Scaffold Migration Note

The uploaded Python builder scaffold was treated as a bootstrap/reference artifact, not as the v1 trusted kernel. This repository replaces it with:

- Rust as the source-of-truth checker;
- TypeScript as an independently readable reference checker;
- canonical binary vectors committed under `golden/bin`;
- a negative corpus with named first-failure gates;
- a frozen TCB ledger and release manifest.

The Python scaffold's core idea - checked proof-obligation bundles before accepting a step - is preserved, but runtime authority now lives in the Rust crate.
