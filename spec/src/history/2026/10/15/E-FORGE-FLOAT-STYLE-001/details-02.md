---
schema: 1
id: E-FORGE-FLOAT-STYLE-001
---

# E-FORGE-FLOAT-STYLE-001 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte catalog/components/store-forge/StoreForgeFloating.stories.svelte -t 'PRSummaryStaysAboveChanges|CreateInlineAndFloat|ComposerWindowPreserves'
```

**Notes**

Before-state lifecycle baseline: 3 passed, 4 intentionally skipped. Source inspection found PanelsTopLeft controls, the same outer card border in inline/floating mode, shaded PR headers and a parent-owned 0.75rem gap. Log .local/forge-float-style-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte -t PRSummaryStaysAboveChanges
```

**Notes**

New regression before production edits: 1 failed, 5 skipped. The file diff remained 12px below its sections container after the PR floated. Log .local/forge-float-gap-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte -t PRSummaryStaysAboveChanges
```

**Notes**

First after-run had 1 failed test and 2 unhandled runtime errors while the new Lucide dependency was being optimized. This did not establish a layout result. Log .local/forge-float-gap-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte catalog/components/store-forge/StoreForgeFloating.stories.svelte catalog/components/store-forge/StoreForgeWorkspace.stories.svelte catalog/components/store-forge/StoreForgeHeader.stories.svelte
```

**Notes**

Interim overlapping browser processes: 6 failed, 9 passed and 7 unhandled errors. Vite explicitly logged optimization of @lucide/svelte/icons/picture-in-picture-2, dependency-change reload and unexpected test reload. Allowed these processes to finish and reran sequentially in fresh processes after optimization settled. No shared configuration, dependency patches, infrastructure repairs or weaker assertions. Log .local/forge-float-style-consumers.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte -t PRSummaryStaysAboveChanges
```

**Notes**

Fresh sequential exact regression: 1 passed, 5 intentionally skipped. Floating leaves no top gap before the file diff; redocking restores positive separation. Existing rendered details, CI inspection and close-file behavior still pass. Log .local/forge-float-gap-after2.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte catalog/components/store-forge/StoreForgeFloating.stories.svelte catalog/components/store-forge/StoreForgeWorkspace.stories.svelte catalog/components/store-forge/StoreForgeHeader.stories.svelte
```

**Notes**

Final fresh-process direct-consumer run: 15 stories passed. Includes editor identity/drafts, float/redock, native move/resize, minimize/restore, Escape/focus, viewport bounds, PR header/menu actions, shared PR+diff placement, explicit submission/recovery and failure toasts. Log .local/forge-float-style-consumers-final.log.
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
pnpm catalog:check
```

**Notes**

Zero Svelte errors/warnings and catalog boundaries pass (450 modules). Initial and final checks both pass; logs .local/forge-float-style-types.log and .local/forge-float-style-types-final.log.
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
node scripts/web-architecture.mjs
```

**Notes**

Maintained module-size/import boundaries pass (1956 modules).
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
pnpm check:ui-ownership
```

**Notes**

Existing owned UI composition and released dependency boundary pass; no new source copy or provenance mapping needed.
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
pnpm evidence:new -- --id E-FORGE-FLOAT-STYLE-001 --task M31 --requirement JST-UI-020 --requirement JST-FORGE-004
```

**Notes**

Created fresh same-change evidence/day links, restored 446 unrelated generator-only navigation changes from the parent and reformatted M31. Existing evidence remains unchanged.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
curl --max-time 3 http://127.0.0.1:6046/index.json
```

**Notes**

The warm review server hung during HMR. Verified PID 66156 belonged to this workspace and exact port/command; TERM did not stop it, then KILL stopped only that task-owned listener before restart.
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
pnpm spec:first
```

**Notes**

The first gate required the mapped ordinary-local Stacks API chapter. Added an explicit JST-API-001 unchanged-contract confirmation for presentation-only icons, surfaces and spacing; no transport, controller or admission behavior changed.
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
pnpm exec storybook dev --host 127.0.0.1 --port 6046 --exact-port --no-open --disable-telemetry
```

**Notes**

Restarted the task-owned review environment after source settled. index.json contains the contextual workspace story; index and iframe return HTTP 200. No screenshot comparison or baseline update.
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
pnpm check:progress
```

**Notes**

Fresh record and explicit unchanged M31 review pass.
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
pnpm spec:first
```

**Notes**

Owning UI presentation chapter and explicit unchanged authority/action confirmation cover the protected files.
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
pnpm exec spec-validator validate --only fileLimits,governance,summary
```

**Notes**

Document limits, governance and navigation pass.
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
pnpm exec prettier --check catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte spec/src/current/contracts/api/38-ordinary-local-stacks-transport.md spec/src/current/contracts/ui/04-local-stack-workspace-catalog.md spec/src/current/progress/M31/index.md web/src/lib/ui/store-forge/StoreForgeCreate.svelte web/src/lib/ui/store-forge/StoreForgeDetails.svelte web/src/lib/ui/store-forge/forge-inline.css web/src/lib/ui/store-stack-workspace/stack-workspace-details.css
```

**Notes**

All changed authored sources pass; generated evidence retains its canonical renderer.
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

<!-- evidence-field:versions markdown -->
## Versions

Node v26.9.0; pnpm 10.32.1; JJ 0.45.1; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11/Chromium; released Design Core 0.3.2, Mira 0.0.3 and Lucide Svelte 1.48.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

UI-only style/icon change. Forge actions, title/body drafts, expected-body/head guards, exact pending commands and receipts, credential handling, operation-pinned file/patch sources, hunk identities, storage, manifest and WAL are unchanged. The generic detachable-panel lifecycle and foundation window frame/geometry are unchanged.
<!-- /evidence-field -->

