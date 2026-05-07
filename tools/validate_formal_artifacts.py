#!/usr/bin/env python3
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
FORBIDDEN = ["sorry", "admit", "Admitted", "cheat", "undefined", "todo!"]
ALLOW_AXIOM_FILES = set()
checked = []
for rel in ["formal/lean", "formal/coq"]:
    base = ROOT / rel
    if not base.exists():
        print(f"missing {rel}", file=sys.stderr)
        sys.exit(1)
    for path in sorted(base.rglob("*")):
        if path.suffix not in {".lean", ".v"}:
            continue
        text = path.read_text(encoding="utf-8")
        low = text
        for token in FORBIDDEN:
            if token in low:
                print(f"formal placeholder token {token!r} found in {path.relative_to(ROOT)}", file=sys.stderr)
                sys.exit(1)
        if "axiom " in text and path.relative_to(ROOT).as_posix() not in ALLOW_AXIOM_FILES:
            print(f"unledgered axiom found in {path.relative_to(ROOT)}", file=sys.stderr)
            sys.exit(1)
        checked.append(path.relative_to(ROOT).as_posix())
print(f"validate_formal_artifacts: OK ({len(checked)} proof files, no placeholders)")
for item in checked:
    print(f"  {item}")
