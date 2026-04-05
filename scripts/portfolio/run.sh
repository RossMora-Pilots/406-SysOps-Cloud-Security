#!/usr/bin/env bash
# Portfolio automation orchestrator for Pilot 406.
#
# Phases:
#   immediate  - refresh README sections (badges, quick start, achievements, skills)
#   short      - create midterm/final project templates (if missing)
#   followup   - rebuild evidence index + scripts README
#   polish     - create weekly summary skeletons + references; install portfolio CI
#   enhance    - add architecture diagrams + learning reflection hooks
#   all        - run every phase in sequence
#
# Environment:
#   PM_COMMIT=1     auto-commit after each phase (default 1)
#   PM_PUSH=0       push after commit (default 0)

set -euo pipefail
umask 077

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PYTHONPATH="${PYTHONPATH:-$ROOT}"
export PYTHONPATH

commit_phase() {
  local phase="$1"
  if [ "${PM_COMMIT:-1}" = "0" ]; then return 0; fi
  if [ -z "$(git -C "$ROOT" status --porcelain 2>/dev/null)" ]; then return 0; fi
  git -C "$ROOT" add -A
  git -C "$ROOT" commit -m "chore(portfolio): $phase phase automation" || true
  if [ "${PM_PUSH:-0}" = "1" ]; then git -C "$ROOT" push || true; fi
}

phase_immediate() {
  echo "[phase: immediate] badges, quick start, achievements, skills"
  # No-op in this pilot; README is hand-authored at high quality.
  echo "  (README already hand-authored — skipping mutation)"
}

phase_short() {
  echo "[phase: short] midterm + final templates"
  echo "  (MIDTERM_PROJECT_SUMMARY.md authored — skipping template)"
}

phase_followup() {
  echo "[phase: followup] evidence index + scripts readme"
  echo "  (EVIDENCE_INDEX.md and SCRIPTS_README.md authored — skipping)"
}

phase_polish() {
  echo "[phase: polish] weekly summaries + references + CI"
  echo "  (weekly/ and docs/references.md authored — skipping)"
}

phase_enhance() {
  echo "[phase: enhance] architecture + learning reflection"
  echo "  (COURSE_REFLECTION.md authored — skipping)"
}

cmd="${1:-all}"
case "$cmd" in
  immediate) phase_immediate; commit_phase immediate ;;
  short)     phase_short;     commit_phase short ;;
  followup)  phase_followup;  commit_phase followup ;;
  polish)    phase_polish;    commit_phase polish ;;
  enhance)   phase_enhance;   commit_phase enhance ;;
  all)
    phase_immediate; commit_phase immediate
    phase_short;     commit_phase short
    phase_followup;  commit_phase followup
    phase_polish;    commit_phase polish
    phase_enhance;   commit_phase enhance
    ;;
  *) echo "Usage: $0 {immediate|short|followup|polish|enhance|all}" >&2; exit 2 ;;
esac
