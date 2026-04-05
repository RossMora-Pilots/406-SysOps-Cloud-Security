#!/usr/bin/env python3
"""Parse ROADMAP.md into a structured JSON artifact.

Reads ROADMAP.md, extracts section hierarchy and checkbox items,
and writes a machine-readable JSON snapshot to the output path.
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

HEADING_RE = re.compile(r"^(#+)\s+(.+?)\s*$")
CHECKBOX_RE = re.compile(r"^(\s*)-\s+\[(x| )\]\s+(.+?)\s*$")


def parse_roadmap(path: Path) -> dict:
    sections: list[dict] = []
    current_path: list[str] = []
    items: list[dict] = []

    with path.open("r", encoding="utf-8") as f:
        for lineno, line in enumerate(f, start=1):
            m_head = HEADING_RE.match(line)
            if m_head:
                level = len(m_head.group(1))
                title = m_head.group(2)
                current_path = current_path[: level - 1] + [title]
                sections.append({"level": level, "title": title, "line": lineno})
                continue
            m_check = CHECKBOX_RE.match(line)
            if m_check:
                checked = m_check.group(2) == "x"
                text = m_check.group(3)
                items.append({
                    "section": " / ".join(current_path),
                    "text": text,
                    "checked": checked,
                    "line": lineno,
                })

    total = len(items)
    checked = sum(1 for i in items if i["checked"])
    return {
        "source": str(path),
        "total": total,
        "checked": checked,
        "sections": sections,
        "items": items,
    }


def main() -> int:
    ap = argparse.ArgumentParser(description="Parse ROADMAP.md to JSON")
    ap.add_argument("roadmap", help="Path to ROADMAP.md")
    ap.add_argument("--out", required=True, help="Output JSON path")
    args = ap.parse_args()

    roadmap = Path(args.roadmap)
    if not roadmap.exists():
        print(f"ERROR: {roadmap} not found", file=sys.stderr)
        return 1

    data = parse_roadmap(roadmap)
    out = Path(args.out)
    out.parent.mkdir(parents=True, exist_ok=True)
    with out.open("w", encoding="utf-8") as f:
        json.dump(data, f, indent=2)
    print(f"Wrote {out} ({data['checked']}/{data['total']} items checked)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
