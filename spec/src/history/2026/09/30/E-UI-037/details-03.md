---
schema: 1
id: E-UI-037
---

# E-UI-037 details 3

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

Canonical requirement/verification validators pass.
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

Protected UI and API changes have same-change canonical chapter updates.
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
pnpm spec:build
```

**Notes**

Canonical mdBook HTML builds.
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
pnpm check:progress
```

**Notes**

Same-change progress/evidence audit passes.
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
pnpm exec prettier --check web/src/lib/ui/store-application/StoreApplication.svelte web/src/lib/ui/store-application/StoreApplicationHeaderActions.svelte web/src/lib/ui/store-conflicts/StoreConflictEditor.svelte web/src/lib/ui/store-conflicts/StoreConflictPath.svelte web/src/lib/ui/store-conflicts/conflict-editor.ts web/src/lib/ui/store-conflicts/store-conflicts.css web/src/lib/ui/store-mutations/StoreRepositoryActions.svelte catalog/fixtures/StoreConflictEditorFixture.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte web/tests/browser/conflicts.spec.ts spec/src/api.md spec/src/ui.md spec/src/verification.md spec/src/milestones.md
```

**Notes**

All 14 changed implementation, regression and canonical files pass formatting.
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
pnpm exec prettier --write web/src/lib/ui/store-application/StoreApplication.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte
```

**Notes**

Final background-navigation inert guard and its restoration assertion formatted.
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
pnpm exec prettier --write spec/src/ui.md
```

**Notes**

Canonical background-navigation and snapshot guard prose formatted.
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

- Changed stories remain visual-pending; baseline approval is separate.
- Root aggregate pnpm spec:check, pnpm check, pnpm test and pnpm build;
  unrelated Storybook/browser suites; Rust checks/tests/builds, HTTP generation
  and real-RustFS qualification are out of scope under the targeted UI policy.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

jj 0.45.1; Node 24.10.0; pnpm 10.32.1; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11; Vite 8.3.1. Existing dependency resolution unchanged; public Design Core AppShell.SurfaceLayer and MergeEditor APIs used. No sibling package or dependency edits.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Presentation and consumer integration only. One shell-root conflict sheet covers main and right, preserves native descriptors in bound state, keeps per-path merge drafts, and omits extra metadata. Built-in accept-all updates the exact native term when center bytes match; custom edits clear a previous term/upload selection until explicitly staged. Stage captures the descriptor and pending upload prevents path switching/dismissal. The complete preview/apply/unknown/stale workflow and request identities remain unchanged. No native object, manifest, WAL, receipt, credential or HTTP format changes. Background repository navigation is inert while the sheet is open, and header mutations/refresh are disabled, so draft resolution stays attached to the captured selection.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain focused native-term, staged-edit, atomic recovery and shell lifecycle regressions; human visual baseline approval remains separate.
<!-- /evidence-field -->

