---
schema: 1
id: E-STORY-PARITY-001
---

# E-STORY-PARITY-001 details 5

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
pnpm catalog:test:browser
```

**Notes**

Docker fbfe469c-cad9-4696-ae65-11b97aa8e0cc built the complete static catalog, then failed the native file-row focus locator on the same accessible-count drift. The cold script now locates the exact data-file-path and also asserts its accessible basename.
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
pnpm catalog:test:browser
```

**Notes**

Docker 0c741abb-4730-4db1-b7ad-0285c1f1e88d: complete static build and cold desktop/mobile/Docs/application browser acceptance passed (1/1).
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
pnpm web:test:browser
```

**Notes**

Docker b483bc99-454b-4902-9d89-6f0d1d52a21d: build/docs/native fixtures passed, then Playwright server readiness expired during cold compilation. Zero browser tests executed; the owned orphan preview was stopped. Build preparation now completes through managed Cargo before the unchanged 30-second readiness window.
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
node --test scripts/browser-tests.test.mjs scripts/validation.test.mjs scripts/validation-pins.test.mjs scripts/storybook-parity.test.mjs
```

**Notes**

24/24 passed, including prepared target/filter preservation, build failure preventing startup, and safe executable quoting.
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
pnpm web:test:browser web/tests/browser/reads.spec.ts -g native_graphs_files_and_source_rendering_are_accessible
```

**Notes**

Docker 79763e43-675b-4b38-a1af-904cd4f3df8c reached browser execution with prepared preview; the exact bin selector failed because accessible file names now include counts. The locator now retains exact data-file-path identity and its accessible basename.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 73
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook
```

**Notes**

Initial unfiltered Docker attempt (storybook-full.log): 36 files failed /143 passed; 73 tests failed /905 passed. Browser transport interruption left 37 files unqualified. Subsequent bounded fresh-browser shards execute the complete inventory; interrupted execution is not a pass.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 18
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:test:browser
```

**Notes**

Docker d843803c-b64b-4d98-b7ee-75a6dc95bd6f: all 114 cases executed, 96 passed /18 failed. Eleven mobile accessibility findings share the unlandmarked toolbar target. Six file-row cases used retired full-path or count-free accessible names; selectors now retain exact data-file-path plus the visible basename. One history case captured the URL before immutable commit canonicalization; it now awaits the exact pinned route before refresh/history assertions. The held old-file request is released even after assertion failure. Original screenshots/traces retained.
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
pnpm test:governance
```

**Notes**

Docker 1b4c8b38-cd40-419e-88cd-8700b432f366: all 118 governance tests passed.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:test:browser web/tests/browser/agent-chat-changes.spec.ts web/tests/browser/appearance.spec.ts web/tests/browser/conflicts.spec.ts web/tests/browser/navigation.spec.ts web/tests/browser/reads.spec.ts
```

**Notes**

Docker 535dfa45-f670-460a-83e2-8a00300c67b1: 17/19 passed. The repaired landmark and conflict cases pass. A later saved-file selector also needed its exact data path plus accessible counts. The cold history case exposed a real refresh race: visible snapshot rows remained after fileTerms was cleared, leaving selection without a blob request. Two deterministic source regressions reproduce identity loss and missing commit selection.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_VALIDATION_RUNTIME=native pnpm catalog:test web/tests/vite/automatic-store-refresh.test.ts web/tests/vite/file-selection-identity.test.ts
```

**Notes**

Before production repair: two failed /five passed. Automatic refresh deleted the visible file identity, and selecting a visible file retained a null commit.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm catalog:test web/tests/vite/automatic-store-refresh.test.ts web/tests/vite/file-selection-identity.test.ts
```

**Notes**

After source repair: all seven passed. Refresh keeps the visible identity map until new rows publish; file selection pins a known snapshot commit while preserving an already selected commit.
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
pnpm web:test:browser web/tests/browser/navigation.spec.ts -g "hash route restores snapshot and file" --repeat-each=3
```

**Notes**

Three repeated cold navigation cases passed after the source identity/commit repair; navigation-race-cold-final.log.
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
pnpm web:test:browser web/tests/browser/reads.spec.ts -g native_graphs_files_and_source_rendering_are_accessible
```

**Notes**

Docker 84e18d48-e4b5-48e0-ac87-0fe2eeed1056: exact original source-rendering regression passed after the final production changes.
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
pnpm web:test:browser web/tests/browser/agent-chat-changes.spec.ts web/tests/browser/appearance.spec.ts web/tests/browser/conflicts.spec.ts web/tests/browser/navigation.spec.ts web/tests/browser/reads.spec.ts
```

**Notes**

Docker e0ca6189-a496-4b1e-81e0-8d05b5efb576: all 19 repaired-family cases passed in a fresh production build.
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
pnpm web:test:browser
```

**Notes**

Docker 2c617a60-2bdb-4b82-8e61-f1762fff8ef8: complete cold served-application suite, 114/114 passed in one command; zero interaction/accessibility failures.
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
pnpm catalog:test
```

**Notes**

Docker 18fe38f1-4d3f-4541-9ee6-f2176f1bb619: all 190 files, 1208 passed /one pre-existing skipped test.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:release
```

**Notes**

57/59 passed. A governance assertion still inspected the old inline package script after Docker routing, and the CI selection fixture omitted history-service/vault packages now present in Cargo metadata.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
node --test --test-name-pattern='bot PRs retain static spec and planned test lanes|real read-index and model impacts preserve the agreed standard cohort' scripts/automation-governance.test.mjs scripts/ci-selection.test.mjs
```

**Notes**

Both exact regressions failed before repair.
<!-- /run -->
<!-- /evidence-field -->

