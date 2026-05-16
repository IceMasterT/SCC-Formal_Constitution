# Licensing and Copying

This repository uses a multi-license model by content type:

- **Code** (`rust/`, `ts/`, `formal/`, `tools/`, `scripts/`, `Dockerfile`, `Makefile`, `.github/`)
  is licensed under **Apache-2.0**. See `LICENSE`.
- **Canon documents** (`docs/SCC_Formal_Canon_*`, `docs/SCC_Engineering_Handbook_*`,
  `docs/SCC_System_Overview_*`) are licensed under **CC BY-SA 4.0**.
  See `LICENSE-DOCS`.
- **Paper sources** (`paper/`) are licensed under **CC BY 4.0**.
  See `LICENSE-PAPER`.
- **Fixtures and data** (`golden/`, `negative_corpus/`, `federation/`, `schemas/`)
  are licensed under **CC BY 4.0**. See `LICENSE-PAPER`.

SPDX short identifiers are used in source headers where applicable.

## REUSE-style guidance

- Source files include SPDX tags such as `Apache-2.0`.
- Root-level license texts are provided verbatim:
  - `LICENSE` (Apache-2.0)
  - `LICENSE-DOCS` (CC BY-SA 4.0)
  - `LICENSE-PAPER` (CC BY 4.0)

If a file has no explicit SPDX header, refer to its containing material class
above.
