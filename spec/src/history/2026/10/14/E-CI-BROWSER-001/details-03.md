---
schema: 1
id: E-CI-BROWSER-001
---

# E-CI-BROWSER-001 details 3

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
CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=.local/build-targets/public-release pnpm dev:cargo -- test -p jj-store-jj --lib query::cache --locked
```

**Notes**

20 affected cache tests passed, including both exact regressions, active
writer/reader lock protection, same-repository byte bounds, generation proof,
damage and cancellation paths. Ordinary admission now uses the existing bounded
managed eviction; no limits increased.
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
CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=.local/build-targets/public-release pnpm dev:cargo -- clippy -p jj-store-jj --all-targets --locked -- -D warnings
```

**Notes**

Strict affected native lint passed.
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
node --test scripts/ci.test.mjs scripts/release.test.mjs scripts/automation-governance.test.mjs; pnpm check:architecture; pnpm check:ui-ownership; actionlint; pnpm format:rust:check; node scripts/changeset-check.mjs
```

**Notes**

19 tooling tests passed; architecture passed with 1903 modules; ownership,
workflow lint, Rust format and Changesets intent passed. Static spec validation
passed with 7 validators, 3843 chapters and 198 requirements before the final
cache-topic/evidence update.
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
CI=true JJ_STORE_CI_PREBUILT=1 JJ_STORE_REAL_KEEP=1 JJ_STORE_REAL_TEST=projects-activity.spec.ts JJ_STORE_DEV_PORT=7535 node scripts/real-browser-acceptance.mjs (paired published ACP, isolated same-source tools and RustFS)
```

**Notes**

Fresh real-RustFS fixture: original review/commit/diff/tool-anchor activity
acceptance passed in 22.0 seconds. Search readiness finished within its existing
bound; full fixture preparation took 258719 ms, cold 824 ms, restart 1126 ms,
273696 KiB resident. No search limit or readiness assertion was changed.
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
pnpm exec playwright test --config web/playwright.real.config.ts work-store.spec.ts --output=/tmp/jj-store-ci-work-fresh (newly initialized isolated fixture)
```

**Notes**

Exact WorkStore regression passed in 46.5 seconds after fixing search
interaction and canonical default Back comparison. Original mutation, comment,
unknown-outcome, linking, filtering, restoration, accessibility and responsive
assertions retained.
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
CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=.local/build-targets/public-release pnpm dev:cargo -- test -p jj-store-jj --lib --locked
```

**Notes**

Complete native library suite: 93 passed, 3 existing ignored stress tests.
Shared cache infrastructure justified widening from the exact regressions and 20
cache tests; ignored stress qualification remains separate.
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
CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=.local/build-targets/public-release pnpm dev:cargo -- build -p jj-store --bins --locked
```

**Notes**

Current native server/client built in 2m27s. Existing macOS debug unwind warning
remains distinct from optimized release qualification. Stopped both owned
fixtures before replacing their two disposable executables; no installed tool or
interactive state replaced.
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm dev:restart (owned retained fixture); GET exact historical /commits/{commit}?at_op={operation} with all LocalTrusted protocol headers
```

**Notes**

Original retained-fixture request changed from HTTP 413 to HTTP 200 after the
cache eviction fix. Exact historical operation and commit were preserved. Cache
slot/byte limits, active locks and source admission unchanged.
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
CI=true JJ_STORE_CI_PREBUILT=1 node scripts/ci-browser.mjs browser (temporary qualification workspace)
```

**Notes**

Setup diagnostic: temporary workspace had no storybook-static artifact; no cold
catalog product scenario was exercised. Rebuilt the complete catalog in the
owning workspace before rerunning.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 198
hasNote: true
noteEncoding: markdown

```

```shell
CI=true JJ_STORE_CI_PREBUILT=1 node scripts/ci-browser.mjs storybook (temporary qualification workspace)
```

**Notes**

All 198 suites failed setup, with zero tests executed: a symlinked dependency
directory resolved the addon setup outside the temporary Vite serving root.
Retry uses the owning workspace and its dependencies; this is not a product
qualification pass.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook (/tmp/jj-store-ci-storybook-final2.log)
```

**Notes**

Interrupted after the first conflict story family found the retired Repository
actions selector. No aggregate pass claimed. Standalone menu fixtures keep their
label; shell scenarios now use the actual Application menu.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-mutations/StoreConflictEditor.stories.svelte
```

**Notes**

All seven exact stories reproduced the old shell menu selector before the fix.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-mutations/StoreConflictEditor.stories.svelte (current Application menu)
```

**Notes**

Seven exact conflict-editor stories passed. Atomic resolution, staged draft,
unknown-outcome recovery, focus restoration and native merge-header assertions
retained.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
CI=true JJ_STORE_CI_PREBUILT=1 node scripts/ci-browser.mjs browser (/tmp/jj-store-ci-served-final2.log)
```

**Notes**

Cold catalog reached the file-browser assertion and found the older README.md
button name. The current button is explicitly named Open file README.md.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
CI=true JJ_STORE_CI_PREBUILT=1 node scripts/ci-browser.mjs browser (/tmp/jj-store-ci-served-final3.log)
```

**Notes**

Cold catalog regression passed in 16.8 seconds after the explicit file selector
update, retaining source safety, mobile and static network assertions. The
subsequent preview stage could not start because its prebuilt executable had not
been staged in the owning workspace. Rebuild/current managed preview
qualification follows.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit (complete affected suite; /tmp/jj-store-ci-catalog-unit-final.log)
```

**Notes**

178 files passed: 1059 passed and one fixture skip. Later legacy and guarded
committed-route tests also require final requalification.
<!-- /run -->
<!-- /evidence-field -->
