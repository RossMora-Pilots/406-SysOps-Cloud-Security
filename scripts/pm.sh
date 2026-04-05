#!/usr/bin/env bash
# PM loop orchestrator for Pilot 406.
#
# Commands:
#   run     - parse roadmap, sync (safe no-op), index sessions, commit artifacts (default)
#   parse   - parse ROADMAP.md to artifacts/roadmap.json
#   sync    - sync roadmap items to GitHub Issues (requires GH_REPO and GH_TOKEN)
#   index   - generate docs/sessions.md from sessions/ directory
#   all     - run + push (if PM_PUSH=1)
#
# Environment:
#   PM_COMMIT=1        auto-commit after run (default 1)
#   PM_PUSH=0          push after commit (default 0)
#   ROADMAP_PATH       override roadmap path (default: ROADMAP.md)
#   GH_REPO            owner/repo for Issues sync
#   GH_TOKEN           GitHub token for sync

set -euo pipefail
umask 077

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ROADMAP_PATH="${ROADMAP_PATH:-$ROOT/ROADMAP.md}"

parse() {
  if [ ! -f "$ROADMAP_PATH" ]; then
    echo "PARSE:SKIP_NO_ROADMAP:$ROADMAP_PATH"
    return 0
  fi
  mkdir -p "$ROOT/artifacts"
  python3 "$ROOT/scripts/roadmap/parse_roadmap.py" "$ROADMAP_PATH" --out "$ROOT/artifacts/roadmap.json"
  echo "PARSE:OK:$ROADMAP_PATH -> artifacts/roadmap.json"
}

sync_safe() {
  if [ -z "${GH_REPO:-}" ] || [ -z "${GH_TOKEN:-}" ]; then
    echo "SYNC:SKIP_NO_CREDS"
    return 0
  fi
  python3 "$ROOT/scripts/roadmap/sync_issues.py" \
    --roadmap "$ROOT/artifacts/roadmap.json" \
    --repo "$GH_REPO"
  echo "SYNC:OK:$GH_REPO"
}

index_sessions() {
  python3 "$ROOT/scripts/sessions/index_sessions.py" \
    --sessions "$ROOT/sessions" \
    --out "$ROOT/docs/sessions.md" 2>/dev/null || echo "INDEX:SKIP"
  echo "INDEX:OK"
}

git_commit() {
  if [ "${PM_COMMIT:-1}" = "0" ]; then
    echo "COMMIT:SKIP_DISABLED"
    return 0
  fi
  if [ -z "$(git -C "$ROOT" status --porcelain artifacts/ docs/sessions.md 2>/dev/null)" ]; then
    echo "COMMIT:SKIP_NOCHANGES"
    return 0
  fi
  git -C "$ROOT" add artifacts/ docs/sessions.md 2>/dev/null || true
  git -C "$ROOT" commit -m "chore(pm): refresh roadmap artifacts and sessions index [PM loop]" || true
  echo "COMMIT:OK"
  if [ "${PM_PUSH:-0}" = "1" ]; then
    git -C "$ROOT" push || echo "PUSH:FAILED"
  fi
}

cmd="${1:-run}"
case "$cmd" in
  run)   parse; sync_safe; index_sessions; git_commit ;;
  parse) parse ;;
  sync)  sync_safe ;;
  index) index_sessions ;;
  all)   PM_PUSH=1 parse; sync_safe; index_sessions; git_commit ;;
  *) echo "Usage: $0 {run|parse|sync|index|all}" >&2; exit 2 ;;
esac
