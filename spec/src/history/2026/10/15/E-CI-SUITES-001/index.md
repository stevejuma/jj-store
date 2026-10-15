---
schema: 1
id: E-CI-SUITES-001
---

# E-CI-SUITES-001

<!-- evidence-field:remaining list -->
## Remaining

- Hosted full qualification and six standalone suite measurements remain pending.
- The pre-existing Kingfisher identity regression and newly exposed product
  failures remain release gaps.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Local Node v24.10.0, pnpm 10.32.1, actionlint 1.7.12. CI preserves Rust 1.99.0 and Nextest 0.9.148.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Qualification receipts and timing JSON use format 1. Existing archive inventory format 1 and product schemas remain unchanged. Artifact names now include the producer attempt; publication uses the same-run attempt-specific web bundle.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Push the locally verified change with immediate protection restoration; collect complete hosted and standalone-suite measurements.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-005 now permits independent heavy suites, verified cross-run
  artifact reuse, continued failure collection and a brief user-authorized
  administrator push exception. Product behavior and release success
  requirements remain intact.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. Restructuring and local checks do not qualify
  incomplete or failing heavy tests.

```
<!-- /evidence-field -->

<!-- evidence-field:id markdown -->
## Id

E-CI-SUITES-001
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
- JST-API-006
- JST-UI-001
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change suqwtxvx
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Preserve the existing qualification inventory while separating six manually dispatchable/reusable suites, removing unrelated dependencies, validating producer provenance and retaining timing/failure reports. Standard checks still precede heavy work; publication remains gated and disabled. User explicitly authorizes direct main pushes with administrator enforcement disabled only during each verified push attempt and restored immediately. Shared CI infrastructure changes require the complete affected tooling/release regression suite and hosted full qualification; unrelated local Rust/product suites are out of scope.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .config/nextest.toml
- .github/actions/qualification-report/action.yml
- .github/workflows/ci-heavy-backend.yml
- .github/workflows/ci-heavy-cold.yml
- .github/workflows/ci-heavy-contracts.yml
- .github/workflows/ci-heavy-native.yml
- .github/workflows/ci-heavy-storage.yml
- .github/workflows/ci-heavy-ui.yml
- .github/workflows/ci-heavy.yml
- .github/workflows/ci-qualification-build.yml
- .github/workflows/publish-release.yml
- README.md
- catalog/vitest.config.ts
- docs/releases.md
- scripts/automation-governance.test.mjs
- scripts/ci-artifacts.mjs
- scripts/ci-artifacts.test.mjs
- scripts/ci-browser.mjs
- scripts/ci-exec.mjs
- scripts/ci-inventory.test.mjs
- scripts/ci-report.mjs
- scripts/ci-required.mjs
- scripts/ci-tests.mjs
- scripts/ci-timing.mjs
- scripts/ci-workflows.test.mjs
- scripts/ci.test.mjs
- spec/src/current/contracts/api/14-generated-application-contracts.md
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/ui/01-storybook-presentation-catalog.md
- spec/src/current/governance/03-bounded-specification-and-evidence.md
- spec/src/current/progress/M30/SUMMARY.nav.md
- spec/src/current/progress/M30/history/M30/SUMMARY.nav.md
- spec/src/current/progress/M30/history/M30/index.md
- spec/src/current/progress/M30/history/SUMMARY.nav.md
- spec/src/current/progress/M30/history/index.md
- spec/src/current/progress/M30/index.md
- spec/src/history/2026/10/15/E-CI-SUITES-001/SUMMARY.nav.md
- spec/src/history/2026/10/15/E-CI-SUITES-001/details-02.md
- spec/src/history/2026/10/15/E-CI-SUITES-001/index.md
- spec/src/history/2026/10/15/SUMMARY.nav.md
- spec/src/history/2026/10/15/index.md
- web/playwright.config.ts
- web/playwright.real.config.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/ci-artifacts.test.mjs#artifact reuse requires successful exact
  producers, not a successful unrelated suite
- scripts/ci-artifacts.test.mjs#receipts reject wrong source, attempts, roles,
  missing payloads and unsafe paths
- scripts/ci-artifacts.test.mjs#downloaded payload checks reject corruption and
  executable symlinks
- scripts/ci-workflows.test.mjs#manual and release qualification pin one source
  and require every lane
- scripts/ci-workflows.test.mjs#aggregates reject missing, failed, canceled and
  unexpectedly skipped prerequisites
- scripts/ci.test.mjs#heavy browser reporting keeps all tests and emits JUnit
<!-- /evidence-field -->


<!-- generated-navigation -->
## Pages

- [E-CI-SUITES-001 details 2](details-02.md)
- [E-CI-SUITES-001 details 3](details-03.md)
