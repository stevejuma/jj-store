---
schema: 1
id: E-CI-STANDARD-001
---

# E-CI-STANDARD-001

<!-- evidence-field:id markdown -->
## Id

E-CI-STANDARD-001
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

- JST-ARCH-005
- JST-GOV-003
- JST-GOV-004
- JST-GOV-011
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Separate standard pull-request checks from complete manually initiated qualification. Enclosing Jujutsu change zmkyknlq. Before: run 38700726811 at 199969c8 passed standard checks in about 14 minutes, then heavy shard 5 failed; sibling cancellation and unrun downstream lanes remain unqualified. Shared CI and release callers warrant all tooling/governance regressions and workflow lint; no product behavior or fixture changes.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .github/workflows/ci-heavy.yml
- .github/workflows/ci.yml
- .github/workflows/publish-release.yml
- .github/workflows/release.yml
- README.md
- docs/releases.md
- package.json
- scripts/automation-governance.test.mjs
- scripts/ci-plan.mjs
- scripts/ci-required.mjs
- scripts/ci-workflows.test.mjs
- scripts/ci.test.mjs
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M30/index.md
- spec/src/history/2026/10/15/E-CI-STANDARD-001/SUMMARY.nav.md
- spec/src/history/2026/10/15/E-CI-STANDARD-001/details-02.md
- spec/src/history/2026/10/15/E-CI-STANDARD-001/details-03.md
- spec/src/history/2026/10/15/E-CI-STANDARD-001/index.md
- spec/src/history/2026/10/15/SUMMARY.nav.md
- spec/src/history/2026/10/15/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/ci-workflows.test.mjs#manual and release qualification pin one source
  and require every lane
- scripts/ci-workflows.test.mjs#aggregates reject missing, failed, canceled and
  unexpectedly skipped prerequisites
- scripts/ci.test.mjs#failed early or ordinary lanes cannot start expensive
  qualification
<!-- /evidence-field -->

