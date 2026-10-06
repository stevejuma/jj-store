---
schema: 1
id: E-UI-039
---

# E-UI-039 details 3

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

4 validators, 18 chapters and 107 canonical requirements pass.
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
pnpm spec:first
```

**Notes**

7 protected files and 3 canonical chapters pass same-change gate.
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
pnpm check:progress
```

**Notes**

79 tasks pass evidence bookkeeping audit.
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
pnpm spec:build
```

**Notes**

Canonical mdBook HTML builds.
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
pnpm exec prettier --check web/src/lib/ui/store-conflicts/StoreConflictEditor.svelte web/src/lib/ui/store-conflicts/StoreConflictPath.svelte web/src/lib/ui/store-conflicts/StoreConflictBody.svelte web/src/lib/ui/store-conflicts/StoreConflictHeaderActions.svelte web/src/lib/ui/store-conflicts/store-conflicts.css catalog/components/store-mutations/StoreConflictEditor.stories.svelte web/tests/browser/conflicts.spec.ts spec/src/api.md spec/src/ui.md spec/src/verification.md spec/src/milestones.md
```

**Notes**

All 11 changed files pass formatting.
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

<!-- evidence-field:deferred list -->
## Deferred

- Changed stories remain visual-pending; human visual baseline approval is
  separate.
- Root aggregate pnpm spec:check, pnpm check, pnpm test and pnpm build;
  unrelated UI suites; Rust/native contract generation and real-RustFS
  qualification are out of scope under targeted UI-only policy.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

jj 0.45.1; Node 24.10.0; pnpm 10.32.1; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11; Vite 8.3.1. Existing dependency resolution unchanged; no sibling or package manifest edits.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Consumer presentation only. Embedded bounded AppShell owns local body-sidebar registration so mobile tree navigation cannot teleport into the inert underlying application. Repository file tree and view-mode controls are reused unchanged. Compact header icons own exact active-path delete/stage, complete atomic Preview and prepared/recovery actions. Close is the sole dismissal button; per-path drafts stay bound above hidden editor instances through browser toggles and preview/back. Pending upload/mutation and prepared-result locks remain. No native identity, object, HTTP, manifest, WAL, upload, receipt or credential format changes.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain focused conflict, per-path draft, header action and mobile/body-layout acceptance; visual baseline approval remains separate.
<!-- /evidence-field -->

