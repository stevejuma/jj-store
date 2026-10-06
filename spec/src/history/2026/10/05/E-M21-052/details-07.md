---
schema: 1
id: E-M21-052
---

# E-M21-052 details 7

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
Historical ledger object/raw prefix comparison against jj @-
```

**Notes**

All532 existing evidence objects and the full preexisting raw JSON prefix remain identical.
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
pnpm check:progress (ledger closure initial)
```

**Notes**

delivery-progress.log:Storybook evidence used generated CamelCase export rather than the literal source story name; corrected only the evidence pointer.
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
pnpm exec spec-validator validate --only governance,verification,summary (ledger closure)
```

**Notes**

delivery-governance.log:3 validators,23 chapters,173 requirements.
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
pnpm spec:first (ledger closure)
```

**Notes**

delivery-spec-first.log:48 protected files and7 canonical chapters.
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
pnpm exec prettier --check <all jj diff --name-only paths> (ledger closure)
```

**Notes**

delivery-format.log:all55 changed files pass.
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
pnpm exec prettier --write spec/src/milestones.md (ledger closure)
```

**Notes**

ledger-close-format.log:tables formatted; historical JSON prefix protected by existing prettier-ignore.
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
pnpm check:progress (story pointer second)
```

**Notes**

delivery-progress-after.log:checker accepts Rust/Python/JS test definitions rather than Svelte Story names. Kept the named Storybook scenario and exact commands in runs/files; removed the unsupported tests-field pointer, without modifying governance code.
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
pnpm check:progress (final named evidence)
```

**Notes**

delivery-progress-final.log:142 tasks pass with exact supported test references; Storybook scenarios remain evidenced in actual runs/files.
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

<!-- evidence-field:notes markdown -->
## Notes

Existing Vite 7476, backend 7478 and Storybook 6057 preserved. No live user-repository mutation, fixture reset, installed executable replacement, package publication or visual baseline changes. Root aggregate and unrelated Rust lanes out of scope; committed native mutation evidence remains in E-M21-045 through E-M21-050. Historical ledger objects and raw JSON prefix preserved.
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

uqxkytrtnlpuqxyvnsssuuxrsrtrskut
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M21
- M21.2
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceLazy.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceProduction.stories.svelte
- catalog/fixtures/StackWorkspaceLazyApplicationFixture.svelte
- catalog/fixtures/StackWorkspaceProductionFixture.svelte
- catalog/fixtures/stack-workspace/production-http-data.ts
- catalog/fixtures/stack-workspace/production-http.ts
- docs/implementation-plan.md
- docs/maintenance/coverage.json
- docs/site/guides/browsing.md
- docs/site/guides/editing.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- docs/site/reference/transport.md
- docs/site/technical/local.md
- scripts/stacks-browser-acceptance.mjs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/documentation.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/stack-workspace/controller-files.ts
- web/src/lib/controller/stack-workspace/controller-snapshot.ts
- web/src/lib/controller/stack-workspace/controller.svelte.ts
- web/src/lib/controller/stack-workspace/diff-selection.ts
- web/src/lib/controller/stack-workspace/files.ts
- web/src/lib/controller/stack-workspace/issued-history.ts
- web/src/lib/controller/stack-workspace/lazy-details.svelte.ts
- web/src/lib/controller/stack-workspace/loaded-ranges.ts
- web/src/lib/controller/stack-workspace/production-adapter.ts
- web/src/lib/controller/stack-workspace/retained-diffs.ts
- web/src/lib/controller/stack-workspace/source-range.ts
- web/src/lib/controller/stack-workspace/stack-history.svelte.ts
- web/src/lib/controller/stack-workspace/stack-segments.ts
- web/src/lib/controller/stack-workspace/types.ts
- web/src/lib/notifications/notification-recovery.svelte.ts
- web/src/lib/ui/store-application/StoreRepositoryNotices.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackBookmarkCard.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackCommitActions.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackLane.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceCanvas.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceDiff.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceForm.svelte
- web/tests/real/stacks-browser.spec.ts
- web/tests/vite/production-stack-adapter.test.ts
- web/tests/vite/production-stacks-application.test.ts
- web/tests/vite/production-stacks-lifecycle.test.ts
- web/tests/vite/stack-workspace-lazy-controller.test.ts
- web/tests/vite/stack-workspace-segments.test.ts
- web/tests/vite/stack-workspace.test.ts
<!-- /evidence-field -->

<!-- evidence-field:summary markdown -->
## Summary

Approved ordinary-local mutation integration is accepted:107 direct units,36 Stacks interaction/accessibility stories and3 complete cold built real-folder browser tests pass. Graph preparation recovery, exact ancestor destinations, dependent per-segment net files, same-scope file selections and retained adjacent patch lifetimes are qualified. Native JJ/jj-lib0.45.1 keeps normal WIP above a retained megamerge; rewrites preserve native change identities and config remains local TOML outside repository history. Committed native evidence is E-M21-045 through E-M21-050. Unsupported partial/binary/conflict shapes, historical retained/native-hosted execution, package publication and visual baselines remain separate; no root aggregate or unrelated Rust lanes run.
<!-- /evidence-field -->

