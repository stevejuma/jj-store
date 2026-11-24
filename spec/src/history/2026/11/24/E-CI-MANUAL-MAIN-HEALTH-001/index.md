---
schema: 1
id: E-CI-MANUAL-MAIN-HEALTH-001
---

# E-CI-MANUAL-MAIN-HEALTH-001

<!-- evidence-field:id markdown -->
## Id

E-CI-MANUAL-MAIN-HEALTH-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-24
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- scripts/ci-health.mjs
- scripts/ci-evidence.test.mjs
- .github/workflows/ci-health.yml
- .github/workflows/ci-heavy.yml
- docs/releases.md
- spec/src/current/contracts/architecture/31-ci-execution.md
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/governance/06-verification-scope.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/ci-evidence.test.mjs#freshness recovers after complete manual
  qualification of the main SHA
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
node --test --test-name-pattern='freshness recovers' scripts/ci-evidence.test.mjs
```

**Notes**

Before implementation, the successful main dispatch could not clear the earlier scheduled failure.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
node --test --test-name-pattern='freshness recovers' scripts/ci-evidence.test.mjs
```
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
node --test scripts/ci*.test.mjs
```

**Notes**

73 CI tests passed.
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

Node.js 26.8.2 and pnpm 10.32.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

GitHub Actions workflow run and attempt-specific jobs API.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Main manual qualification recovery for nightly health and completion-triggered reevaluation.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Observe the completion-triggered health job after a manual qualification of main.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-005 freshness accepts successful exact-source manual
  qualification on canonical main.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains in progress. The change concerns freshness reporting only.

```
<!-- /evidence-field -->

