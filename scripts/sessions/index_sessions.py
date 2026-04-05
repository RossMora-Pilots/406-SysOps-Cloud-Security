#!/usr/bin/env python3
"""Generate docs/sessions.md from files in the sessions/ directory.

Scans sessions/ for .md files and writes an index with size and sha256.
"""
from __future__ import annotations

import argparse
import hashlib
import sys
from pathlib import Path


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(8192), b""):
            h.update(chunk)
    return h.hexdigest()


def main() -> int:
    ap = argparse.ArgumentParser(description="Generate sessions index")
    ap.add_argument("--sessions", required=True, help="Sessions directory")
    ap.add_argument("--out", required=True, help="Output markdown file")
    args = ap.parse_args()

    sessions_dir = Path(args.sessions)
    out_path = Path(args.out)
    out_path.parent.mkdir(parents=True, exist_ok=True)

    lines = ["# Sessions Index", ""]
    if not sessions_dir.exists() or not sessions_dir.is_dir():
        lines.append("_No sessions found._")
    else:
        md_files = sorted(sessions_dir.glob("*.md"))
        if not md_files:
            lines.append("_No sessions found._")
        else:
            for f in md_files:
                size = f.stat().st_size
                digest = sha256_file(f)
                rel = f.relative_to(sessions_dir.parent)
                lines.append(f"- `{rel}` ({size} bytes) — sha256: `{digest}`")
    lines.append("")

    out_path.write_text("\n".join(lines), encoding="utf-8")
    print(f"INDEX:OK:{out_path}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
