---
schema: 1
id: E-DEPENDENCY-AUTOMATION-001
---

# E-DEPENDENCY-AUTOMATION-001

<!-- evidence-field:id markdown -->
## Id

E-DEPENDENCY-AUTOMATION-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-GOV-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change pnoummmrmtpksvwxlkolpztqqmznrrsx
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .github/dependabot.yml
- .github/workflows/ci-cache-build.yml
- .github/workflows/ci-cache-trial.yml
- .github/workflows/ci-heavy-backend.yml
- .github/workflows/ci-heavy-cold.yml
- .github/workflows/ci-heavy-contracts.yml
- .github/workflows/ci-heavy-native.yml
- .github/workflows/ci-heavy-storage.yml
- .github/workflows/ci-heavy-ui.yml
- .github/workflows/ci-heavy.yml
- .github/workflows/ci-qualification-build.yml
- .github/workflows/ci-ui.yml
- .github/workflows/ci.yml
- .github/workflows/npm-publish-jj-conversations.yml
- .github/workflows/npm-publish-jj-history.yml
- .github/workflows/publish-spec-pages.yml
- package.json
- scripts/automation-governance.test.mjs
- scripts/dependency-automation/spec-policy.mjs
- scripts/validation/docker.mjs
- scripts/validation/lanes.json
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M30/index.md
- spec/src/history/2026/11/01/E-DEPENDENCY-AUTOMATION-001/SUMMARY.nav.md
- spec/src/history/2026/11/01/E-DEPENDENCY-AUTOMATION-001/index.md
- spec/src/history/2026/11/01/SUMMARY.nav.md
- spec/src/history/2026/11/01/index.md
- spec/src/history/2026/11/SUMMARY.nav.md
- spec/src/history/2026/11/index.md
- spec/src/history/2026/SUMMARY.nav.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/automation-governance.test.mjs#bot PRs skip only change-specific
  governance
- scripts/automation-governance.test.mjs#bot PRs skip specification calls and
  retain planned functional and release lanes
<!-- /evidence-field -->

<!-- evidence-field:removed_tests list -->
## Removed tests

- scripts/automation-governance.test.mjs#bot PRs retain static spec and planned
  test lanes
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec spec-validator first --json
```

**Notes**

Initial same-change gate correctly rejected missing fresh evidence. The retained contract and this receipt address that source-change boundary.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
node --test scripts/automation-governance.test.mjs
```

**Notes**

Five focused tests pass. The PR-author condition excludes the trusted dependency App and Renovate while retaining the new App's release-intent gate and the planned functional/release lanes.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

The local Docker profile requested 8 CPUs but the available daemon has 5; the direct checker below validates this slice without changing the profile.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/check-progress.mjs
```

**Notes**

186 tasks and 1001 evidence records validated; M30 remains In progress.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec spec-validator first --json
```

**Notes**

All protected source changes have the required canonical chapter and fresh evidence after focused navigation cleanup.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
actionlint .github/workflows/*.yml
```

**Notes**

All repository workflow files validated after the pinned coordinator notification integration.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node 26.9.0 for the focused source tests; the coordinator separately pins and exercises Renovate 44.199.0 on Node 24.15.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No repository object, WAL, manifest CAS, RPC or native distribution format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

CI specification exemption, public PR-context propagation into Docker validation workers, and post-verification npm consumer notifications. These focused checks do not qualify native storage, UI behavior, package publication or App installation.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review and land the coordinator integration, provision the dedicated App, and review a dry-run before enabling dependency writes. M30 full release qualification remains pending.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: The public-distribution contract extends the trusted dependency PR
  specification exemption. Runtime, security, functional and publication
  authority are unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress; its existing native/container/platform
  qualification and publication gates have not been satisfied or waived.

```
<!-- /evidence-field -->

