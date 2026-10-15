---
schema: 1
id: E-FORGE-DETAILS-001
---

# E-FORGE-DETAILS-001 details 2

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
pnpm exec storybook dev --host 127.0.0.1 --port 6046 --exact-port --no-open --disable-telemetry
```

**Notes**

Restarted this task-owned development server after the warm HMR session stopped answering. Fresh index.json returned HTTP 200. Reloaded the existing contextual browser tab and observed rendered title/body, GitHub icons, Draft badge, two successful checks and one running check, with actions below. Historical warm-session console errors remain distinct from the fresh scoped passes; no Storybook/shared configuration changes or baselines.
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
pnpm spec:first
```

**Notes**

The new vendored icon license maps to the compatibility chapter. Added an explicit JST-COMPAT-001/003 unchanged-contract confirmation for these presentation assets; no protected-check exception or mapping was weakened.
<!-- /run -->

<!-- run -->
### Run 3

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

Before implementation: 2 failed, 5 passed. The contextual story could not find the forge icon in the change PR badge; the new detail story could not find a rendered title. This isolates the requested change from the prior verified header/composer slice.
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

Initial implementation used Button href, unsupported by released Design Core 0.3.2. Corrected the application composition to a semantic external Details anchor; no dependency patch.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeWorkspace.stories.svelte catalog/components/store-forge/StoreForge.stories.svelte
```

**Notes**

First rerun: 6 passed, 2 contextual stories failed importing the optimized axe-core module. Both Vitest projects had been started concurrently against the same optimizer cache. Subsequent cold runs were sequential; no shared test or optimizer configuration changed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/forge.test.ts
```

**Notes**

Initial controller target: 8 passed, including automatic selected-PR check reads, discarded late selection responses and unchanged pending-effect behavior.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeWorkspace.stories.svelte catalog/components/store-forge/StoreForge.stories.svelte
```

**Notes**

Second rerun: 9 passed, 1 failed. The new connection-dialog assertion queried a password input as a textbox; corrected to its associated Session token label. This also makes the initial-detail absence assertion meaningful. No production accessibility relaxation.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/forge.test.ts
```

**Notes**

Final focused controller target: 9 passed. Added stale branch-check coverage: neither stale aggregate SHA nor an individual check from another revision may display passed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeWorkspace.stories.svelte catalog/components/store-forge/StoreForge.stories.svelte
```

**Notes**

Final fresh Vitest/Chromium process: 10 passed, zero interaction/accessibility failures. Covers rendered-first details, equal column headers, forge badge, mixed check rows/counts, edit/save and failed-save toast/draft retention, jobs/logs and run menu, keyboard merge selection, provider-supported methods, Forgejo check-failure toast, GitLab connection menu/dialog and stale observations. No fixture onMount action loads checks on behalf of production.
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
pnpm catalog:check
```

**Notes**

Final zero Svelte errors/warnings; catalog boundaries pass. Only the affected catalog surface was exercised.
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
pnpm web:check
```

**Notes**

Final zero Svelte errors/warnings.
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
node scripts/web-architecture.mjs
```

**Notes**

Maintained module size boundaries pass (1941 modules).
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
pnpm check:ui-ownership
```

**Notes**

New presentation is application-owned, existing Community components stay maintained locally and brand vectors have pinned CC0 provenance. No catalog or external checkout imports in production.
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
pnpm check:progress
```

**Notes**

Fresh same-change record and explicit M31 review pass.
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
pnpm spec:first
```

**Notes**

Owning forge/UI, API unchanged-contract and ownership chapters cover this presentation/controller slice.
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
pnpm exec spec-validator validate --only fileLimits,governance,summary
```

**Notes**

Evidence and comparison navigation generated for this directory only; authored page budgets pass.
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
pnpm exec prettier --check catalog/components/store-forge/StoreForge.stories.svelte catalog/components/store-forge/StoreForgeWorkspace.stories.svelte catalog/fixtures/ForgeFixture.svelte catalog/fixtures/ForgeWorkspaceFixture.svelte catalog/fixtures/forge/workspace-transport.ts docs/ui-provenance.json spec/src/current/contracts/api/38-ordinary-local-stacks-transport.md spec/src/current/contracts/architecture/24-forge-stacks.md spec/src/current/contracts/compatibility/13-additive-ordinary-stacks-topology.md spec/src/current/contracts/ui/04-local-stack-workspace-catalog.md spec/src/current/governance/spec-governance.md spec/src/current/progress/M31/index.md web/src/lib/controller/forge/controller.svelte.ts web/src/lib/ui/store-forge/StoreForgeButton.svelte web/src/lib/ui/store-forge/StoreForgeCheckIcon.svelte web/src/lib/ui/store-forge/StoreForgeCheckRow.svelte web/src/lib/ui/store-forge/StoreForgeChecks.svelte web/src/lib/ui/store-forge/StoreForgeDescription.svelte web/src/lib/ui/store-forge/StoreForgeIcon.svelte web/src/lib/ui/store-forge/StoreForgePanel.svelte web/src/lib/ui/store-forge/StoreForgePanelActions.svelte web/src/lib/ui/store-forge/StoreForgePullRequest.svelte web/src/lib/ui/store-forge/forge-icons.ts web/src/lib/ui/store-forge/store-forge.css web/tests/vite/forge.test.ts spec/src/history/2026/10/15/E-FORGE-DETAILS-001/design-comparison.md
```

**Notes**

All changed authored sources pass. Vendored Simple Icons license retains upstream text; generated evidence/navigation use their canonical renderer.
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

Node v26.9.0; pnpm 10.32.1; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11 with Chromium; released Design Core 0.3.2 and Mira 0.0.3. Simple Icons 16.0.0 paths bundled locally. No Rust builds/tests.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing version-1 Forge actions and expected body/head guards are unchanged. Automatic checks use the existing read-only action with selection epoch/identity guards, never a pending mutation. Source Markdown and safe rendering are unchanged; credentials remain session-only. No public API, schema generation, storage, WAL or persistence format changes.
<!-- /evidence-field -->

