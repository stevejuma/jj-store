---
schema: 1
id: E-FORGE-EDIT-001
---

# E-FORGE-EDIT-001 details 2

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
pnpm check:progress
```

**Notes**

The first evidence check rejected a pass record while its dev-server refresh remained listed as a gap. Completed the refresh and cleared that gap before the final checks; no checker or acceptance rule was changed.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeWorkspace.stories.svelte catalog/components/store-forge/StoreForge.stories.svelte -t 'PullRequestDetailsBesideStacks|MarkdownDescriptionAndMergeMethodMenu'
```

**Notes**

Before implementation: 2 failed, 12 intentionally skipped. Both contextual header lookup and standalone header containment detect the existing body-only Edit details control. Log .local/forge-edit-header-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeWorkspace.stories.svelte catalog/components/store-forge/StoreForge.stories.svelte -t 'PullRequestDetailsBesideStacks|MarkdownDescriptionAndMergeMethodMenu|EditingUnavailable'
```

**Notes**

Initial implementation: 3 passed, 12 intentionally skipped. Accessible icon-only header action, keyboard edit/preview pressed state, removed label and unavailable-edit guard pass. Log .local/forge-edit-header-after.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

The header tooltip child snippet cannot retain an outer controller.data narrowing. Added an optional capability read that remains disabled when data is absent; no assertion or dependency workaround. Log .local/forge-edit-header-types.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeWorkspace.stories.svelte catalog/components/store-forge/StoreForge.stories.svelte
```

**Notes**

All 15 directly affected Forge/Workspace stories pass in fresh Vitest/Chromium processes with interaction/accessibility coverage. Includes save, failed-save toast and retained draft, safe Markdown, composer preview, merge menu, automatic checks, unsubmitted creation, top-card/menu submission and light/dark title/button surfaces. Log .local/forge-edit-header-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeWorkspace.stories.svelte catalog/components/store-forge/StoreForge.stories.svelte -t 'PullRequestDetailsBesideStacks|MarkdownDescriptionAndMergeMethodMenu|EditingUnavailable'
```

**Notes**

Final narrow rerun after the null-safe capability read: 3 passed, 12 intentionally skipped. Header geometry, icon-only accessible name, Enter to edit title/composer, preview pressed-state reset and capability-disabled pencil pass. Log .local/forge-edit-header-final-target.log.
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

Final zero Svelte errors/warnings; catalog boundaries pass (447 modules). Log .local/forge-edit-header-types-final.log.
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

Maintained module boundaries pass (1941 modules).
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

Owned production/released dependency boundary passes; no new production file or attribution changes.
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
pnpm evidence:new -- --id E-FORGE-EDIT-001 --task M31 --requirement JST-UI-020 --requirement JST-FORGE-004 --requirement JST-API-001
```

**Notes**

Started fresh same-change evidence. Retained its directory/day links and restored 446 unrelated generated navigation edits. Reformatted the owning milestone page after generation.
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
curl --max-time 3 -sS -o /dev/null -w 'Storybook: %{http_code}\n' http://127.0.0.1:6046/index.json
```

**Notes**

The warm development server timed out with no response. Verified listener PID 29151 belonged to this workspace and its exact Storybook command, then stopped only that process before refresh. This does not invalidate the fresh passing browser tests or qualify visual acceptance.
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

Restarted the task-owned development server after authoring settled. Final index.json includes the contextual workspace story and both index and iframe return HTTP 200. No shared Storybook/test infrastructure changes.
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

Fresh record and explicit M31 review pass.
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

Owning UI/forge chapters and API unchanged-contract confirmation cover the three production files and direct catalog consumers.
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

Evidence, page limits and navigation pass.
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
pnpm exec prettier --check catalog/components/store-forge/StoreForge.stories.svelte catalog/components/store-forge/StoreForgeWorkspace.stories.svelte catalog/fixtures/ForgeFixture.svelte spec/src/current/contracts/api/38-ordinary-local-stacks-transport.md spec/src/current/contracts/architecture/24-forge-stacks.md spec/src/current/contracts/ui/04-local-stack-workspace-catalog.md spec/src/current/progress/M31/index.md web/src/lib/ui/store-forge/StoreForgeDescription.svelte web/src/lib/ui/store-forge/StoreForgePanel.svelte web/src/lib/ui/store-forge/StoreForgePullRequest.svelte
```

**Notes**

Changed authored sources pass; generated evidence uses its canonical renderer.
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

Node v26.9.0; pnpm 10.32.1; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11 with Chromium; released Design Core 0.3.2 and Mira 0.0.3.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing forge request/response schemas, expected-body protection, session credentials, drafts and exact pending effect identities are unchanged. Editing state is presentation-only and shared through a Svelte binding; no controller, persistence, storage or WAL change.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

PR editing moves from the body to an icon-only pencil in the shared column header, retaining an accessible Edit details label, tooltip, pressed state and capability/busy/pending guards. The selected-PR component shares the editing state with its parent and still clears it when selection changes. The description omits the redundant visible Description title; rendered PRs have no inline edit control. Editing keeps Preview description and save/draft behavior; submission descriptions retain their existing edit/preview controls and accessible composer labels. The complete direct acceptance surface is the two Forge story files (15 stories), the sole standalone fixture gaining a read-only edit scenario and the three production components. Existing maintained Community composer/Markdown and released Design Core header/action/tooltip primitives were inspected; no copied source or dependency patches. Root aggregates, unrelated Stack stories, controller/Rust tests, full Storybook and wider suite changes are out of scope under user direction and targeted policy. Startup/lifecycle/generated assets are unchanged; dev-server refresh only restores the review environment. Stories remain visual-pending, with no baseline comparison/update or live-provider/packaged qualification.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the contextual workspace story at http://127.0.0.1:6046/?path=/story/jj-store-forge-workspace--pull-request-details-beside-stacks. M31 live forge, native/platform, packaged and visual gates remain open.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: "JST-UI-020 and JST-FORGE-004 specify header-owned icon editing and
  description presentation without a redundant label. JST-API-001 is unchanged:
  same title/body drafts, save and expected-body guards, capability admission
  and exact pending commands."

```
<!-- /evidence-field -->

