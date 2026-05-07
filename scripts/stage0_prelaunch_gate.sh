#!/usr/bin/env bash
set -euo pipefail

echo "SCC Kernel Fortress Stage 0 prelaunch gate"
echo "working_dir=$(pwd)"

need() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "missing required command: $1" >&2
    exit 1
  fi
}

need python3
need npm
need cargo
need rustc
need docker
need sha256sum

python3 tools/validate_vectors.py
python3 tools/forbidden_imports.py
python3 tools/validate_formal_artifacts.py
(cd ts && npm test)
(cd rust && cargo fmt --check)
(cd rust && cargo clippy --all-targets --all-features -- -D warnings)
(cd rust && cargo test --all-features)

if [ -f SCC_Kernel_Fortress_checksums.sha256 ]; then
  sha256sum -c SCC_Kernel_Fortress_checksums.sha256
else
  echo "missing SCC_Kernel_Fortress_checksums.sha256" >&2
  exit 1
fi

if [ -f SCC_Kernel_Fortress_checksums.sha256.asc ]; then
  gpg --verify SCC_Kernel_Fortress_checksums.sha256.asc SCC_Kernel_Fortress_checksums.sha256
else
  echo "missing detached checksum signature" >&2
  exit 1
fi

docker build -t scc-kernel-fortress .
docker run --rm scc-kernel-fortress

echo "stage0_prelaunch=PASS"
