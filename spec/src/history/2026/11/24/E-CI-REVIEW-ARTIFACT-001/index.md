---
schema: 1
id: E-CI-REVIEW-ARTIFACT-001
---

# E-CI-REVIEW-ARTIFACT-001

<!-- evidence-field:id markdown -->
## Id

E-CI-REVIEW-ARTIFACT-001
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
- JST-GOV-003
- JST-API-001
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- scripts/ci-artifact-io.mjs
- scripts/ci-artifacts.mjs
- scripts/ci-artifacts.test.mjs
- web/tests/real/projects-activity.spec.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/real/projects-activity.spec.ts#native project task and review pages
  share pinned commit activity
- scripts/ci-artifacts.test.mjs#artifact download retries HTTP 503 and stops on
  HTTP 403
<!-- /evidence-field -->

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
gh run view 45470881983
```

**Notes**

Backend 3/8 failed before test execution while downloading the qualification-source-1 receipt. Azure blob storage returned HTTP 503. Cold application 3/4 had ten passes, one existing skip and one review failure. Its trace showed a repository anchor with no code comparison. The downstream complete gate also failed.
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
node --test --test-name-pattern='artifact download retries' scripts/ci-artifacts.test.mjs
```

**Notes**

Before implementation.
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
node --test --test-name-pattern='artifact download retries' scripts/ci-artifacts.test.mjs
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
node --test scripts/ci*.test.mjs
```

**Notes**

74 passed.
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
JJ_STORE_REAL_SHARD=3/4 pnpm web:test:real
```

**Notes**

Fresh embedded RustFS fixture and prebuilt browser assets. Eleven passed, one existing skip.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec svelte-check --tsconfig web/tsconfig.json
```
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
downloadArtifact and verifyReceipt against qualification-source-1 from Actions run 45470881983, attempt 1
```
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

Linux x86_64, GitHub CLI, pnpm 10.32.1, baseline main fc99a143.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing artifact names, receipt and payload formats.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Bounded retry for transient hosted artifact downloads and deterministic selection of the seeded revision review for the cold changed-files assertion.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Run full qualification for the integrated revision.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-ARCH-005, JST-GOV-003 and JST-API-001 retain the same gate,
  provenance checks and native review semantics.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains in progress pending integrated qualification.

```
<!-- /evidence-field -->

