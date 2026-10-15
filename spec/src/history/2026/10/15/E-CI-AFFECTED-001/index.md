---
schema: 1
id: E-CI-AFFECTED-001
---

# E-CI-AFFECTED-001

<!-- evidence-field:id markdown -->
## Id

E-CI-AFFECTED-001
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

Cargo affected-package planning, exact archived-test inventory and dependency cache repair. Enclosing Jujutsu change urkpvmsr. Before: standard selection compiled all 16 packages and PR caches were restore-only. Shared CI infrastructure warrants all CI/release/governance/spec-tooling regressions, actionlint and a real selected archive smoke. Product and full heavy qualification are outside this slice.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .github/workflows/ci-heavy.yml
- .github/workflows/ci.yml
- .github/workflows/publish-release.yml
- README.md
- docs/releases.md
- scripts/ci-browser.mjs
- scripts/ci-build.mjs
- scripts/ci-inventory.mjs
- scripts/ci-inventory.test.mjs
- scripts/ci-packages.mjs
- scripts/ci-plan.mjs
- scripts/ci-report.mjs
- scripts/ci-required.mjs
- scripts/ci-selection.test.mjs
- scripts/ci-source.mjs
- scripts/ci-tests.mjs
- scripts/ci-timing.mjs
- scripts/ci-workflows.test.mjs
- scripts/ci.test.mjs
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M30/index.md
- spec/src/history/2026/10/15/E-CI-AFFECTED-001/SUMMARY.nav.md
- spec/src/history/2026/10/15/E-CI-AFFECTED-001/details-02.md
- spec/src/history/2026/10/15/E-CI-AFFECTED-001/details-03.md
- spec/src/history/2026/10/15/E-CI-AFFECTED-001/index.md
- spec/src/history/2026/10/15/SUMMARY.nav.md
- spec/src/history/2026/10/15/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/ci-selection.test.mjs#affected packages include reverse transitive
  normal build dev optional and target edges
- scripts/ci-selection.test.mjs#renames and deletions include both old and new
  owners
- scripts/ci-selection.test.mjs#cache writes stay PR scoped or require canonical
  main context and source
- scripts/ci-inventory.test.mjs#verified empty shards are allowed only with
  exact source packages digest and complete disjoint inventory
<!-- /evidence-field -->

