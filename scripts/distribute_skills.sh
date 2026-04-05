#!/usr/bin/env bash
# Distribute unified-skills/ via symlinks to .claude/, .codex/, .gemini/.
#
# Idempotent: recreates symlinks to point at unified-skills/ sources.
# Backs up any existing real directory to <dir>.bak before replacing.

set -euo pipefail
umask 077

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SRC="$ROOT/unified-skills"

if [ ! -d "$SRC" ]; then
  echo "ERROR: unified-skills/ not found at $SRC" >&2
  exit 1
fi

for vendor in .claude .codex .gemini; do
  vendor_dir="$ROOT/$vendor/skills"
  mkdir -p "$(dirname "$vendor_dir")"
  # Back up existing real directory (not a symlink)
  if [ -d "$vendor_dir" ] && [ ! -L "$vendor_dir" ]; then
    mv "$vendor_dir" "${vendor_dir}.bak.$(date +%s)"
  fi
  # Remove existing symlink
  [ -L "$vendor_dir" ] && rm "$vendor_dir"
  # Create new symlink
  ln -s "../unified-skills" "$vendor_dir"
  echo "  linked $vendor/skills -> ../unified-skills"
done

echo "DISTRIBUTE:OK"
