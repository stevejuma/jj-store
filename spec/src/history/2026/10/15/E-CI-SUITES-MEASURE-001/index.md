---
schema: 1
id: E-CI-SUITES-MEASURE-001
---

# E-CI-SUITES-MEASURE-001

<!-- evidence-field:id markdown -->
## Id

E-CI-SUITES-MEASURE-001
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

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change soskvslx
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Measure the parallel full coordinator and all six independent suites at the same immutable source. Preserve product failures and incomplete qualification. Correct additive TAP/native timing reporting and scheduler-wait labels from hosted evidence. Shared CI/reporting changes require the complete release/orchestration regression suite and relevant document gates; unrelated local product suites remain out of scope.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- README.md
- docs/releases.md
- scripts/ci-artifacts.test.mjs
- scripts/ci-report.mjs
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/governance/03-bounded-specification-and-evidence.md
- spec/src/current/progress/M30/history/M30/index.md
- spec/src/current/progress/M30/index.md
- spec/src/history/2026/10/15/E-CI-SUITES-MEASURE-001/SUMMARY.nav.md
- spec/src/history/2026/10/15/E-CI-SUITES-MEASURE-001/details-02.md
- spec/src/history/2026/10/15/E-CI-SUITES-MEASURE-001/details-03.md
- spec/src/history/2026/10/15/E-CI-SUITES-MEASURE-001/index.md
- spec/src/history/2026/10/15/SUMMARY.nav.md
- spec/src/history/2026/10/15/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/ci-artifacts.test.mjs#timing reports retain Node TAP counts without
  inventing missing results
- scripts/ci-artifacts.test.mjs#native timing separates target preparation from
  test execution
<!-- /evidence-field -->


<!-- generated-navigation -->

## Pages

- [E-CI-SUITES-MEASURE-001 details 2](details-02.md)
- [E-CI-SUITES-MEASURE-001 details 3](details-03.md)
