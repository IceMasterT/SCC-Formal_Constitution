#!/usr/bin/env python3
from __future__ import annotations
from pathlib import Path
import re, sys

ROOT = Path(__file__).resolve().parents[1]
SOURCES = [ROOT / 'rust' / 'src', ROOT / 'ts' / 'src']
FORBIDDEN = [
    r'\bunsafe\b',
    r'std::time',
    r'SystemTime',
    r'Instant::now',
    r'\brand\b',
    r'Math\.random',
    r'Date\.now',
    r'process\.env',
    r'fetch\(',
    r'XMLHttpRequest',
]
violations = []
for src in SOURCES:
    for path in src.rglob('*'):
        if path.suffix not in {'.rs', '.ts'}:
            continue
        text = path.read_text(encoding='utf-8')
        for pattern in FORBIDDEN:
            for m in re.finditer(pattern, text):
                violations.append(f'{path.relative_to(ROOT)}:{text[:m.start()].count(chr(10))+1}: {pattern}')
if violations:
    print('Forbidden imports/capabilities found:')
    print('\n'.join(violations))
    sys.exit(1)
print('forbidden_imports: OK')
