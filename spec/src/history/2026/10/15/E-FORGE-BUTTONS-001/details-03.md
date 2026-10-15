---
schema: 1
id: E-FORGE-BUTTONS-001
---

# E-FORGE-BUTTONS-001 details 3

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
pnpm exec spec-validator validate --only fileLimits,governance,summary
```

**Notes**

Authored budgets, evidence fields and split-page navigation pass; navigation regenerated only for this new evidence directory.
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
pnpm exec prettier --check catalog/components/store-forge/StoreForgeWorkspace.stories.svelte spec/src/current/contracts/api/38-ordinary-local-stacks-transport.md spec/src/current/contracts/architecture/24-forge-stacks.md spec/src/current/contracts/ui/04-local-stack-workspace-catalog.md spec/src/current/progress/M31/index.md web/src/lib/ui/store-forge/StoreForgeButton.svelte web/src/lib/ui/store-stack-workspace/StoreStackBookmarkActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackBookmarkCard.svelte web/src/lib/ui/store-stack-workspace/StoreStackLane.svelte web/src/lib/ui/store-stack-workspace/stack-workspace.css
```

**Notes**

Changed authored sources pass. Generated evidence/navigation retain their canonical renderer.
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

Node v26.9.0; pnpm 10.32.1; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11 with Chromium; released Design Core 0.3.2 and Mira 0.0.3. No Rust builds/tests.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing version-1 Forge actions, expected body/head guards, exact pending request identities and credential boundaries are unchanged. No public API, schema generation, persistence, TOML, storage or WAL format change.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

User-directed application presentation only: forge PR buttons align at the trailing end of bookmark titles while CI status stays in metadata. Opaque theme-based background, border and hover/focus surfaces stay distinct on hovered rows. Submit stack moves beside Push on the topmost displayed bookmark card, appears also in the column kebab menu, and is omitted on lower cards and as a separate header button. Both entries share existing conflict/locked/busy/pending guards and preview behavior. The generic branch-action component receives an optional afterPush snippet; it does not depend on the forge controller. Existing released Design Core and maintained Community primitives were inspected in this session; no upstream defect is patched. Small local derived head/history/immutability facts remove repeated reads while retaining the module boundary. The complete affected surface is the changed Forge workspace stories plus the five selected direct Stack refinements. Root aggregates, full Storybook, Rust, unrelated controller tests, wider test-suite repairs and live-provider/packaged qualification are out of scope under the user instruction and targeted policy. UI startup, lifecycle and generated assets are unchanged, so a cold app restart is not required. Stories stay visual-pending; no screenshot comparison or visual-baseline updates. A manual inspection attempt through the existing CUA tab handle was blocked by the browser URL policy; no alternate surface or policy workaround was attempted. Focused browser tests provide functional evidence, not visual approval.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the contextual Storybook view at http://127.0.0.1:6046/?path=/story/jj-store-forge-workspace--pull-request-details-beside-stacks. M31 credentialed live-provider, native/platform, packaged application and visual approval gates remain open.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: "JST-UI-020 and JST-FORGE-001/004/007 record title-row PR controls,
  theme-distinct surfaces and topmost action-row plus column-menu submission.
  JST-API-001 unchanged: entry-point placement shares the same preview/actions,
  selection guards and exact pending intent; clicking PR controls cannot select
  the containing row. Existing Community ownership and released dependencies
  remain unchanged."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M31 remains In progress and links this focused evidence. No live
  provider, native/platform, packaged or visual gate is promoted; broader test
  work remains deferred.

```
<!-- /evidence-field -->

