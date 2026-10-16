---
schema: 1
id: E-UI-INLINE-LINKS-001
---

# E-UI-INLINE-LINKS-001 details 2

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
jj --no-pager st; inspect current UI/Appearance contracts, M16, Community then released Mira/Design Core, and Buzz sources
```

**Notes**

Clean plzrnqys over d441cdae in jj-store-forge-stacks. Existing separate test work was untouched. Buzz ef2aa1ae38fadcc0bc22b8bf6ed96b35933146be leaves ordinary links inline and adds prepared snapshot cards separately; standalone-only cards and general local brand icons are deliberate jj-store adaptations. Community a5f64a4c7dea2f29b4058aa7666a98c4c0dee656 was inspected first; no reference code dependency or foundation change.
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
pnpm spec:search -- JST-UI-018
```

**Notes**

Installed spec-validator rejects the literal -- forwarded by pnpm. Corrected invocation below; no tooling change.
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
pnpm exec spec-validator search JST-UI-018
```

**Notes**

Search reported unavailable QMD native modules and supplied the rg fallback. Used rg -n JST-UI-018 spec/src/current/verification spec/src/current/contracts and opened the canonical UI chapter. QMD repair is outside this slice.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/appearance.test.ts
```

**Notes**

Before adding regression: 12 tests passed. Baseline converted all safe website anchors to cards and defaulted new preferences to Compact.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/appearance.test.ts
```

**Notes**

After adding focused regression, before production edits: 2 failed / 11 passed. New preference expected Inline; ordinary anchor expected inline renderer. Log jj-store-inline-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/appearance.test.ts
```

**Notes**

After renderer/default change: 13 passed; both original regressions resolved.
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
pnpm web:check
```

**Notes**

Initial HAST paragraph conversion needed an explicit defined-href guard. Fixed locally; subsequent checks have zero errors/warnings.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

New stories initially used Playwright exact options with Testing Library getByRole. Removed unsupported options; exact string names already match exactly. Subsequent catalog check has zero errors/warnings.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/StoreLinkRendering.stories.svelte
```

**Notes**

Initial rendered-link interaction tests reached the destination hint but could not find tooltip semantics. Added explicit role=tooltip and simulated keyboard focus using Tab/Shift+Tab instead of relying on programmatic focus modality. No assertion was removed.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/StoreLinkRendering.stories.svelte catalog/components/store-appearance/StoreAppearance.stories.svelte catalog/components/store-forge/StoreForgeLinks.stories.svelte
```

**Notes**

Intermediate: 6 passed / 4 failed. Three destination-tooltip tests were corrected as above. One dark PR full-body accessibility failure belongs to unchanged removed diff text. Log jj-store-inline-interactions.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/StoreLinkRendering.stories.svelte
```

**Notes**

3 stories passed in a fresh Chromium process, including Inline, Compact-only eligible content, narrow dark Rich, preserved formatted labels, code/quote/list exclusions, wrapping, keyboard destination hints/Escape and no external resource requests. Log jj-store-inline-stories.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/appearance.test.ts web/tests/vite/site-links.test.ts
```

**Notes**

62 passed before adding media/native exclusions. Then 65 passed with direct transcript-unit coverage; final expanded run is below.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/StoreAppearance.stories.svelte catalog/components/store-application/ConversationMarkdown.stories.svelte catalog/components/store-application/MarkdownReading.stories.svelte catalog/components/store-forge/StoreForgeWorkspace.stories.svelte catalog/components/store-forge/StoreForgeFloating.stories.svelte catalog/components/store-forge/StoreForgeLinks.stories.svelte
```

**Notes**

15 passed / 1 failed across 16 directly affected stories. All functional interactions passed, including preference previews/persistence, PR body navigation, float/redock, existing submission/description flows and file/list rendering. The dark PR after float/redock fails Axe on two unchanged diff removed-text spans: contrast 4.34 versus 4.5, colors #e1e4e8 / #7b6268. This exact diff contrast gate was already recorded in E-FORGE-INTEGRATIONS-001 and E-FORGE-AUTH-LIVE-001. Log jj-store-inline-consumers.log; no diff code or accessibility suppression was changed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeWorkspace.stories.svelte -t DarkForgeButtonsAlignWithTitlesAndRetainHoverContrast
```

**Notes**

Existing dark-button story passed (1 passed / 5 filtered). This story does not exercise the same post-redock visible diff state; it is not substituted for the failing dark PR accessibility gate.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeChecksLayout.stories.svelte -t GroupedChecksInDarkMode
```

**Notes**

Existing grouped-check dark story passed (1 passed / 2 filtered). Its expanded checks alter visible diff exposure; the new post-redock dark PR failure remains explicit.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeLinks.stories.svelte -t StackNavigationInDarkPRDetails
```

**Notes**

After aligning both global and fixture dark mode with existing dark stories: same unchanged two diff contrast findings. Functional inline-link identity and float/redock assertions pass. No further unrelated repairs or retest loops. Log jj-store-inline-dark-final.log.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check <changed production, tests, stories and provenance files>
```

**Notes**

The newly added media test needed formatting. Ran Prettier only on changed slice files; final formatting check recorded separately.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/appearance.test.ts web/tests/vite/site-links.test.ts catalog/fixtures/delta-chat/markdown-reading.test.ts
```

**Notes**

Final: 66 tests passed in 3 files. Includes preference/default/restoration/nonpersisted previews, all initial site identities and aliases, lookalikes, safe addresses, content classifiers, top-level paragraph context, explicit labelled URL exclusions, stale eligibility, formatted children, native entity priority and file/mail/media/code preservation. Log jj-store-inline-unit-final.log.
<!-- /run -->
<!-- /evidence-field -->

