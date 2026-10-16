---
schema: 1
id: E-FORGE-INLINE-001
---

# E-FORGE-INLINE-001 details 4

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
pnpm spec:first
```

**Notes**

Owning forge/UI chapters, API unchanged-contract confirmation and governance ownership declaration cover this slice.
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
pnpm exec spec-validator validate --only fileLimits,governance,summary
```

**Notes**

Canonical document limits, governance and navigation pass.
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
pnpm exec prettier --check catalog/components/store-forge/StoreForge.stories.svelte catalog/components/store-forge/StoreForgeFloating.stories.svelte catalog/components/store-forge/StoreForgeHeader.stories.svelte catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte catalog/components/store-forge/StoreForgeWorkspace.stories.svelte catalog/fixtures/ForgeWorkspaceFixture.svelte catalog/fixtures/forge/workspace-transport.ts docs/ui-provenance.json spec/src/current/contracts/api/38-ordinary-local-stacks-transport.md spec/src/current/contracts/architecture/24-forge-stacks.md spec/src/current/contracts/ui/04-local-stack-workspace-catalog.md spec/src/current/governance/spec-governance.md spec/src/current/progress/M31/index.md web/src/lib/controller/forge/controller.svelte.ts web/src/lib/controller/forge/effects.svelte.ts web/src/lib/controller/forge/inline.svelte.ts web/src/lib/controller/stack-workspace/details-columns.ts web/src/lib/controller/stack-workspace/diff-selection.ts web/src/lib/ui/store-forge/StoreForgeButton.svelte web/src/lib/ui/store-forge/StoreForgeCreate.svelte web/src/lib/ui/store-forge/StoreForgeDescription.svelte web/src/lib/ui/store-forge/StoreForgeDetails.svelte web/src/lib/ui/store-forge/StoreForgePanel.svelte web/src/lib/ui/store-forge/StoreForgePullRequest.svelte web/src/lib/ui/store-forge/StoreForgePullRequestActions.svelte web/src/lib/ui/store-forge/forge-inline.css web/src/lib/ui/store-forge/store-forge.css web/src/lib/ui/store-panels/StoreDetachablePanel.svelte web/src/lib/ui/store-panels/detachable-panel.css web/src/lib/ui/store-stack-workspace/StoreStackBookmarkActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackBookmarkCard.svelte web/src/lib/ui/store-stack-workspace/StoreStackBookmarkHeader.svelte web/src/lib/ui/store-stack-workspace/StoreStackLane.svelte web/src/lib/ui/store-stack-workspace/StoreStackLaneMenu.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceCanvas.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceDetailsColumn.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceDiff.svelte web/src/lib/ui/store-stack-workspace/stack-workspace-details.css web/tests/vite/forge-details.test.ts web/tests/vite/forge.test.ts
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

Node v26.9.0; pnpm 10.32.1; JJ 0.45.1; Git 2.55.0; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11 with Chromium; released Design Core 0.3.2 and Mira 0.0.3.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No public transport/schema, durable receipt, local JJ command, storage, manifest, WAL or credential format change. Existing operation-pinned file/patch sources, expected-body/head guards, source hunk identities and server admission remain authoritative. Per-branch plans/drafts, inline selection, disclosure and floating geometry are session-only UI state. Unknown writes retain their original pinned request; explicit checkPending observes recovery before clearing it. Floating windows never own publication intent.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Configured forges offer Create PR only before Push on the topmost unsubmitted bookmark. It opens a branch-default title and maintained Markdown chat composer inline above commits; dependencies and Draft/Ready use the existing explicit pinned submission. PR badges and selecting a changed file open a single per-origin details column using the Op Log composition: a separately expandable PR card first and the selected file diff below. Its header owns title, linked number, draft/state and pencil/float/checks/close controls; a compact header menu owns Draft, Restack, provider-supported merge methods, merge status observation and explicitly confirmed guarded merge. The body omits repeated title/number/state and repository metadata; authored navigation links remain intact. PR details are removed from stack bodies. PR-only opening needs no selected file. Collapsing PR content preserves the diff; closing the file preserves the PR. Detailed checks remain behind explicit Inspect checks. The existing no-forge, Unstaged and target-history diff layout remains intact. A generic application-owned adapter over released floating windows reparents the same content/editor between inline and floating hosts and disposes its controller/listeners. Existing bookmark header and lane menu are extracted without changing action semantics to meet production module bounds. Tests are limited to changed components, the forge controller and directly affected Stack/diff scenarios; root aggregates, full Storybook, Rust lanes and wider suite repairs are out of scope under user direction and targeted policy. Cold browser processes qualify lifecycle; visual baselines, live providers, packaged and native/platform gates remain separate and open.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the contextual PR+diff story at http://127.0.0.1:6046/?path=/story/jj-store-forge-workspace--pull-request-details-beside-stacks. M31 remains In progress pending credentialed disposable-forge, packaged, native/platform and visual qualification; wider test-suite work stays deferred.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: "JST-UI-020 and JST-FORGE-001/004/007 now specify inline topmost
  creation, branch-default titles, the Op Log-style shared PR+diff column and
  reusable floating panels. JST-GOV-008 ownership/provenance is updated.
  JST-API-001 is explicitly unchanged: same plans, authorized actions,
  expected-body/head/operation guards and exact pending receipts; presentation
  cannot grant hunk or publication authority."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M31 links this focused evidence and stays In progress. Passing scoped
  mock/browser/controller checks do not promote live-provider, native/platform,
  packaged or visual gates. Wider test-suite fixes remain deferred as requested.

```
<!-- /evidence-field -->

