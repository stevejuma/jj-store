---
schema: 1
id: E-M21-004
---

# E-M21-004 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check catalog/components/store-stack-workspace/StoreStackWorkspaceLazy.stories.svelte catalog/fixtures/StackWorkspaceLazyApplicationFixture.svelte docs/maintenance/coverage.json docs/site/technical/architecture.md spec/src/api.md spec/src/documentation.md spec/src/ui.md web/src/lib/controller/stack-workspace/controller.svelte.ts web/src/lib/controller/stack-workspace/diff-selection.ts web/src/lib/controller/stack-workspace/eligibility.ts web/src/lib/controller/stack-workspace/file-drag.ts web/src/lib/controller/stack-workspace/lazy-details.svelte.ts web/src/lib/controller/stack-workspace/retained-diffs.ts web/src/lib/controller/stack-workspace/target-history.svelte.ts web/src/lib/controller/stack-workspace/types.ts web/src/lib/ui/store-stack-workspace/StoreStackBookmarkActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackBookmarkCard.svelte web/src/lib/ui/store-stack-workspace/StoreStackCommitActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackCommitComposer.svelte web/src/lib/ui/store-stack-workspace/StoreStackDiffHunk.svelte web/src/lib/ui/store-stack-workspace/StoreStackLane.svelte web/src/lib/ui/store-stack-workspace/StoreStackRelativeActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackSourceFiles.svelte web/src/lib/ui/store-stack-workspace/StoreStackTargetHistory.svelte web/src/lib/ui/store-stack-workspace/StoreStackUnstaged.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceConflict.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceDiff.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePicker.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceSpacer.svelte web/tests/vite/stack-workspace-lazy-controller.test.ts
```

**Notes**

Initial source-file second-pass formatting issue; lazy-controller-format-check.log. Later concurrent review changed coverage.json during formatting check: one formatting failure; lazy-controller-format-verified.log. Final check is sequential after review completion.
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
pnpm exec prettier --check catalog/components/store-stack-workspace/StoreStackWorkspaceLazy.stories.svelte catalog/fixtures/StackWorkspaceLazyApplicationFixture.svelte docs/maintenance/coverage.json docs/site/technical/architecture.md spec/src/api.md spec/src/documentation.md spec/src/ui.md web/src/lib/controller/stack-workspace/controller.svelte.ts web/src/lib/controller/stack-workspace/diff-selection.ts web/src/lib/controller/stack-workspace/eligibility.ts web/src/lib/controller/stack-workspace/file-drag.ts web/src/lib/controller/stack-workspace/lazy-details.svelte.ts web/src/lib/controller/stack-workspace/retained-diffs.ts web/src/lib/controller/stack-workspace/target-history.svelte.ts web/src/lib/controller/stack-workspace/types.ts web/src/lib/ui/store-stack-workspace/StoreStackBookmarkActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackBookmarkCard.svelte web/src/lib/ui/store-stack-workspace/StoreStackCommitActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackCommitComposer.svelte web/src/lib/ui/store-stack-workspace/StoreStackDiffHunk.svelte web/src/lib/ui/store-stack-workspace/StoreStackLane.svelte web/src/lib/ui/store-stack-workspace/StoreStackRelativeActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackSourceFiles.svelte web/src/lib/ui/store-stack-workspace/StoreStackTargetHistory.svelte web/src/lib/ui/store-stack-workspace/StoreStackUnstaged.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceConflict.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceDiff.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePicker.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceSpacer.svelte web/tests/vite/stack-workspace-lazy-controller.test.ts
```

**Notes**

Final changed-file formatting passes; lazy-controller-format-complete.log. Exact argument list is the current jj diff file list excluding the ledger.
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
pnpm spec:first
```

**Notes**

29 protected files/three canonical chapters pass; lazy-controller-spec-first.log. Final including ledger also passes in lazy-controller-spec-first-final.log.
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
pnpm spec:validate
```

**Notes**

Five validators, 23 chapters, 173 requirements; lazy-controller-spec-validate.log and lazy-controller-spec-validate-final.log.
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
pnpm check:progress
```

**Notes**

Final ledger/check scope bookkeeping passes; 485 historical JSON evidence objects preserved byte-identically; lazy-controller-progress.log.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- "M21.2 remains incomplete: production adapter/App injection,
  absent-target/combination setup, bounded stack discovery and real graph
  segments, disk scanning, durable executor/checkout, named combinations, exact
  file/hunk and graph commands, linked-native and complete cold application
  lifecycle qualification remain."
- Live CUA selection of the existing 6057 story timed out in
  Emulation.setFocusEmulationEnabled; no new live visual review is claimed.
  Fresh headless Storybook interaction/accessibility evidence remains distinct
  from a real-provider lifecycle run.
- Inherited Clippy diagnostics remain recorded separately; this UI-only slice
  does not rerun Rust.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Represent an absent workspace target and named combination honestly, then build the repository/binding-scoped production read adapter and inject it through App lifecycle. Qualify real-folder browsing before enabling any durable write.
<!-- /evidence-field -->

<!-- evidence-field:out_of_scope markdown -->
## Out of scope

Root aggregate suites, Rust builds/tests, package publication, installed-service changes, screenshot comparisons and visual baseline updates.
<!-- /evidence-field -->

