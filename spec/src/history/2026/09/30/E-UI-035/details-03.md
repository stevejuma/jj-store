---
schema: 1
id: E-UI-035
---

# E-UI-035 details 3

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
pnpm exec prettier --write spec/src/milestones.md spec/src/verification.md
```

**Notes**

Evidence ledger and verification references formatted.
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
pnpm exec prettier --check web/src/lib/ui/store-application/StoreApplication.svelte web/src/lib/ui/store-application/StoreRepositoryContextActions.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte web/tests/browser/context-actions.spec.ts spec/src/ui.md spec/src/api.md spec/src/milestones.md spec/src/verification.md
```

**Notes**

All changed product, regression and canonical files pass after story formatting correction.
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
  unrelated Storybook/browser lanes; Rust checks/tests/builds; HTTP generation
  and real-RustFS qualification are out of scope under the targeted UI policy.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

jj 0.45.1; pnpm 10.32.1; Node 24.10.0; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11; Vite 8.3.1. Existing released dependency resolution unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Presentation only: loaded JJ Log revisions, Op Log operations and branch bookmarks reattach the existing panel below Details. Capability checks and existing mutation preview/confirmation flows are reused. Clearing/changing the selection hides or replaces the panel; keyed selection identity prevents stale panel dialogs. Generic metadata stays in the body. No new mutation, controller, HTTP, native object, manifest, WAL, receipt, credential or persistence format changes.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain focused selection, capability and navigation regressions; visual baselines remain separately approved.
<!-- /evidence-field -->

