---
schema: 1
id: E-CI-QUALIFICATION-001
---

# E-CI-QUALIFICATION-001

<!-- evidence-field:id markdown -->
## Id

E-CI-QUALIFICATION-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-GOV-003
- JST-GOV-007
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/current/contracts/architecture/31-ci-execution.md
- spec/src/current/progress/M30/index.md
- docs/reference/ci-validation-plan-status-2026-11-02.md
- spec/src/history/2026/11/03/E-CI-QUALIFICATION-001/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
gh run view 41681207161 --json databaseId,headSha,status,conclusion,updatedAt,url,workflowName
gh api repos/stevejuma/jj-store/actions/runs/41681207161
gh api 'repos/stevejuma/jj-store/actions/runs/41681207161/jobs?per_page=100'
gh api repos/stevejuma/jj-store/actions/runs/41681207161/artifacts?per_page=100
```

**Notes**

Manual complete qualification passed on canonical `main` at exact merged SHA
`5c2c4c9c8c3dc77ee7ff774a0cf724f23b9d94e3`. Run 41681207161 completed on
attempt 1 in 37 minutes 51 seconds. Its 54 jobs contain 53 successes, including
the complete qualification gate, and one expected skipped advisory compilation
cache comparison. The run retained 64 source-bound result/build artifacts.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
npx --yes pnpm@10.32.1 spec:search -- "JST-ARCH-005"
```

**Notes**

The repository wrapper already inserts a separator, so the extra documented
`--` reached `spec-validator` as an unsupported argument. No repository or
validation state changed.
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
npx --yes pnpm@10.32.1 spec:search "JST-ARCH-005"
```

**Notes**

The corrected wrapper invocation located the owning JST-ARCH-005 CI execution,
verification, governance and M30 sources before the documentation edit.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
node node_modules/@lapismd/spec-validator/dist/cli.js first --base @- --head @ --json
```

**Notes**

Spec-first rejected two initial human-readable test labels because evidence test
entries must resolve to tracked repository paths. The labels were replaced by
the owning `.github/workflows/ci-heavy.yml` path and descriptive anchors.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
node node_modules/@lapismd/spec-validator/dist/cli.js first --base @- --head @ --json
```

**Notes**

The tracked workflow resolved, but workflow job labels are not discoverable
named tests for evidence anchors. The evidence target now names the owning
workflow without a `#name` suffix.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
node node_modules/@lapismd/spec-validator/dist/cli.js first --base @- --head @ --json
```

**Notes**

Workflow YAML is not an accepted local test-source type. Following existing
hosted qualification evidence, the documentation-only record uses an empty
local test list and retains the external run and gate in Validation.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
node node_modules/@lapismd/spec-validator/dist/cli.js first --base @- --head @ --json
```

**Notes**

The evidence reference shape passed, leaving only the expected requirement that
an unchanged-contract confirmation record a completed passing validation result.
The record remained pending until the document checks below completed.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
npx --yes pnpm@10.32.1 docs:check
```

**Notes**

Documentation closure passed for 144 files and 26,736,843 bytes with links and
anchors valid. Report:
`.local/validation/runs/jj-store-validation-3d13aefd-1680-43d1-8b91-165bf2899fe5`.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
node node_modules/@lapismd/spec-validator/dist/cli.js validate --only governance,summary,fileLimits,verification --json
```

**Notes**

Four selected document validators passed across 4,606 chapters and 211
requirements with no findings.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
node node_modules/@lapismd/spec-validator/dist/cli.js first --base @- --head @ --json
npx --yes pnpm@10.32.1 check:progress
```

**Notes**

Spec-first passed for one protected file and one canonical chapter. Progress
passed for 186 tasks and 1,028 evidence records. Report:
`.local/validation/runs/jj-store-validation-a004c086-62b1-4b0d-b729-5cff8bb3251a`.
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

GitHub Actions hosted runners from run 41681207161; pnpm 10.32.1; repository
source `5c2c4c9c8c3dc77ee7ff774a0cf724f23b9d94e3`.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No production source, API, persisted data, generated contract, artifact or
cache format changed. This slice records completed evidence only.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Exact-source manual complete qualification for the integrated CI rollout. It
qualifies the selected standard and heavy lanes represented by run 41681207161,
including native, storage, API, UI, cold application and aggregate evidence.
It does not substitute for a canonical schedule-event attempt or independent
quarantine, latency, reliability, publication, provider, platform or visual
gates.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Run canonical scheduled qualification from `main`, confirm freshness health
selects that attempt, and retain the four quarantine and independent release
gates until their own evidence closes them.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-ARCH-005 already defines manual complete qualification and keeps
  scheduled freshness, quarantine diagnostics and publication independent. The
  documentation records one passing exact-source run without changing that
  contract.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. Exact-source manual complete qualification now
  passes, while canonical scheduled freshness, four quarantines, representative
  latency and reliability review, and independent publication/provider/platform/
  visual gates remain open.
```
<!-- /evidence-field -->
