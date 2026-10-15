---
schema: 1
id: E-CI-AFFECTED-001
---

# E-CI-AFFECTED-001 details 2

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
node --test --test-name-pattern="docs-only impact" scripts/ci.test.mjs
```

**Notes**

Before regression expected the old planner shape. Preserved the historical test anchor and asserted the richer selection explicitly.
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
node --test scripts/ci*.test.mjs scripts/release.test.mjs scripts/automation-governance.test.mjs
```

**Notes**

35 tests passed on Node 22.23.3, including leaf/core, all dependency kinds, shared/unknown/fixture inputs, rename/deletion, invalid graph/cohort, empty selection, cache policy and inventory cases.
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
actionlint
```

**Notes**

All workflows passed.
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
pnpm release:check
```

**Notes**

35 regressions and all 20 Cargo package containment/version checks passed.
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
CARGO_TARGET_DIR=.local/build-targets/archive-check pnpm dev:cargo -- nextest archive -p jj-store-read-index --locked --archive-file .release/smoke.tar.zst
```

**Notes**

Ad-hoc smoke initially omitted .release output directory. Created it and reran successfully; production ci-tests creates it before archiving.
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
CARGO_TARGET_DIR=.local/build-targets/archive-check pnpm dev:cargo -- nextest list --archive-file .release/smoke.tar.zst --extract-to .release/smoke --workspace-remap "$PWD" --message-format json
```

**Notes**

Ad-hoc listing exposed managed Cargo target-dir incompatibility for archive listing. Final inventory reads only metadata from the archive and lists existing binaries, with no extra target extraction or compilation.
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
cargo nextest list --archive-file .release/smoke.tar.zst --extract-to .release/smoke --workspace-remap "$PWD" --message-format json
```

**Notes**

Ad-hoc metadata research required an existing extraction directory. Created it, inspected the real pinned Nextest JSON, then removed this extraction path from the final approach.
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
CARGO_TARGET_DIR=.local/build-targets/archive-check CI_PACKAGES='["jj-store-read-index"]' node scripts/ci-tests.mjs build ordinary
```

**Notes**

Actual Nextest 0.9.148 archive: 4 binaries, 10 test identities; partition counts 0/4/6/0, exact disjoint union. Selected package applies to archive compilation.
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
JJ_STORE_BUILD_ROOT=<disposable-root> CARGO_TARGET_DIR=target CI_PACKAGES='["jj-store-read-index"]' CI_SHARD=<1..4> node scripts/ci-tests.mjs run ordinary
```

**Notes**

Four sequential archived shard runs in a disposable root through managed Cargo. 10/10 tests passed; two verified empty shards passed. Temporary fixture cleaned; existing root target and development data untouched.
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
pnpm check:architecture
```

**Notes**

1911 modules passed.
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
pnpm test:governance
```

**Notes**

71 tests passed.
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
pnpm test:spec
```

**Notes**

28 tests passed.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:ui-ownership
```

**Notes**

Passed.
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
pnpm spec:validate --only governance,summary,fileLimits,verification
```

**Notes**

4 validators, 3866 chapters, 198 requirements passed.
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
pnpm check:progress
```

**Notes**

155 tasks and 811 evidence records passed.
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
pnpm spec:first
```

**Notes**

16 protected files and 2 canonical topics passed.
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
gh run view 38708053841 --repo stevejuma/jj-store
```

**Notes**

Slice 1 hosted required check passed at ccec8f02; every standard lane passed and no heavy job was scheduled.
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
pnpm dev:target:retire -- --target .local/build-targets/archive-check --apply
```

**Notes**

Qualified isolated target retired and cleaned; development fixture and live targets preserved.
<!-- /run -->
<!-- /evidence-field -->

