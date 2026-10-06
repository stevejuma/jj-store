---
schema: 1
id: E-M21-056
---

# E-M21-056

<!-- evidence-field:id markdown -->
## Id

E-M21-056
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M21
- M21.1
- M21.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

ui
<!-- /evidence-field -->

<!-- evidence-field:change markdown -->
## Change

Enclosing Jujutsu change: Use a Command View picker for workspace conflict versions
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Replace introductory text and version-button row with a searchable Command View popover using native-order text summaries. Remove the second trailing inset from the shared workspace/native conflict surface through the public Shell token.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-stack-workspace/StoreStackWorkspaceConflictVersions.stories.svelte
- docs/maintenance/coverage.json
- docs/site/guides/editing.md
- spec/src/api.md
- spec/src/documentation.md
- spec/src/milestones.md
- spec/src/ui.md
- web/src/lib/ui/store-conflicts/StoreConflictSurface.svelte
- web/src/lib/ui/store-conflicts/store-conflicts.css
- web/src/lib/ui/store-stack-workspace/StoreStackConflictVersionPicker.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackConflictVersions.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceConflict.svelte
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; pnpm 10.32.1; Svelte 5.57.1; existing Design Core validation dependency unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No HTTP/RPC, native command, TOML, layout, manifest, WAL or durable-state changes. Version details derive only from bounded native text; no commit provenance is fabricated.
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
detail: "Before picker: 2 failed/1 passed; buttons/intro remained and no command
  picker existed. before.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceConflictVersions.stories.svelte
```
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
detail: "Initial picker: 2 failed/1 passed; Escape reached the shell surface and
  Command Root supplied a different accessible input name. Added owned
  nested-open dismissal coordination and aligned the public root label;
  after.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceConflictVersions.stories.svelte
```
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
detail: "Observed outer surface right 1272px and inner surface right 1264px: a
  second 8px inset. The existing public
  --ui-shell-surface-layer-inline-end-inset token owns this presentation
  setting."
hasNote: false
noteEncoding: markdown

```

```shell
CUA live body/surface geometry before trailing-inset configuration
```
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
detail: "Fresh browser process: 14 passed. Covers search and keyboard selection,
  summaries, disabled absent versions, Escape preserving editor/trigger focus,
  full measured body coverage within 1px, retained drafts and complete-file
  preview/apply. final-stories.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceConflictVersions.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte
```
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
detail: 0 errors/0 warnings; web.log.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm web:check
```
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
detail: 0 errors/0 warnings, 242 catalog modules; catalog.log.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm catalog:check
```
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
detail: 1134 modules pass source ownership/size boundaries; architecture.log.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:architecture
```
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
detail: 1 validator/23 chapters/173 requirements; ownership.log. Uses public
  Command View, Popover and Shell token; no dependency patches or upstream
  edits.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:ui-ownership
```
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
detail: "Live 7476: descriptive paragraph absent, versions show exact line/byte
  counts and a bounded first differing-line preview; trailing surface gap is 0px
  and Escape retains Edit Resolved. Proof live-picker.png. No resolution
  applied."
hasNote: false
noteEncoding: markdown

```

```shell
CUA live reload, version picker, geometry and Escape check
```
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
detail: Focused formatting and reviewed application source/manual fingerprint;
  no backend or persisted formats changed.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --write <changed implementation/story/canonical/manual files>; pnpm docs:review application
```
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
detail: 139-file/14,823,332-byte closure, links and anchors pass; docs.log.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm docs:check
```
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
detail: Mapped API chapter was required for the workspace conflict composition;
  added explicit frontend-only consumption/no transport-change requirement.
  spec-first.log.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm spec:first
```
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
detail: Mapped chapter coverage, 5 validators/23 chapters/173 requirements and
  142-task progress bookkeeping pass;
  spec-first-final.log/spec.log/progress.log. Historical 594 evidence entries
  retained.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm spec:first; pnpm spec:validate; pnpm check:progress
```
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
detail: Review CLI output needed formatting in docs/maintenance/coverage.json;
  corrected without changing its source fingerprint. format.log.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --check <12 changed files>
```
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
detail: Final focused evidence/source formatting; format-final.log. No product
  source changed.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --write docs/maintenance/coverage.json spec/src/milestones.md; pnpm exec prettier --check <12 changed files>
```
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

<!-- evidence-field:next markdown -->
## Next

Keep the existing local preview available; native conflict execution and separately deferred unsupported shapes/release/visual qualification remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:summary markdown -->
## Summary

The picker retains reviewed merge text when switching versions and keeps deletion choices visibly disabled. Command labels/search include exact line and UTF-8 byte counts plus at most 100 characters of content preview. Owned nested-open state suppresses shell Escape dismissal while the picker is active. Both native and workspace editors cover the measured body boundary without the inherited second 8px inset.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-056 details 2](details-02.md)
