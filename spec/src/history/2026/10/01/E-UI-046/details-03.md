---
schema: 1
id: E-UI-046
---

# E-UI-046 details 3

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
pnpm spec:validate
```

**Notes**

131 requirements validated.
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
pnpm check:progress
```

**Notes**

Final ledger names/removal/chapter references verified.
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
python3 .scratch/format-native-slice.py
```

**Notes**

Formats the existing changed files and verifies their formatting; removed paths excluded.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One native collaboration presentation/controller surface and its direct UI consumers. Fresh Vitest browser processes, exact controller tests, focused type/boundary/architecture checks and cold web output. Root aggregates, unrelated Rust and full Storybook remain out of scope. Main settings workspace is untouched. Subsequent repository Projects/activity-filter/document changes belong to a separate slice.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing V1 ReviewStore edit_comment and safe_markdown/review-markdown-v1; no HTTP schema, Rust, WAL or repository format change. In-memory WorkDraft retains full number|bigint comment/task versions. Readable native IDs copy complete values; only exact recorded Git/JJ associations are inline. Review drafts are memory-only and clear on matching committed request after an unknown outcome. Captured agent transcripts remain immutable.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; JJ 0.45.1; Node 24.10.0; pnpm 10.32.1; existing pinned Svelte, Community, Design Core and Chromium.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Complete repository-linked Projects tab, document Markdown preview and requested project Activity filter; preserve stored demo data during owned restart. Visual approval remains separate.
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- Visual baseline approval remains pending.
- Shared release and independent deployment qualification remain deferred.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

Ignored .scratch/native-actions-*.log; no baseline changes or seed mutations.
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

- web/src/lib/ui/store-work/StoreWorkMessageActions.svelte
<!-- /evidence-field -->

<!-- evidence-field:stories list -->
## Stories

- catalog/components/store-application/NativeMessageActions.stories.svelte#Native
  copy reply and own author edit
- catalog/components/store-application/NativeMessageActions.stories.svelte#Read
  only and other author comments expose copy without edit
- catalog/components/store-application/NativeMessageActions.stories.svelte#Unknown
  edit retains original context and draft
- catalog/components/store-application/NativeAgentMenus.stories.svelte#Native
  identifier menus preserve selection and exact values
- catalog/components/store-application/StoreApplication.stories.svelte#Mobile
  details control stays at toolbar end
- catalog/components/store-application/StoreCollaboration.stories.svelte#Agent
  response activity
- catalog/components/store-work/StoreWork.stories.svelte#Native threaded
  discussion
- catalog/components/store-work/ProjectsHub.stories.svelte#Landing tabs sidebar
  hierarchy reviews and icon empty state
<!-- /evidence-field -->

