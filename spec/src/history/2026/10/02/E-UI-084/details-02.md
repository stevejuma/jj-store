---
schema: 1
id: E-UI-084
---

# E-UI-084 details 2

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
jj --no-pager st; jj --no-pager diff; inspect Community and released Design Core InputGroup/Button; read official shadcn-svelte component docs
```

**Notes**

Clean tlnzznzu above zqzuymro/19a7bf96 before implementation. Threads had no toolbar search and a1px bottom border. Acceptance target is shared search geometry/interaction, loaded title/preview filtering and a0px toolbar border.
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
pnpm test:storybook catalog/components/store-threads/StoreThreadSearch.stories.svelte (before)
```

**Notes**

Two exact regression failures: Search threads button absent. /tmp/jj-thread-search-before.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-threads/StoreThreadSearch.stories.svelte (first implementation)
```

**Notes**

Returning to the directory reset the query because the effect tracked a reconstructed route object. Derived the repository identity separately before resetting, preserving the query for same-owner selection. /tmp/jj-thread-search-after.log.
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
pnpm test:storybook catalog/components/store-threads/StoreThreadSearch.stories.svelte (second)
```

**Notes**

2/2 desktop/mobile search stories passed after owner-scoped reset fix. /tmp/jj-thread-search-after-second.log.
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
pnpm web:check (initial)
```

**Notes**

0errors0warnings. /tmp/jj-thread-search-web.log.
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
pnpm catalog:check (initial)
```

**Notes**

New fixture directory summary lacked required message_count. Added message_count:1; no product contract or assertion changes. /tmp/jj-thread-search-catalog.log.
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
pnpm exec prettier --write (changed files and subsequent fixture/story/header/chapter updates)
```

**Notes**

Changed source, regression, fixture and canonical prose formatted. /tmp/jj-thread-search-format.log.
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
pnpm check:architecture; pnpm spec:first (initial)
```

**Notes**

794modules with unchanged budgets;9protected files/2canonical chapters. /tmp/jj-thread-search-architecture.log; /tmp/jj-thread-search-spec-first.log.
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
pnpm test:storybook catalog/components/store-threads/StoreThreadSearch.stories.svelte (final regression)
```

**Notes**

3/3 in a fresh browser/Vite process: desktop/mobile geometry, Clear/Close/Escape/focus, title/preview filtering, retained query through selection, no-match copy, filtered pagination, repository switch reset and0px toolbar border. /tmp/jj-thread-search-final-regression.log.
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
pnpm test:storybook catalog/components/store-threads catalog/components/store-work/ProjectsFilters.stories.svelte catalog/components/store-work/LayoutControls.stories.svelte catalog/components/store-work/NavigationPersistence.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/ProjectsAgents.stories.svelte catalog/components/store-work/ProjectsActivityControls.stories.svelte
```

**Notes**

49/49 across12 directly affected story files in a fresh browser/Vite process. /tmp/jj-thread-search-consumers.log.
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
pnpm web:check; pnpm catalog:check; pnpm check:ui-ownership; pnpm spec:first (qualified)
```

**Notes**

Both Svelte checks0errors0warnings;149catalog modules; ownership1validator/21chapters/147requirements;9protected files/2chapters. /tmp/jj-thread-search-web-final.log; /tmp/jj-thread-search-catalog-final.log; /tmp/jj-thread-search-ownership.log; /tmp/jj-thread-search-spec-first.log.
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
Read-only live Threads toolbar verification through CUA temporary background tab
```

**Notes**

Expanded and collapsed trigger both x263,y52.5,width32,height32; toolbar border0px. Submitted test and observed only matching title/preview. Saved and inspected full viewport screenshot. Temporary tab closed; user tab untouched. No repository data writes or service restart. One generic-role selector timed out; verified the labeled toolbar through its accessible label.
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
pnpm check:architecture; pnpm check:ui-ownership; pnpm spec:first; pnpm spec:validate; pnpm exec prettier --check (13 changed files)
```

**Notes**

Final source and initial ledger checks:794modules, unchanged budgets; ownership1validator;10protected files/3chapters; specification5validators/21chapters/147requirements;13changed files formatted. /tmp/jj-thread-search-{architecture-final,ownership-final,spec-first-final,spec-validate,format-check}.log.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress (initial ledger)
```

**Notes**

Two invalid test references: Storybook file paths need function names in the test-reference schema. Story evidence remains in files and completed runs; removed the unsupported test references. No test assertions or product behavior changed. /tmp/jj-thread-search-progress.log.
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
pnpm exec prettier --write spec/src/milestones.md; pnpm check:progress (qualified)
```

**Notes**

Ledger formatted;120tasks pass after removing unsupported Storybook test references. /tmp/jj-thread-search-ledger-format-final.log; /tmp/jj-thread-search-progress-qualified.log. This evidence insertion is checked again before the standing JJ commit.
<!-- /run -->
<!-- /evidence-field -->

