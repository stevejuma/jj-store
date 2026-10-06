---
schema: 1
id: E-UI-055
---

# E-UI-055 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/AppearanceComposer.stories.svelte
```

**Notes**

Before: inset white editor on a popover shell. Initial foreground/diagnostic retries each fail once; these expose empty cyclic aliases when the reverse Mira bridge is placed on its own host. /tmp/jj-composer-palettes-before.log, after.log, text.log, diagnostic.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/AppearanceComposer.stories.svelte
```

**Notes**

Ancestor bridge and transparent editor pass 1/1 across the original 62 palettes; app-background and Buzz styling retries also pass. /tmp/jj-composer-palettes-fixed.log and white.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/AppearanceComposer.stories.svelte
```

**Notes**

New ArrowUp dependency initially hit a Tailwind import declaration failure while owned temporary Storybook was running; no test cases executed. Stopped the owned inspection server/tab and repeated cold. /tmp/jj-composer-palettes-buzz.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/AppearanceComposer.stories.svelte
```

**Notes**

Cold process passes 1/1. Toolbar colour assertion subsequently waits for the existing 160ms transition without relaxing equality; all 62 text/placeholder/retained-draft checks pass. /tmp/jj-composer-palettes-buzz-cold.log and final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/AppearanceComposer.stories.svelte
```

**Notes**

Live inspection found square send geometry. Added exact circular-radius regression: radius8 at width32 fails. Existing foundation send-button part fixes geometry. /tmp/jj-composer-send-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/AppearanceComposer.stories.svelte catalog/components/store-threads catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-mutations/StoreRepositoryActions.stories.svelte catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

Initial 34 passed/1 failed assertion (animated toolbar colour) and 3 Bits teardown derived_inert errors. An exact actions retry reproduces that assertion. /tmp/jj-composer-consumers.log and palettes-actions.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/AppearanceComposer.stories.svelte catalog/components/store-threads catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-mutations/StoreRepositoryActions.stories.svelte catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

Final assertions pass 35/35, including circular geometry; process remains non-green due to three Bits dismissible/scrollbar derived_inert errors. No diagnostic suppression. /tmp/jj-composer-consumers-final.log.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
python3 /tmp/jj-composer-baseline.py
```

**Notes**

Temporarily restored the three production composer files from @- and ran only EditAndRename|PreviewAndApply|UnknownOutcomeRecovery; 3 assertions pass, 3 skipped, 2 inherited Bits teardown errors. All files restored byte-for-byte in finally. /tmp/jj-composer-consumer-baseline.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/AppearanceDialog.stories.svelte
```

**Notes**

Before: normal and confirmation modal backgrounds are rgb235 rather than white. /tmp/jj-modal-palettes-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/AppearanceDialog.stories.svelte
```

**Notes**

After background binding, exact WorkStore field-value assertion exposes muted rather than primary text; next dark transition frame still has old modal fill. /tmp/jj-modal-palettes-background.log.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/AppearanceComposer.stories.svelte catalog/components/store-appearance/AppearanceDialog.stories.svelte
```

**Notes**

Composer passes; both modal stories initially read the previous background during foundation transition:all 100ms. Read-only CUA inspection confirms settled roles are correct; waitFor retains exact colour assertions. /tmp/jj-composer-modal-palettes-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/AppearanceDialog.stories.svelte
```

**Notes**

Two stories pass across all original 62 palettes after waiting for settled modal colours. Values use foreground and helpers remain muted. /tmp/jj-modal-palettes-settled.log.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/appearance.test.ts -t "restores the original theme"
```

**Notes**

Before: unknown original preference falls back to Buzz; 7 filtered skips. /tmp/jj-original-theme-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/appearance.test.ts
```

**Notes**

After adding Original first in the catalog, the existing unpaired Light fallback accidentally changes from Buzz to Original. Preserved the prior fallback explicitly in switchMode; 7 other tests pass. /tmp/jj-original-theme-units-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/appearance.test.ts
```

**Notes**

All 8 preference/link/draft tests pass, including Original persistence and System/Light/Dark pairing. /tmp/jj-original-theme-units-corrected.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance
```

**Notes**

7/7 across three files, with axe: Original picker/mode comparison, all controls, and exact composer/normal/confirmation roles across all64 variants (62 Buzz styles plus Original pair). /tmp/jj-theme-alignment-exact.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit
```

**Notes**

215/215 across36 files. Wider unit lane is justified by the shared theme registry/preference parser/mode fallback consumed app-wide. /tmp/jj-theme-alignment-all-unit.log.
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
pnpm web:check; pnpm catalog:check; pnpm check:ui-ownership; pnpm check:architecture
```

**Notes**

Initial and expanded targeted checks pass: zero errors/warnings, final129 catalog and719 production modules. /tmp/jj-theme-alignment-{web-check,catalog-check,ownership,architecture}.log and earlier /tmp/jj-composer-*-complete.log.
<!-- /run -->
<!-- /evidence-field -->

