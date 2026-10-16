---
schema: 1
id: E-MAIN-LOCAL-QUALIFICATION-001
---

# E-MAIN-LOCAL-QUALIFICATION-001 details 5

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
pnpm spec:check
```

**Notes**

Stopped at cold catalog: bundled DM Sans font replaced by utility defaults. The original docs check first reported stale generated installation/types references. Broader aggregate remains pending.
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
JJ_STORE_DOC_REFERENCE=$PWD/target/debug/examples/docs_reference pnpm docs:generate
```

**Notes**

Used the just-built main-workspace reference executable; updated generated installation/types documentation.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentToolGroups.stories.svelte catalog/components/store-application/AgentChatBody.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte
```

**Notes**

45 passed after explicit recorded-agent view readiness, fixture-only toolbar padding and settled commit-preview visibility. Forge workspace separately passed all six after menu-visibility waits.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentLayout.stories.svelte catalog/components/store-application/WorkspaceFilePresentation.stories.svelte catalog/components/store-appearance/AppearanceOverlay.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte
```

**Notes**

27 passed, three workspace-file failures remained; AgentLayout, AppearanceOverlay and ProjectsHub fully passed. Keep full Axe rules; added names to comparison range/base listboxes.
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
pnpm build-storybook
```

**Notes**

Fresh catalog after stylesheet order matched application: utility rules before released prepared Design Core CSS. Dependency output already contains correct DM Sans tokens; no upstream override or dependency edit was needed.
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
node --test scripts/catalog-browser.test.mjs
```

**Notes**

Exact cold_catalog_contract passed after fresh catalog build, including font, layout, focus and no external/API/WebSocket request checks.
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
pnpm test:storybook
```

**Notes**

Initial broad diagnostic run was deliberately interrupted (exit 130) when shared catalog CSS ordering changed. Its partial failures are retained in storybook.log; it never establishes full coverage. Fresh complete run with unchanged assertions and a diagnostic reporter is in progress.
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
pnpm web:check
```

**Notes**

web-check-03: zero errors/warnings before later inbox resize batching.
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
pnpm catalog:check
```

**Notes**

catalog-check-03: zero errors/warnings; 461 modules passed before later readiness edits.
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
JJ_STORE_DOC_REFERENCE=$PWD/target/debug/examples/docs_reference pnpm docs:check
```

**Notes**

docs-check-03: strict documentation build, nine local Mermaid diagrams and closure of 143 files/26,546,119 bytes passed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/WorkspaceFilePresentation.stories.svelte catalog/components/store-application/AgentSessions.stories.svelte
```

**Notes**

observer-diagnostic: 17 passed, exit 0. An unsuppressed ResizeObserver runtime error was also logged. Diagnostic instrumentation identified callbacks; fresh proof without instrumentation is required. An app-owned frame-scheduling experiment did not eliminate the runtime warning and was reverted.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 14
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceConflictVersions.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceSettings.stories.svelte catalog/components/store-application/StoreEvolutionColumn.stories.svelte
```

**Notes**

ui-regressions-03: 11 passed/14 failed. Thirteen missing lazy-view readiness results, stale geometry during pre-fix CSS and incomplete evolution fixture readiness. Original assertions retained; module preloads and exact row identities now require a fresh run.
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
Rust corpus audit of complete workspace run and workspace-context execution-only repair reruns
```

**Notes**

270 targets, 2309 passed cases, zero unresolved failures and 57 declared ignored cases. The initial unfiltered run remains a recorded failure; its two failed private-test targets and the subsequently repaired opt-in server acceptance file have complete fresh passing results. The original and repair run have identical target feature/profile identities. Snapshot b69a0763 and current root-workspace inputs differ only in three private test files; SHA-256 checks bind repaired inputs. The server file differs solely inside its ignored deployment test, and its fresh default target has the original feature/profile identity. All 267 other unchanged passing target summaries are retained, and filtered empty runs never count as replacement coverage.
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
Fresh Storybook67 browser console review
```

**Notes**

Shard3 logged unsuppressed ResizeObserver loop notifications in NativeChangeDraftSurvivesMobileToDesktopRelocation; that case and all 241 shard cases passed, exit zero. These browser warnings are retained and no error filter, dependency patch or assertion suppression was introduced.
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
jj --no-pager st; lsof .git/index.lock; archive stale empty lock; jj --no-pager st
```

**Notes**

Continuation initially found an empty Git index lock older than ten minutes with no owner in two lsof checks and no Git/JJ writer. Archived that lock under the ignored qualification directory, then JJ status/log succeeded. No repository data or active writer lock was removed.
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
gh pr checks 10
```

**Notes**

Hosted standard PR run 38875409846 at exact head 01ea0741ff608bbe14daa8d929448e3ec0790afd passed required, all four standard Rust shards, all seven UI jobs, policy, tooling, formatting and web checks. Compilation-cache experiment and Pages deployment were declared skipped. Heavy manual qualification was not dispatched; a later evidence-only head needs its own hosted result.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
Fresh complete UI inventories after the shared Settings guard repair
```

**Notes**

180 web units; 1193 catalog units across 186 files with native working-tree capture enabled and no skip; all 1021 Storybook cases across 211 files in four entirely fresh shards with exact key equality and no overrides; 114 served Chromium cases; 20 Help cases across Chromium and WebKit. Fresh web/catalog type checks, ownership, architecture, production web and catalog builds, original cold catalog, isolated real local-release/profile restart and HMR acceptance pass. The detailed command history retains original failure and interruption attempts.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
Source-bound local qualification audit
```

**Notes**

101 complete required lanes pass with zero unresolved lanes. Final documentation/ledger consuming validation is recorded in the subsequent command history. Every initial diagnostic failure and interruption remains in the run history. Native compilation was serialized and cold UI qualification used isolated services; user development ports and fixtures were preserved.
<!-- /run -->
<!-- /evidence-field -->

