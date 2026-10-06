---
schema: 1
id: E-UI-019
---

# E-UI-019 details 2

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
pnpm exec tsx --test web/tests/store-conflict-model.test.ts web/tests/production-store-conflicts.test.ts
```

**Notes**

The test-first controller and command-model cases failed because the conflict modules did not exist. They established the exact snapshot/upload binding and complete atomic selection boundary before implementation.
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
pnpm exec tsx --test web/tests/store-conflict-model.test.ts web/tests/production-store-conflicts.test.ts
```

**Notes**

All five focused controller and command-model cases pass: uploads bind the captured head, commit, path and fingerprint; oversized bytes stop before transport; commands retain every path and reject omissions or foreign terms.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture
```

**Notes**

Production and catalog Svelte diagnostics report no errors or warnings, catalog boundaries cover 51 modules, and all 186 governed modules stay within the 300/400/500-line budgets after extracting conflict and index controller concerns.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-mutations/StoreConflictEditor.stories.svelte --reporter=dot
```

**Notes**

All four real-control interaction/accessibility stories pass for atomic term selection, bounded staged content, unknown-outcome recovery and a stale fingerprint that remains visible without implicit reread. Stories remain visual-pending.
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
pnpm web:build
```

**Notes**

The production bundle builds with the conflict editor and existing CSP assets. Existing Design Core :global selector and bundle-size diagnostics remain warnings rather than failures.
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
pnpm exec playwright test --config web/playwright.config.ts conflicts.spec.ts
```

**Notes**

The cold built-browser regression passes 1/1 and proves the exact upload query binding plus one unchanged complete atomic preview body.
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
pnpm web:test:real
```

**Notes**

Three of four real cases passed. Conflict selection timed out because demo seeding left the checkout working copy on the conflict merge, so an earlier external publication rewrote that bookmark-visible description. The seed now creates a separate working copy after preserving conflict/demo.
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
pnpm web:test:real
```

**Notes**

All four cold built-browser cases pass against a fresh native repository and pinned RustFS: snapshot-bound Tantivy search, external publication with explicit SSE refresh, atomic conflict resolution and another durable mutation. The public lane starts through pnpm dev:restart and removes the disposable stack through pnpm dev:stop.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first; pnpm check:progress; pnpm format:check
```

**Notes**

Spec-first passed for 30 protected files across five canonical chapters and formatting passed. The progress guard only recognizes named test functions rather than Svelte Story declarations, so exact Storybook coverage remains recorded by its focused run instead of invented named-test references.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools VITEST_MAX_WORKERS=1 pnpm test
```

**Notes**

The aggregate reached 67/68 passing Storybook cases before axe scanned the mobile application while its virtualized bookmark options had not mounted, leaving a transient empty listbox. Design Core's Empty primitive already renders the required option role; the stable mobile story now waits for its representative main option before accessibility scanning.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryApplication.stories.svelte --reporter=dot; VITEST_MAX_WORKERS=1 pnpm test:storybook
```

**Notes**

All four application stories and all 68 Storybook interaction/accessibility cases pass after the mobile story waits for its virtualized bookmark row. The check changes no runtime behavior and Design Core requires no source repair.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools VITEST_MAX_WORKERS=1 pnpm spec:check
```

**Notes**

The canonical aggregate stopped at catalog type checking because Storybook's typed findByRole overload does not expose the exact option accepted by the runtime query helper. The unambiguous main query now uses the supported name-only form.
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
pnpm catalog:check; pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryApplication.stories.svelte --reporter=dot
```

**Notes**

Catalog diagnostics and boundaries pass with no warnings, and the four application stories pass after the type-only query correction.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools VITEST_MAX_WORKERS=1 pnpm spec:check
```

**Notes**

Cold catalog, diagnostics and unit lanes passed before the interaction lane exposed an existing search-unavailable teardown race. Its final assertion synchronously queried a portal dialog that may already be unmounted while Storybook replaces the story.
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
pnpm catalog:check; pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-search/StoreSearchCommand.stories.svelte --reporter=dot (twice); VITEST_MAX_WORKERS=1 pnpm test:storybook
```

**Notes**

Catalog diagnostics pass, the three search stories pass in two consecutive fresh runs and all 68 Storybook interaction/accessibility cases pass. The fixture now exposes its authoritative bound open state outside the portal, and the story awaits initial status before closing.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

Specification validation, 11 package boundaries, the 186-module architecture budget, progress, rustfmt, 51 actual HTTP/SSE responses, six contract tests, production/catalog diagnostics and locked-workspace Clippy pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools VITEST_MAX_WORKERS=1 pnpm test
```

**Notes**

The complete non-visual aggregate passes 23 governance, three storage-harness, six contract, 40 production web, 34 catalog unit and 68 Storybook interaction/accessibility cases, cold catalog browser acceptance and the full locked Rust workspace. Declared external-service cases remain ignored in this normal lane.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

The cold Storybook catalog, production browser bundle and complete locked Rust workspace build successfully. Existing Design Core :global selector and bundle-size diagnostics remain warnings rather than failures.
<!-- /run -->
<!-- /evidence-field -->

