---
schema: 1
id: E-UI-099
---

# E-UI-099 details 2

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
pnpm search remark xml --json; package metadata and README review for remark-mdx and remark-directive
```

**Notes**

No suitable drop-in tolerant XML prompt renderer found. Directive syntax differs; MDX imposes JSX/expression syntax. Research retained in ignored .local/xml-plugin-research.json, remark-mdx-readme.md and remark-directive-readme.md.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentPromptMarkup.stories.svelte (before)
```

**Notes**

Both activity and recorded chunk paths lack the arbitrary &lt;harness:context&gt; disclosure. .local/agent-prompt-markup/before.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 10
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentPromptMarkup.stories.svelte (repair runs)
```

**Notes**

Four repair runs: 3 sanitizer marker failures, 3 normalized-link assertions, 2 immediate collapse visibility assertions, 2 incorrect unmount assertions. Safe filtering strips className on div; switch marker to inert title. Bits UI retains hidden content, so await hidden visibility. Final exact run below passes.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check (before fixture fixes)
```

**Notes**

Fixture media_type widened to string; an embedded script-like fixture string confused Svelte diagnostics. Add literal typing and move sample to TypeScript fixture; no diagnostic suppression.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/prompt-elements.test.ts (initial)
```

**Notes**

7 parser boundary tests.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentPromptMarkup.stories.svelte catalog/components/store-application/AgentToolGroups.stories.svelte catalog/components/store-application/AgentLayout.stories.svelte catalog/components/store-application/NativeMessageActions.stories.svelte catalog/components/store-application/AgentComments.stories.svelte
```

**Notes**

24 unchanged direct consumer stories pass; 2 new prompt collapse assertions fail and 2 other new stories pass. Only the new assertions were repaired; no subsequent production change. .local/agent-prompt-markup/stories.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentPromptMarkup.stories.svelte (final)
```

**Notes**

5 interactions/accessibility pass: arbitrary nesting, recorded chunks, malformed and code literals, inert attributes/cross-section references, default safe HTML behavior. Stories remain visual-pending. .local/agent-prompt-markup/after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/prompt-elements.test.ts web/tests/vite/agent-timeline.test.ts web/tests/vite/projects-agent-response.test.ts
```

**Notes**

13 tests across 3 files. Generic Unicode names/attributes, balance/escape/depth/code boundaries and direct timeline/response consumers. .local/agent-prompt-markup/units.log.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

Final web and catalog checks: 0 errors/0 warnings, catalog boundaries 172 modules. Web check run before and after sanitizer-marker repair; catalog rerun after both fixture errors and ordinary-renderer fixture addition.
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
pnpm web:build
```

**Notes**

Fresh production bundle and manifest generated. .local/agent-prompt-markup/build.log.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config .local/agent-prompt-markup/playwright.config.ts agent-prompt-markup.spec.ts (first run)
```

**Notes**

Initial ignored config used the preview executable, whose compiled asset path points to the older preview bundle irrespective of cwd. Corrected to existing main executable; no Rust rebuild.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config .local/agent-prompt-markup/playwright.config.ts agent-prompt-markup.spec.ts (repair runs)
```

**Notes**

Three subsequent runs: 2 unopened Copy submenu failures; 1 touch/mobile fixture failure because viewport-only emulation retains desktop hover actions clipped by the existing sidebar header; 1 full-document mobile Axe run exposes existing landmark-one-main/page-has-heading-one findings while main is hidden behind the response sidebar. Use actual touch emulation and scope Axe to changed transcript. Document-level mobile landmarks remain a separate UI gap.
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
pnpm exec playwright test --config .local/agent-prompt-markup/playwright.config.ts agent-prompt-markup.spec.ts (final)
```

**Notes**

3 cold built-bundle tests pass. Desktop activity and touch-mobile recorded chunks prove Markdown before/inside/between/after, keyboard disclosures, measured virtual row growth/no overlap, exact original Copy message, no horizontal page overflow and transcript-scoped Axe. Third test proves inert tags/attributes/no network leaks and visible malformed/indented fallback. .local/agent-prompt-markup/browser.log.
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
pnpm check:architecture; pnpm check:boundaries; pnpm check:ui-ownership
```

**Notes**

890 UI modules, 17 packages, 1 ownership validator/22 chapters/167 requirements.
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
pnpm exec prettier --write <changed production, fixture, test and canonical chapter files>; pnpm exec prettier --check <same files>
```

**Notes**

All new/changed source and canonical paragraphs formatted. Historical milestone ledger encodings retained verbatim; only the new evidence entry is formatted.
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
jj rebase -r @ -o tkwpktrq (empty codex-preview workspace); CUA reload7470 and inspect live Codex session
```

**Notes**

Preview clean before rebase; source refreshed through Vite HMR with backend/fixture untouched. Actual captured &lt;environment_context&gt;, &lt;filesystem&gt; and &lt;cwd&gt; disclosures expand into nested controls and original content. Screenshot .local/agent-prompt-markup/live-preview.png; tab retained for user review.
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
pnpm spec:first (initial updated ledger)
```

**Notes**

Mapped web scope also requires api.md. Add a browser/transport boundary paragraph preserving exact recorded text and identity; no schema generation needed.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress (final updated canonical chapters and ledger)
```

**Notes**

Spec-first:14 protected files/5 canonical chapters; validation:5 validators/22 chapters/167 requirements; progress:136 tasks. All historical evidence retained; no milestone status promoted.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

