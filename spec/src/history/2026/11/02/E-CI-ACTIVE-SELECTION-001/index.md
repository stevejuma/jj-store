---
schema: 1
id: E-CI-ACTIVE-SELECTION-001
---

# E-CI-ACTIVE-SELECTION-001

<!-- evidence-field:id markdown -->
## Id

E-CI-ACTIVE-SELECTION-001
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

2026-11-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .github/workflows/ci.yml
- .github/workflows/ci-ui.yml
- .github/workflows/ci-heavy-ui.yml
- scripts/ci-plan.mjs
- scripts/ci-required.mjs
- scripts/ci-selection.test.mjs
- scripts/ci-ui.test.mjs
- scripts/ci-workflows.test.mjs
- docs/reference/ci-validation-plan-status-2026-11-02.md
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/architecture/31-ci-execution.md
- spec/src/current/governance/06-verification-scope.md
- spec/src/current/progress/M30/index.md
- spec/src/current/progress/M30/history/M30/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/ci-selection.test.mjs#active domains separate Help catalog contracts
  and targeted backend
- scripts/ci-ui.test.mjs#core UI runs alongside preflight, gates PRs and can run
  alone without full qualification
- scripts/ci-ui.test.mjs#UI partitions preserve every original lane once and
  source jobs use Node setup
- scripts/ci-workflows.test.mjs#standard checks never schedule heavy
  qualification and preserve the protected gate
- scripts/ci-workflows.test.mjs#manual and release qualification pin one source
  and require every lane
- scripts/ci-workflows.test.mjs#aggregates reject missing, failed, canceled and
  unexpectedly skipped prerequisites
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
PATH="$PWD/.local/cargo/bin:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" node --test scripts/ci-selection.test.mjs scripts/ci-ui.test.mjs scripts/ci-workflows.test.mjs
```

**Notes**

All 26 focused selector, domain-workflow, full-manual and required-gate
regressions passed locally. No browser, product or Docker lane was invoked.
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
/tmp/actionlint-check/actionlint -shellcheck= -pyflakes=
```

**Notes**

Pinned actionlint 1.7.12 accepted the active domain expressions, reusable input
types and split UI suites. Shellcheck and Pyflakes were unavailable and not run.
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
npx --yes pnpm@10.32.1 exec prettier --check .
```

**Notes**

The complete repository formatting lane passed.
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
PATH="$PWD/.local/cargo/bin:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" node --test scripts/ci*.test.mjs scripts/validation.test.mjs scripts/release.test.mjs scripts/automation-governance.test.mjs
```

**Notes**

All 88 CI, validation, release-policy and automation regressions passed locally.
The wider set was required because shared workflow admission and aggregation can
alter independent lanes. Product suites and Docker remained out of scope.
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
node node_modules/@lapismd/spec-validator/dist/cli.js first --base @- --head @
```

**Notes**

Spec-first rejected a renamed PR 35 test identity referenced by immutable
E-CI-SELECTION-001. The original test name was restored and a distinct active
selection regression was added.
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
node node_modules/@lapismd/spec-validator/dist/cli.js validate --only governance,summary,fileLimits,verification
```

**Notes**

The first complete validation found the public-distribution chapter at 408
lines. The new policy wording was tightened without dropping gate semantics.
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
node node_modules/@lapismd/spec-validator/dist/cli.js validate --only governance,summary,fileLimits,verification
```

**Notes**

All four validators passed across 4584 chapters and 211 requirements; the
owning public-distribution chapter is exactly 400 physical lines.
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
node node_modules/@lapismd/spec-validator/dist/cli.js first --base @- --head @
```

**Notes**

Spec-first passed: nine protected files map to three canonical chapters.
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
node scripts/check-progress.mjs
```

**Notes**

Progress bookkeeping passed for 186 tasks and 1009 evidence records. M30 stays
In progress.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Hosted active-selection execution and representative narrow PR timings.
- Explicit manual full standard and heavy qualification.
- Existing UI lifecycle failures and release qualification.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node.js 26.8.2; pnpm 10.32.1; Rust/Cargo 1.99.0; JJ 0.46.0; selection schema 2.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Selection schema advances from version 1 shadow decisions to version 2 active
domains. Archive inventory, receipt, JUnit and evidence schema 1 formats are
unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Pull-request lane admission and aggregation only. Explicit full standard,
scheduled/manual heavy and release qualification retain their complete lane
sets. No product behavior or test identity changed.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Observe representative narrow hosted PRs, then run full standard and heavy
qualification manually against the integrated source.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-005 and JST-GOV-003/007 now authorize active domain-scoped PR
  checks before hosted heavy qualification, while retaining fail-closed broad
  fallback and complete explicitly dispatched standard/heavy gates.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. Active selection reduces ordinary PR scope but
  does not complete hosted observations, heavy qualification, lifecycle repairs
  or release gates.
```
<!-- /evidence-field -->

<!-- generated-navigation -->

## Pages
