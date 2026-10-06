---
schema: 1
id: E-UI-028
---

# E-UI-028 details 5

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
pnpm exec prettier --write spec/src/api.md spec/src/ui.md spec/src/spec-governance.md spec/src/milestones.md && pnpm spec:first && pnpm check:progress
```

**Notes**

The API, UI, acceptance and progress chapters are formatted; all 14 protected files map to four current canonical chapters and the evidence ledger remains valid.
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
pnpm check:architecture
```

**Notes**

The responsive tab measurement pushed the existing repository directory component to 374 counted lines and its stylesheet to 333, above the 300-line production module budget.
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
pnpm web:check && pnpm catalog:check && pnpm check:architecture
```

**Notes**

Both Svelte projects, catalog boundaries and all 257 architecture modules pass after extracting the 232-line repository-document component and its 182-line stylesheet.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryDirectory.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte
```

**Notes**

Both focused Chromium interaction/accessibility stories still pass after the repository-document extraction; visual status remains pending.
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

The production UI bundle and manifest rebuild pass; only the existing Lightning CSS :global and bundle-size warnings remain.
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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/mutations.spec.ts
```

**Notes**

The focused browser mutation lane did not start because another active jj-store workspace owned the fixed test port 7458; its process was preserved and no product assertion ran.
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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/mutations.spec.ts
```

**Notes**

After the independent workspace released port 7458, Playwright's default 30-second web-server limit expired before the quiet cargo-run preview command became ready; no browser assertion ran.
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
node scripts/browser-fixtures.mjs
```

**Notes**

The existing native fixture tools recaptured current HTTP/SSE and mutation browser fixtures without rebuilding Rust.
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
target/debug/examples/browser_preview; pnpm exec playwright test --config web/playwright.existing.config.ts web/tests/browser/mutations.spec.ts
```

**Notes**

All 5 focused production mutation browser cases pass against the rebuilt UI and already-built preview executable, including body unlock, request retention, unknown-outcome recovery and file edit/rename. The temporary no-webServer Playwright config was removed after the run.
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
pnpm dev:stop; restart the preserved demo with target/debug/jj-store; live browser verification at wide and 540px viewports
```

**Notes**

The demo restarted with the rebuilt UI and existing native binaries, without a Rust build. Live main-bookmark verification shows all five friendly document tabs at wide width, README/Code of conduct/Contributing plus More at 540px, License and Security in the menu, selected-document Edit, breadcrumb rename, persistent Preview/Source/Edit, body-only editing and the Save changes description dialog. A live search-result selection restored empty body overflow and pointer state, and the Commits tab remained clickable without refresh.
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
pnpm exec prettier --write <20 changed UI, test and canonical files> && pnpm spec:first && pnpm check:progress
```

**Notes**

All changed UI, focused test and canonical files are formatted; 16 protected files map to the four owning chapters and the targeted evidence ledger passes validation.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
focused repository-document source and GitHub overflow reference audit
```

**Notes**

The hidden selected document retained menu icons and a checkmark instead of the reference's bold label and leading active bar, while the square divided More trigger did not inherit the selected background or active underline.
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
pnpm web:check && pnpm catalog:check && pnpm check:architecture
```

**Notes**

Both affected Svelte projects, catalog boundaries and all 257 architecture modules pass for the overflow selected-state refinement.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryDirectory.stories.svelte
```

**Notes**

The focused Chromium interaction and automatic accessibility story passes. It verifies the rounded selected More trigger, visible active underline, bold selected Security item, icon removal and selected-item state; the story remains visual-pending.
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
pnpm web:build
```

**Notes**

The production UI bundle and manifest rebuild pass with the existing Lightning CSS :global and bundle-size warnings; no Rust build ran.
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
pnpm spec:first && pnpm check:progress
```

**Notes**

Progress validation passed, while spec-first required the already mapped API chapter to record that hidden-document selected styling remains a client presentation rule.
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
pnpm spec:first && pnpm check:progress
```

**Notes**

The API, UI, acceptance and progress chapters pass the spec-first and evidence-ledger gates for three protected UI files and four canonical chapters.
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
pnpm dev:stop; restart target/debug/jj-store with the preserved demo configuration; live 540px browser verification
```

**Notes**

The demo restarted with the rebuilt web bundle and existing native binaries. At 540px, selecting the hidden License document gives More the rounded active background and coral underline; the focused Storybook interaction separately verifies the open menu's bold label, leading bar and absence of icons.
<!-- /run -->
<!-- /evidence-field -->

