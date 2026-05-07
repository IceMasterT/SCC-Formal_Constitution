SCC Kernel Fortress mechanization-hardening validation transcript
Generated for the NeurIPS 2026 E&D artifact package.

Formatting/build:
  cd paper && latexmk -pdf -interaction=nonstopmode main.tex
  result: OK
  output: main.pdf, 16 pages total
  main content: references begin on page 8; appendices and checklist follow references
  visual check: render_pdf.py rendered 16 pages at 150 dpi
  text check: no unresolved placeholders, no author-identifying strings, no non-anonymous footer

Artifact checks executed locally:
  python3 tools/validate_vectors.py
  result: validate_vectors: OK (17 fixtures)

  python3 tools/forbidden_imports.py
  result: forbidden_imports: OK

  python3 tools/validate_formal_artifacts.py
  result: validate_formal_artifacts: OK (4 proof files, no placeholders)

  cd ts && npm test
  result: tests 22, pass 22, fail 0
  added stress coverage: memory-compression lineage accept, repaired-hash lineage erasure rejection, simplex numerical envelope, federation quorum arithmetic

Rust status in this sandbox:
  cargo and rustc are not installed in this execution environment.
  The paper therefore does not claim a locally observed Rust pass.
  Rust-equipped release validation is supplied through scripts/run_release_gates.sh and the CI workflow.

Proof-assistant status in this sandbox:
  lake/Lean and coqc/Coq are not installed in this execution environment.
  Local formal validation therefore consists of the static no-placeholder/no-unledgered-axiom scan.
  Proof-equipped validation is supplied through scripts/run_formal_gates.sh and the CI formal job.

Release gate supplied for Rust-equipped validation:
  bash scripts/run_release_gates.sh

Formal gate supplied for proof-equipped validation:
  bash scripts/run_formal_gates.sh

CI evidence collector supplied:
  bash scripts/collect_ci_evidence.sh

Docker path supplied:
  docker build -t scc-kernel-fortress .
  docker run --rm scc-kernel-fortress
