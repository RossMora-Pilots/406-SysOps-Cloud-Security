#!/usr/bin/env python3
"""Sanitize session transcripts by redacting PII and secrets.

Redacts email addresses, student IDs (Axxxxxxx format), and common
credential patterns from an input transcript, writing a sanitized copy.
"""
from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

EMAIL_RE = re.compile(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b")
STUDENT_ID_RE = re.compile(r"\bA\d{8}\b")
TOKEN_RE = re.compile(r"\b(?:ghp|gho|ghu|ghs|ghr)_[A-Za-z0-9]{30,}\b")
API_KEY_RE = re.compile(r"\b[A-Za-z0-9]{32,}\b")


def sanitize(text: str) -> str:
    text = EMAIL_RE.sub("[EMAIL-REDACTED]", text)
    text = STUDENT_ID_RE.sub("[STUDENT-ID-REDACTED]", text)
    text = TOKEN_RE.sub("[TOKEN-REDACTED]", text)
    # Conservative API key matcher — only apply within common patterns
    text = re.sub(
        r"(api[_-]?key|token|secret|password)\s*[:=]\s*[\"']?[A-Za-z0-9_\-]{16,}[\"']?",
        r"\1=[REDACTED]",
        text,
        flags=re.IGNORECASE,
    )
    return text


def main() -> int:
    ap = argparse.ArgumentParser(description="Sanitize a session transcript")
    ap.add_argument("input", help="Input file")
    ap.add_argument("--out", required=True, help="Output file")
    args = ap.parse_args()

    in_path = Path(args.input)
    out_path = Path(args.out)
    if not in_path.exists():
        print(f"ERROR: {in_path} not found", file=sys.stderr)
        return 1

    out_path.parent.mkdir(parents=True, exist_ok=True)
    text = in_path.read_text(encoding="utf-8", errors="replace")
    out_path.write_text(sanitize(text), encoding="utf-8")
    print(f"SANITIZE:OK:{in_path} -> {out_path}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
