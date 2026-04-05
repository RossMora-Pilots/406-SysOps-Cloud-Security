# Privacy & Sanitization Policy

This repository is **public**. The following rules govern what may and may not appear here.

## Retained (Public Attribution)

- ✅ **Instructor name** (Aditya Palshikar) — public record per course listing.
- ✅ **Repository owner's name** (Ross Moravec) — portfolio attribution.
- ✅ **Course title, code, term, institution** — public record.
- ✅ **Vendor names** (Palo Alto Networks, Wazuh, Microsoft, etc.) — factual references.

## Removed / Excluded

- ❌ **Other students' names and student IDs** — never appear in this repository, including in:
  - Group project team rosters (e.g., `Build a SOC Groups.xlsx`)
  - Co-authored submission filenames
  - Any lecture/discussion transcripts that name individuals
- ❌ **Owner's student ID** (A00-style) beyond repository-level attribution metadata.
- ❌ **Email addresses** of instructor, students, staff, or third parties.
- ❌ **Phone numbers**, office locations, or personal contact details.
- ❌ **Internal Cambrian College URLs** or login-protected LMS links.

## Copyright (Referenced Only, Not Redistributed)

- ❌ **Palo Alto Networks lab PDFs** (SOFv2, CSFv2 series) — vendor-copyrighted training material.
- ❌ **Wazuh platform documentation PDFs** — vendor-copyrighted.
- ❌ **Course materials authored by the instructor** (lecture slides if any, assignment specifications) — instructor copyright.
- ❌ **Third-party imagery** (vendor screenshots, diagrams) embedded in DOCX submissions.

References to these materials use **titles and identifiers** only. Links point to vendor public sites.

## Excluded for Size

- ❌ **Video lecture recordings** (MP4, ~1.5 GB total) — stored locally, not committed to repository.

## Secret-Scanning Enforcement

- ✅ **Gitleaks workflow** runs on every push and PR.
- ✅ **`.gitleaks.toml`** allowlist permits documentation mentions of standard env-var names only (e.g., `GH_TOKEN`, `AWS_SECRET_ACCESS_KEY` in prose).
- ❌ **No real tokens, API keys, passwords, certificates, or private keys** may be committed.

## Incident Response

If PII or copyrighted material is discovered in this repository:

1. Open an issue (or email the maintainer) describing the file and line.
2. The maintainer will remove the content, force-rewrite history if necessary, and push the rewritten repository.
3. A note will be added to the ROADMAP's "Later" section to document the incident.

## Maintenance Cadence

This policy is reviewed whenever:

- A new week of content is added
- A new contributor is granted commit access
- A vendor's redistribution terms change

---

*Policy last reviewed: 2026-04-04.*
