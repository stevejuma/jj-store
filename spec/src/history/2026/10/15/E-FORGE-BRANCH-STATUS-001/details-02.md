---
schema: 1
id: E-FORGE-BRANCH-STATUS-001
---

# E-FORGE-BRANCH-STATUS-001 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeBranchStatus.stories.svelte
```

**Notes**

Before production implementation: 1 failed. The new under-title Branch status group was absent. Log .local/forge-branch-status-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeBranchStatus.stories.svelte
```

**Notes**

Exact after-run: 1 passed before adding three further status scenarios. Proves under-title placement, a smaller PR control, running spinner and PR activation without changing branch selection. Log .local/forge-branch-status-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/forge-branch-status.test.ts web/tests/vite/forge-details.test.ts
```

**Notes**

17 tests passed: 14 new presentation-model cases and 3 existing direct forge-detail cases. Covers exact SHA-1 head comparison, incomparable/missing/stale heads, failed/running/queued precedence, skipped/unknown/attention states, incomplete snapshots and outer/per-check revision mismatches. Log .local/forge-branch-status-unit.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeBranchStatus.stories.svelte catalog/components/store-forge/StoreForgeWorkspace.stories.svelte catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte catalog/components/store-forge/StoreForgeHeader.stories.svelte catalog/components/store-forge/StoreForgeFloating.stories.svelte
```

**Notes**

19 directly affected browser stories passed in a fresh test process. Includes all four new status scenarios, opaque neutral light/dark button hover contrast, lower title-row PR placement, inline creation/submission, details and file placement, float/redock/drafts, guarded actions and failure toasts. Log .local/forge-branch-status-consumers.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte -t 'BranchChipsShare|BranchAndCommitChips|BranchActionsDiffer|KebabMenusHave'
```

**Notes**

5 directly affected no-forge stack/header/action stories passed; 5 unrelated stories intentionally skipped. Log .local/forge-branch-status-no-forge.log.
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
pnpm catalog:check
```

**Notes**

Initial and final checks: zero Svelte errors or warnings; catalog boundaries pass for 451 modules. Logs .local/forge-branch-status-types.log and .local/forge-branch-status-types-final.log.
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
node scripts/web-architecture.mjs
```

**Notes**

Production module size and import boundaries pass for 1959 modules. Log .local/forge-branch-status-architecture.log.
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
pnpm check:ui-ownership
```

**Notes**

Owned UI and released dependency boundaries pass. Existing Community source/presentation was inspected before released Design Core Button xs and Spinner; no copied source, package, alias or foundation patch was introduced.
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
pnpm evidence:new -- --id E-FORGE-BRANCH-STATUS-001 --task M31 --requirement JST-UI-020 --requirement JST-FORGE-005 --requirement JST-API-001 --requirement JST-GOV-008
```

**Notes**

Created fresh evidence/day links and restored 446 unrelated generator-only navigation files from the parent. Existing historical evidence remains unchanged; M31 was reformatted.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
python3 urllib.request.urlopen(http://127.0.0.1:6046/index.json, timeout=5)
```

**Notes**

Warm review Storybook stopped responding during HMR. Verified the task-owned listener PID 84945, command, workspace cwd and port before TERM; after it remained unresponsive, KILL stopped only this verified listener. No shared startup/test infrastructure was changed.
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
python3 Storybook index health assertion
```

**Notes**

After restart the index returned HTTP 200. The ad hoc health assertion counted the generated docs entry as a fifth story; corrected the selector to require type=story. All four authored stories were present. This was a health-script assertion error, not an application or interaction failure.
<!-- /run -->

<!-- run -->
### Run 12

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

Restarted the task-owned review preview after source authoring settled. Final index and existing contextual iframe return HTTP 200; the index contains all four new Branch status stories.
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
pnpm check:progress
```

**Notes**

Fresh record and explicit unchanged M31 review pass.
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
pnpm spec:first
```

**Notes**

Owning UI/forge requirements, provenance and explicit unchanged API confirmation cover this presentation change.
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
pnpm exec spec-validator validate --only fileLimits,governance,summary
```

**Notes**

Document limits, change governance and navigation pass.
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
pnpm exec prettier --check catalog/components/store-forge/StoreForgeBranchStatus.stories.svelte catalog/components/store-forge/StoreForgeWorkspace.stories.svelte catalog/fixtures/ForgeWorkspaceFixture.svelte catalog/fixtures/forge/workspace-transport.ts docs/ui-provenance.json spec/src/current/contracts/api/38-ordinary-local-stacks-transport.md spec/src/current/contracts/architecture/24-forge-stacks.md spec/src/current/contracts/ui/04-local-stack-workspace-catalog.md spec/src/current/governance/spec-governance.md spec/src/current/progress/M31/index.md web/src/lib/controller/forge/branch-status.ts web/src/lib/ui/store-forge/StoreForgeBranchStatus.svelte web/src/lib/ui/store-forge/StoreForgeButton.svelte web/src/lib/ui/store-stack-workspace/StoreStackBookmarkCard.svelte web/src/lib/ui/store-stack-workspace/StoreStackBookmarkHeader.svelte web/tests/vite/forge-branch-status.test.ts
```

**Notes**

All changed authored files pass; generated evidence retains its canonical renderer.
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

Node v26.9.0; pnpm 10.32.1; JJ 0.45.1; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11/Chromium; released Design Core 0.3.2 and Mira 0.0.3.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

UI-only presentation model, owned component and catalog fixtures. No API/schema, forge mutation, credential, exact pending intent/receipt, draft, operation-pinned diff, storage, manifest or WAL format change. Head equality is display-only and never grants push or merge authority.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

The topmost stack change places its compact neutral bordered PR control below the title, with observed push status and a textual checks badge. Running/queued checks show a spinner; Passed/Failed use distinct icons and tones. Missing, stale, incomplete or incomparable observations remain explicit; no checks or skipped checks never imply success. Push labels compare the exact local SHA-1 head with the observed open-PR head, without asserting ancestry or ahead counts. Lower changes retain title-row PR controls and existing CI metadata. Existing Button xs and Spinner exports plus the maintained chip/check icons are reused. Catalog-only options supply synced, changed, failed, running and incomplete observations; simulated submission updates the observed head. The contextual story URL remains unchanged. Verification is restricted to this UI, its pure model, type/ownership checks, 19 direct forge stories and 5 no-forge consumers. Root aggregates, Rust, full catalog, unrelated test repairs, screenshot comparison and baseline updates are out of scope under user direction and the targeted verification policy. Stories remain visual-pending.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review http://127.0.0.1:6046/?path=/story/jj-store-forge-workspace--pull-request-details-beside-stacks. M31 live-provider, native/platform, packaged and visual approval gates remain open; wider test-suite work is deferred.
<!-- /evidence-field -->

