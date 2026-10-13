---
schema: 1
id: E-WORKFLOW-CHOICES-001
---

# E-WORKFLOW-CHOICES-001 details 2

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
jj --no-pager st
```

**Notes**

Before: empty mlswrpuo at 8225d77b, parent xnsuxmul b2bc8e3c. Preserve existing .local/dev-demo and the user browser draft.
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
Read-only admitted GET /v1/repositories, /v1/directory/profile, /v1/repos/<native-home>/threads?limit=50 and /v1/local/agents
```

**Notes**

Existing API worked: server-only and code repository had zero threads; personal metadata home had three threads. Own profile includes display name and blob avatar; agent catalogue includes Nyra. Browser admission headers were retained. Operator endpoints returned expected access_denied in LocalTrusted; no access policy changed.
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
pnpm web:check
```

**Notes**

0 errors and 0 warnings, initial and final checks.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Initial fixture shape errors (native graph timestamp/visibility/bookmark flags) and test selector typing were repaired. Later catalog ownership rejected a direct API type import; it now imports the existing controller type surface. Final Svelte check: 0 errors and 0 warnings; pnpm exec node scripts/check-catalog.mjs passes 423 modules.
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
pnpm exec node scripts/check-catalog.mjs
```

**Notes**

After repairing the fixture-only API type import: catalog boundaries pass, 423 modules.
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
pnpm check:architecture
```

**Notes**

Initial new composition exceeded component line limits; extracted the existing text-condition row and picker stylesheet into owned local files. Final pass: 1808 modules.
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
pnpm check:ui-ownership
```

**Notes**

Pass: owning declarations cover new workflow field/helper and maintained local stylesheet; no Community source or catalog production dependency.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/repository-thread-navigation.test.ts web/tests/vite/history-language.test.ts
```

**Notes**

10 tests pass. Fresh shared thread reads observe data after a cached empty page without invalidating active detail reads.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/workflow-code-options.test.ts
```

**Notes**

3 tests pass: pinned operation/time across pages, stale source reply refusal and foreign snapshot rejection.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-workflows/WorkflowChoices.stories.svelte
```

**Notes**

Initial new stories: 1 passed, 2 failed due an unclosed previous modal and a test counting the portaled toast as inline. Corrected cleanup and inspector-specific assertion. Second run exposed searchbox versus textbox selector; corrected to the component accessibility role.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-workflows/WorkflowChoices.stories.svelte catalog/components/store-workflows/WorkflowTriggers.stories.svelte catalog/components/store-workflows/WorkflowEditor.stories.svelte catalog/components/store-workflows/StoreWorkflows.stories.svelte catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-application/StoreRepositoryPickerPending.stories.svelte
```

**Notes**

20 tests pass in 6 files. Cold choices fetch profiles/avatars, unavailable agent rows, cross-home thread selection and retained YAML anchors; highlighted revsets, source author replacement, four failure toasts and existing editor/picker behavior pass. Storybook runtime diagnostics guard remains active.
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
Cold fresh in-app browser tab at http://127.0.0.1:7460/#/workflows
```

**Notes**

Current native dev instance: existing threads displayed with canonical metadata owner; choosing one updated source and thread together. Own server avatar displayed as a hydrated blob image; Nyra unavailable author row visible. Code author Maya Chen fetched without opening History; query bookmarks("main") &amp; all() highlights. Only unsaved temporary drafts, no workflow commands sent.
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
pnpm docs:review application "Workflow trigger choices discover source history and admitted thread homes, hydrate server profile names and avatars, and reuse the highlighted revset editor. Agent setup IDs remain unavailable author filters because the existing backend event contract only exposes human principals."
```

**Notes**

Owning application chapter updated; public HTTP/schema/native storage contracts unchanged. pnpm docs:review passes.
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
pnpm check:progress; pnpm spec:first
```

**Notes**

Initial fresh evidence file list missed the subsequently updated M28 ledger and verification page. Added both paths to this record; no requirement was weakened.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Mapped chapters also required API, documentation and governance confirmations for browser/controller source, coverage and provenance changes. Added focused unchanged-wire/ownership confirmations, retaining requirements and checking the mapped gate again.
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
PATH=/Users/stevejuma/.local/bin:$PATH pnpm docs:check
```

**Notes**

143-file documentation closure; links/anchors valid; 9 Mermaid diagrams rendered in light/dark without network access.
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
Final cold in-app browser reload and read-only workflow picker interaction
```

**Notes**

All three existing native threads load with owner labels, steve displays the server blob avatar, and Nyra has an unavailable author row. No error/warning console diagnostics. Screenshots: /tmp/jj-store-workflow-thread-choices-final.png and /tmp/jj-store-workflow-author-choices-final.png. No source/workflow mutation, installed service restart or reset.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/workflow-code-options.test.ts
```

**Notes**

Final 4 tests pass, including exclusion of independently owned local companion actor suggestions. Source pin/time, stale reply and mismatched history checks retained.
<!-- /run -->
<!-- /evidence-field -->

