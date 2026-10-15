---
schema: 1
id: E-CI-GOVERNANCE-001
---

# E-CI-GOVERNANCE-001

<!-- evidence-field:id markdown -->
## Id

E-CI-GOVERNANCE-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-15
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-GOV-002
- JST-GOV-003
- JST-GOV-004
- JST-GOV-011
- JST-ARCH-005
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Repair Git-only CI progress selection while retaining direct-parent evidence freshness, immutable history and explicit full-PR spec-first range validation. Enclosing Jujutsu change qvmyqtnxnvty. Shared governance infrastructure warrants the complete spec-tool suite plus direct progress, CI and automation consumers; product builds and Rust/browser suites are outside this slice.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- scripts/spec/change-confirmations.mjs
- scripts/spec/ci-revision-regression.test.mjs
- scripts/spec/revision-reader.mjs
- spec/src/current/governance/06-verification-scope.md
- spec/src/current/progress/M30/index.md
- spec/src/history/2026/10/15/E-CI-GOVERNANCE-001/SUMMARY.nav.md
- spec/src/history/2026/10/15/E-CI-GOVERNANCE-001/details-02.md
- spec/src/history/2026/10/15/E-CI-GOVERNANCE-001/details-03.md
- spec/src/history/2026/10/15/E-CI-GOVERNANCE-001/index.md
- spec/src/history/2026/10/15/SUMMARY.nav.md
- spec/src/history/2026/10/15/index.md
- spec/src/history/2026/10/SUMMARY.nav.md
- spec/src/history/2026/10/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/spec/ci-revision-regression.test.mjs#CI range does not replace
  direct-parent progress evidence; explicit ranges stay complete
- scripts/spec/ci-revision-regression.test.mjs#Git CI progress retains every
  merge parent and rejects reused or edited evidence
<!-- /evidence-field -->
