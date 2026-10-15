---
schema: 1
id: E-CI-SUITES-001
---

# E-CI-SUITES-001 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
node --test scripts/ci-artifacts.test.mjs scripts/ci-workflows.test.mjs scripts/ci-inventory.test.mjs scripts/ci.test.mjs scripts/release.test.mjs scripts/automation-governance.test.mjs
```

**Notes**

Initial run found a new fixture syntax error and a stale automation-governance assertion naming the old heavy jobs. Both repaired; subsequent focused run passed 33 tests.
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
pnpm test:release
```

**Notes**

44 tests passed before the additional report-configuration regression; final rerun follows.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
actionlint -oneline
```
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
pnpm web:check
```

**Notes**

Svelte check: 0 errors and 0 warnings.
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
pnpm catalog:check
```

**Notes**

0 errors and warnings; catalog boundaries checked across 442 modules.
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
gh run view 38716420649 --repo stevejuma/jj-store --log-failed
```

**Notes**

Pre-existing hosted backend failure: kingfisher_redacts_secrets_and_normalizes_workspace_paths expected mongodb-kingfisher@2.7.0/builtin-medium/v1 but received kingfisher-scanner@1.4.0/builtin-medium/v1. Seven sibling shards canceled and dependent lanes skipped. Backend build completed in 17m15s; this is not a full baseline.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:release
```

**Notes**

Final regression run: 45 tests passed, including report configuration and attempt-specific release handoff.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress; pnpm spec:validate --only governance,summary,fileLimits,verification; pnpm spec:first
```

**Notes**

New evidence initially used filename-only test references, a prose result outside the allowed enum, and a local navigation update created an unwanted nested SUMMARY.md. Corrected the evidence metadata, removed only the newly created nested summary, and added the owning navigation include; no prior evidence is rewritten.
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
pnpm test:governance
```

**Notes**

71 governance/tooling fixtures passed.
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
node scripts/package-check.mjs
```

**Notes**

All 20 Cargo packages have registry dependencies and contained compilation inputs.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

155 tasks, 814 evidence records validated.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate --only governance,summary,fileLimits,verification
```

**Notes**

4 validators, 3876 chapters and 198 requirements passed.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Owner coverage additionally required API, UI and governance chapters for reporting/configuration edits. Added explicit unchanged-product-contract confirmations under JST-API-006, JST-UI-001 and JST-GOV-003/004/011.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check .
```

**Notes**

Repository-wide formatting passed; final new evidence/topic formatting checked again after metadata updates.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm progress:history
```

**Notes**

Regenerated M30 backlinks. Restored unrelated generated navigation edits, retaining this change's owning chapters and M30/new-evidence navigation only.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

New M30 history files needed inclusion in the fresh evidence file list; updated that list without changing earlier evidence.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

28 protected files and 4 mapped owning chapters passed.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate --only governance,summary,fileLimits,verification
```

**Notes**

4 validators, 3881 chapters and 198 requirements passed after final M30 navigation.
<!-- /run -->
<!-- /evidence-field -->

