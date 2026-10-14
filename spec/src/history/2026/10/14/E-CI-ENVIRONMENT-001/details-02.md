---
schema: 1
id: E-CI-ENVIRONMENT-001
---

# E-CI-ENVIRONMENT-001 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
GitHub Actions CI 38628214721: fast and build (ordinary/heavy)
```

**Notes**

Three jobs failed: two governance tests inherited the PR revision or colored Cargo output; both build cohorts stopped on the same two Linux-only st_mode identity-cast diagnostics. Pages and web passed; native jobs were still running.
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
JJ_STORE_CHECK_REVISION=c396cddf9b5636a4c6e952c61c9654bc740b3b9c node --test --test-name-pattern=progress_checks_merge_against_every_parent scripts/governance.test.mjs (before repair)
```

**Notes**

Reproduced the missing PR revision in the disposable fixture before editing.
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
CARGO_TERM_COLOR=always node --test --test-name-pattern='two managed workspaces' scripts/managed-cargo.test.mjs (before repair)
```

**Notes**

Reproduced ANSI style sequences interrupting Fresh managed-proof; compilation reuse itself succeeded.
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
JJ_STORE_CHECK_REVISION=c396cddf9b5636a4c6e952c61c9654bc740b3b9c node --test --test-name-pattern=progress_checks_merge_against_every_parent scripts/governance.test.mjs
```

**Notes**

One passed after selecting the fixture working revision explicitly. Production still defaults to JJ_STORE_CHECK_REVISION.
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
CARGO_TERM_COLOR=always node --test --test-name-pattern='two managed workspaces' scripts/managed-cargo.test.mjs
```

**Notes**

One passed after stripping only terminal control sequences from the assertion input. Fresh required and Compiling forbidden assertions remain.
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
JJ_STORE_CHECK_REVISION=c396cddf9b5636a4c6e952c61c9654bc740b3b9c JJ_STORE_CHECK_BASE=a7458b22d0c9e4913a46132a7d4fc83684b44511 CARGO_TERM_COLOR=always pnpm test:governance
```

**Notes**

All 68 governance/tooling tests passed under the actual CI environment. Shared governance/test infrastructure justifies the complete owning tooling suite.
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
CARGO_TARGET_DIR=.local/build-targets/public-release CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 pnpm dev:cargo -- clippy -p jj-store-service --all-targets --locked -- -D warnings
```

**Notes**

Strict all-target affected-package lint passed on macOS ARM64. Linux hosted lint must still pass; no lint allowance added.
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
CARGO_TARGET_DIR=.local/build-targets/public-release CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-service --test working_tree_inventory --locked -- --test-threads=1
```

**Notes**

All 10 direct physical inventory regressions passed. Mode masks now operate on the native field type; fingerprint bytes are unchanged.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:spec (first local run)
```

**Notes**

The migration audit correctly rejected concurrent working-copy formatting during its revision-pinned read. No assertion changed; rerun with the working copy stable.
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
pnpm test:spec (stable rerun)
```

**Notes**

All 26 spec-tooling regressions passed.
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
pnpm test:release
```

**Notes**

All 12 CI/release regressions passed.
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
pnpm format:rust:check
```

**Notes**

Both root and embedded-runtime formatting checks passed.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress; pnpm spec:first --base codex/ci-portability-and-bootstrap --head @ (initial evidence checks)
```

**Notes**

The unchanged-contract confirmation rejected a pending result or remaining broader qualification gap. Evidence now explicitly scopes its pass to completed local checks, while M30 and Next retain required hosted/release qualification. No checker weakened. The service file also required explicit JST-MUT-001 unchanged owner coverage; added after reading the local stacks observation chapter.
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
pnpm check:progress
```

**Notes**

Passed: 155 tasks and 803 evidence records; bookkeeping only.
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
pnpm spec:first --base main@origin --head @
```

**Notes**

Passed the complete PR range: 118 protected files and 15 canonical chapters. The focused follow-up range also passed.
<!-- /run -->

<!-- run -->
### Run 16

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

All four static validators passed.
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
pnpm exec prettier --check .
```

**Notes**

Whole-checkout formatting passed.
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

Rust 1.99.0; upstream JJ 0.45.1; Node 24.10.0; pnpm 10.32.1; macOS ARM64 local verification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native modes, physical fingerprints, repository objects/WAL/receipts and public contracts unchanged.
<!-- /evidence-field -->

