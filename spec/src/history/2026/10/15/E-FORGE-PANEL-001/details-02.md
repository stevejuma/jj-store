---
schema: 1
id: E-FORGE-PANEL-001
---

# E-FORGE-PANEL-001 details 2

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
pnpm exec spec-validator validate --only fileLimits,governance,summary
```

**Notes**

Final evidence split created details-02.md without a navigation entry. Regenerated navigation for this evidence directory only; no shared generator changes.
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
pnpm check:progress
```

**Notes**

Initial evidence used story paths as test-registry references. These are not registered test IDs; removed the invalid references while retaining exact executed Storybook commands/results below. No product or shared checker changes.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeWorkspace.stories.svelte catalog/components/store-forge/StoreForge.stories.svelte -t 'Pull request details beside stacks|Markdown description and merge method menu'
```

**Notes**

Initial name filter skipped all five stories because the runner uses exported identifiers. This is not acceptance evidence; immediately reran both complete affected story files.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeWorkspace.stories.svelte catalog/components/store-forge/StoreForge.stories.svelte
```

**Notes**

Before implementation: 2 failed, 3 passed. PullRequestDetailsBesideStacks rejected the custom header without the column-header primitive. MarkdownDescriptionAndMergeMethodMenu could not find the rendered Review summary heading in the raw-text-only description. No accessibility failures reported in the passing stories.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Initial implementation had two local type errors: optional controller.data in a snippet and unsupported exact query option. Corrected both in the changed PR component/story; zero warnings. No wider suite repairs.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeWorkspace.stories.svelte catalog/components/store-forge/StoreForge.stories.svelte
```

**Notes**

First implementation rerun: all five affected interaction/accessibility stories passed from a fresh Vitest/browser process.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeWorkspace.stories.svelte catalog/components/store-forge/StoreForge.stories.svelte
```

**Notes**

Final cold rerun: six interaction/accessibility stories passed, zero failures. Added keyboard End/Enter selection and Escape focus return, provider-limited methods, and preview/re-edit after failed save. Context story proves equal header height/background, icon-only close and rendered managed links; submission story edits and previews Markdown through the shared composer.
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
pnpm catalog:check
```

**Notes**

Zero Svelte errors/warnings; catalog boundaries pass (447 modules).
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
pnpm web:check
```

**Notes**

Zero Svelte errors/warnings.
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
node scripts/web-architecture.mjs
```

**Notes**

Owned production and affected catalog modules satisfy maintained size boundaries (1936 modules).
<!-- /run -->

<!-- run -->
### Run 11

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

Production imports only maintained local Community components and released Design Core/Mira. New application composition has its own provenance declaration.
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
pnpm check:progress
```

**Notes**

Fresh same-change evidence and explicit M31 review pass; M31 remains In progress.
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
pnpm spec:first
```

**Notes**

Owning architecture/UI updates, API unchanged-contract confirmation and governance ownership record cover the changed files.
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
pnpm exec spec-validator validate --only fileLimits,governance,summary
```

**Notes**

Authored page limits and governed evidence/navigation pass.
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
pnpm exec prettier --check catalog/components/store-forge/StoreForge.stories.svelte catalog/components/store-forge/StoreForgeWorkspace.stories.svelte catalog/fixtures/ForgeFixture.svelte docs/ui-provenance.json spec/src/current/contracts/api/38-ordinary-local-stacks-transport.md spec/src/current/contracts/architecture/24-forge-stacks.md spec/src/current/contracts/ui/04-local-stack-workspace-catalog.md spec/src/current/governance/spec-governance.md spec/src/current/progress/M31/index.md web/src/lib/ui/store-forge/StoreForgeDescription.svelte web/src/lib/ui/store-forge/StoreForgePanel.svelte web/src/lib/ui/store-forge/StoreForgePullRequest.svelte web/src/lib/ui/store-forge/store-forge.css
```

**Notes**

All changed authored sources are formatted. Exact path list was derived from jj diff, excluding generated evidence/navigation which retain generator format.
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

No public action/schema, persistence, credential or request-identity change. Same typed forge actions, expected body/head guards and capability input. PR description source remains Markdown; rendering uses the maintained safe HTML policy. Fixtures have no credentials or remote effects.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

The PR panel now composes the same ColumnCanvas header/title/action primitives as stack columns. Its close action is icon-only with accessible name and tooltip. The application-owned StoreForgeDescription composes the maintained CommunityChatComposer and CommunityMarkdownContent, showing Markdown initially with retained edit/preview, preserving failed-save drafts and rendering managed stack links. Submission descriptions use the same composer/preview. The merge-method dropdown composes released shadcn radio items, limits methods to provider capabilities and falls back when squash is unsupported. Only the two affected Forge story files and web/catalog types were run. Root aggregates, full Storybook, Rust, live-provider and wider suite repairs remain out of scope under the user instruction. Visual-pending; no baseline comparison/update or gate promotion.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

User review of the updated contextual workspace story at http://127.0.0.1:6046/?path=/story/jj-store-forge-workspace--pull-request-details-beside-stacks. M31 credentialed disposable-forge, native/platform, packaged application and visual approval gates remain open.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: "JST-FORGE-004/005/007 and JST-UI-020 now specify matching column
  headers, an accessible icon-only close, shared Markdown/composer descriptions
  and the provider-gated shadcn merge menu. JST-API-001 transport/action guards
  and JST-GOV-008 ownership are explicitly unchanged: this composes existing
  local components and released foundations without copied external source or
  new authority."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M31 remains In progress and links this focused presentation evidence.
  M21 and live provider, packaged, native/platform and visual qualification
  gates are unchanged. Passing scoped stories/types do not qualify those gates
  or the wider suite.

```
<!-- /evidence-field -->

