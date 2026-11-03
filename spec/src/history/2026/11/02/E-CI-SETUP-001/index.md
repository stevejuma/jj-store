---
schema: 1
id: E-CI-SETUP-001
---

# E-CI-SETUP-001

<!-- evidence-field:id markdown -->
## Id

E-CI-SETUP-001
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

- .github/actions/setup/action.yml
- .github/workflows/ci.yml
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

- scripts/ci-workflows.test.mjs#standard checks never schedule heavy
  qualification and preserve the protected gate
- scripts/ci-ui.test.mjs#UI partitions preserve every original lane once and
  source jobs use Node setup
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
PATH="$PWD/.local/cargo/bin:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" node --test scripts/ci-ui.test.mjs scripts/ci-workflows.test.mjs
```

**Notes**

All 11 focused setup and workflow regressions passed locally.
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
/tmp/jj-store-actionlint.KVppJF/actionlint -shellcheck= -pyflakes=
```

**Notes**

Pinned actionlint 1.7.12 accepted the capability expressions. Shellcheck and
Pyflakes were unavailable and not run.
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
PATH="$PWD/.local/cargo/bin:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" node --test scripts/ci*.test.mjs scripts/validation.test.mjs scripts/release.test.mjs scripts/automation-governance.test.mjs
```

**Notes**

All 88 shared CI, validation, release-policy and automation regressions passed.
Product suites and Docker remained out of scope.
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
node node_modules/@lapismd/spec-validator/dist/cli.js validate --only governance,summary,fileLimits,verification
```

**Notes**

Four validators passed across 4588 chapters and 211 requirements.
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
node node_modules/@lapismd/spec-validator/dist/cli.js first --base @- --head @
```

**Notes**

Spec-first passed: five protected files map to three canonical chapters.
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
node scripts/check-progress.mjs
```

**Notes**

Progress bookkeeping passed for 186 tasks and 1012 evidence records. M30 stays
In progress.
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
npx --yes pnpm@10.32.1 exec prettier --check .
```

**Notes**

The complete repository formatting lane passed.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Hosted cold/warm timing for the minimal capability profiles.
- Manual full standard/heavy qualification and release gates.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node.js 26.8.2; pnpm 10.32.1; Rust/Cargo 1.99.0; JJ 0.46.0; actionlint 1.7.12.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Setup capabilities add `format` and `tooling`; selection schema 2 and all
artifact/evidence formats are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Hosted setup admission only. Selected tests, product behavior and full/archive
qualification inputs are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Compare hosted setup and job duration, then continue measured bottleneck work.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-005 and JST-GOV-003/007 now assign explicit least-capability
  setup profiles to formatting, tooling and governance without changing their
  selected checks.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. Minimal setup is locally verified but hosted
  timing, complete qualification and release gates remain outstanding.
```
<!-- /evidence-field -->

<!-- generated-navigation -->

## Pages
