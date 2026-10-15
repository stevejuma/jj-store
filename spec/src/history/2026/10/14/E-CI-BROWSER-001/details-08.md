---
schema: 1
id: E-CI-BROWSER-001
---

# E-CI-BROWSER-001 details 8

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
CI=true pnpm exec vitest run --config catalog/vitest.config.ts --project storybook StoreRepositoryConversion.stories.svelte --reporter=verbose (/tmp/jj-store-ci-conversion-stories19.log)
```

**Notes**

All four conversion interactions/accessibility cases passed after the observer
ownership repair.
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
pnpm web:check; pnpm catalog:check (/tmp/jj-store-ci-web-check-final19.log, /tmp/jj-store-ci-catalog-check-final19.log)
```

**Notes**

Final conversion source: zero type errors/warnings; catalog boundaries pass with
442 modules.
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
JJ_STORE_DOC_REFERENCE=.local/build-targets/public-release/debug/examples/docs_reference pnpm web:build (/tmp/jj-store-ci-web-final19.log)
```

**Notes**

Fresh complete production browser/Help bundle includes conversion observer
ownership and all preceding fixes. Documentation source review was refreshed
with pnpm docs:review application. No production source changed after this
build.
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
node --test scripts/ci.test.mjs scripts/automation-governance.test.mjs scripts/release.test.mjs; actionlint (/tmp/jj-store-ci-tooling-final20.log, /tmp/jj-store-ci-actionlint-final20.log)
```

**Notes**

All 21 tests and workflow lint passed. Browser runners omit native headers and
build tools except the archive-owning Git parent; Chromium is shared and WebKit
is installed only for dual-engine Help.
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts --max-failures=1 (/tmp/jj-store-ci-served-final19.log)
```

**Notes**

107 passed, one thread-performance failure and six unrun. Dual-engine Help did
not launch after this failure. This is explicitly incomplete acceptance. The
failing unrelated read path was not identified until the exact diagnostic below.
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts thread-performance.spec.ts --max-failures=1 (/tmp/jj-store-ci-thread-perf21.log)
```

**Notes**

One failure: threads directory reads increased from one to two. Network trace
shows completed mocked repository SSE responses reconnecting and refreshing
directory/detail/timeline independently of the reaction. Added the exact request
path to assertion diagnostics.
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts thread-performance.spec.ts --max-failures=1 (/tmp/jj-store-ci-thread-perf-after21.log)
```

**Notes**

One exact cold served case passed after retaining an open, abortable idle SSE
response. All original one-target query and zero unrelated
directory/detail/timeline/emoji/asset read assertions remain. Product refresh,
disconnect and reconnection behavior is unchanged; this fixture now models the
healthy idle feed required by the test.
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
CI=true JJ_STORE_CI_PREBUILT=1 node scripts/ci-browser.mjs browser (/tmp/jj-store-ci-served-final21.log)
```

**Notes**

Complete served-browser acceptance: all 114 tests passed in a fresh preview
process against final19 production assets. No tests skipped or left unrun. All
original query, reaction, navigation, CSP, accessibility, retained-draft and
conversion assertions remain selected.
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
CI=true JJ_STORE_CI_PREBUILT=1 node scripts/ci-browser.mjs help (/tmp/jj-store-ci-help-final21.log)
```

**Notes**

Complete dual-engine Help acceptance: all 20 tests passed, ten in Chromium and
ten in WebKit. These are excluded from served shards and owned by this separate
CI lane.
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
pnpm release:check; actionlint; pnpm docs:check (/tmp/jj-store-ci-release-final21.log, -actionlint-final21.log, -docs-final21.log)
```

**Notes**

Final 21 release/orchestration/governance tooling tests, all 20 portable Cargo
packages, coordinated version 0.1.0, workflow lint and complete documentation
closure pass. Nine Mermaid diagrams render in both themes; 143 documentation
files have valid local links and anchors.
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
pnpm dev:clean -- --target /private/tmp/jj-store-ci-cold-20261014/target --dry-run; pnpm dev:clean -- --target /private/tmp/jj-store-ci-cold-20261014/runtime/embedded-rustfs/target --dry-run (/tmp/jj-store-ci-temp-clean-preview21.log)
```

**Notes**

Both exact temporary targets reported eligible idle artifacts. These directories
contained copied regular executables, not Cargo caches.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:clean -- --target /private/tmp/jj-store-ci-cold-20261014/target --apply (/tmp/jj-store-ci-temp-clean21.log)
```

**Notes**

The managed cleaner refused the missing CACHEDIR.TAG, preserving its guard. No
marker was fabricated and no active/ambiguous Cargo target was removed.
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
Archive ci-cold results; verify four copied tools against retained originals with SHA-256 and lsof; unlink only exact copies; jj workspace forget ci-cold; remove forgotten owned temporary checkout (/tmp/jj-store-ci-temp-clean21.log)
```

**Notes**

Both divergent temporary changes were audited: empty, or seven generated shard-3
result/screenshot files; no unique source. Results and earlier bundles are
preserved in ignored .local/ci-artifacts/ci-cold-20261014-evidence.tar.gz
(27812092 bytes). Exact files: target/debug/jj-store, target/debug/jj-store-jj,
target/debug/examples/docs_reference and
runtime/embedded-rustfs/target/debug/jj-store-rustfs under the temporary
checkout. All were regular single-link files, byte-identical to retained
originals, with no open process. The guard-refused directories held only these
four disposable tool copies; those copies were explicitly removed without
treating them as Cargo caches. Only default workspace remains. Main fixture,
retained release target and live services are preserved.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate --only governance,summary,fileLimits,verification (/tmp/jj-store-ci-spec-final21.log)
```

**Notes**

Forcing generic Prettier prose wrapping onto the ignored generated evidence expanded seven pages beyond 400 physical lines. Repartitioned the same complete evidence with its canonical writer, retaining wrapped notes, commands and all runs. The repository formatter already excludes generated evidence; no limits or formatters were weakened.
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
pnpm check:progress (/tmp/jj-store-ci-progress-final22.log)
```

**Notes**

The evidence checker correctly rejected an overall pass while fresh hosted/release gates remain in Remaining. Restored overall pending: every completed local lane has its own passing run; neither hosted qualification nor release status is promoted.
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
pnpm spec:validate --only governance,summary,fileLimits,verification (/tmp/jj-store-ci-spec-final22.log)
```

**Notes**

Canonical pagination passes all four static validators across 3849 chapters and 198 requirements. Generic formatting expansion did not change limits or drop evidence.
<!-- /run -->
<!-- /evidence-field -->
