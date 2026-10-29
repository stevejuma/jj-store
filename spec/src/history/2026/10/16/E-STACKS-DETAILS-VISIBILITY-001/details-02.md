---
schema: 1
id: E-STACKS-DETAILS-VISIBILITY-001
---

# E-STACKS-DETAILS-VISIBILITY-001 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
jj --no-pager st; pnpm spec:search -- JST-UI-014; pnpm spec:search -- JST-UI-020
```

**Notes**

Baseline clean. The documented extra -- is rejected by this installed specification CLI; no product test ran.
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
pnpm spec:search JST-UI-014; pnpm spec:search JST-UI-020
```

**Notes**

QMD native modules mismatch Node ABI 137. Used the reported rg fallback and read the canonical requirements, Settings/Stacks topics and M21 ledger.
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
pnpm evidence:new -- --id E-STACKS-DETAILS-VISIBILITY-001 --task M21.1 --requirement JST-UI-012 --requirement JST-UI-014 --requirement JST-UI-020
```

**Notes**

Created this evidence. Restored 452 unrelated generated-navigation rewrites; retained only this record and its date navigation.
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
pnpm test:storybook src/shared/shell/app-shell/AppShellVisibility.stories.svelte -t "Closed sidebar retains"; pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte -t "Closed Details stays absent"
```

**Notes**

Exploratory filters matched no generated story names: source 2 skipped and consumer 8 skipped. These are not regression passes; corrected filters/runs follow.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cd ../design-core && pnpm test:storybook src/shared/shell/app-shell/AppShellVisibility.stories.svelte
```

**Notes**

Before source fix: closed hide/show regression fails (hidden=false instead of true); open panel baseline passes. 1 failed, 1 passed.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte -t "Closed.*Details"
```

**Notes**

Before fix against released Design Core 0.5.1: 1 failed, 7 skipped. Return from Repository settings exposes the closed rail.
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
cd ../design-core && pnpm test:storybook src/shared/shell/app-shell/AppShellVisibility.stories.svelte; pnpm test:storybook src/shared/shell/app-shell
```

**Notes**

After source fix: both exact visibility stories pass; all 19 shell stories pass in fresh Vitest browser runs, including a rerun after limiting reconciliation to desktop. Source fix is a shared shell effect; no copied dependency implementation or consumer CSS workaround.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cd ../design-core && pnpm checks
```

**Notes**

Initial run stopped on two new documentation diagnostics: unmapped story title and multi-sentence acceptance bullet. Corrected the title to the existing Shell/App Shell family and made the acceptance bullet atomic; rerun separately.
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
cd ../design-core && node --input-type=module -e 'import { packLocal } from "./scripts/pack-local.mjs"; await packLocal(process.cwd(), {suffix: "-sidebar-visibility"});'
```

**Notes**

Built twice, with the second archive selecting the final source. Final archive .local/packages/lapismd-design-core-0.5.1-sidebar-visibility.tgz SHA-256 4bed98d473e8376a7d855e3aea1eee81a8246eb6bf3e132b644f03f7da43ea3d. Browser CSS prepared: 3 compiled, 221 audited, no remaining utility references.
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
pnpm install --no-frozen-lockfile --ignore-scripts --config.pnpmfile=.local/sidebar-visibility-pnpmfile.cjs; jj restore pnpm-lock.yaml
```

**Notes**

Temporary ignored root-only hook selects the verified owning-library archive for review/consumer acceptance. One dependency replaced; tracked registry lock restored exactly. No Community substitution, source alias, node_modules patch or npm publication.
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
node package-location inspection using require.resolve("@lapismd/design-core/package.json")
```

**Notes**

Package metadata is not an exported subpath. Corrected read-only verification uses fs.realpathSync("node_modules/@lapismd/design-core") and confirms the packed final AppShellSidebar implementation.
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
pnpm dev:ui
```

**Notes**

Cold restart of owned Vite/backend only; interactive native fixture preserved, search ready, review URL http://127.0.0.1:7460/. No Rust rebuild/reset or installed-tool change.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte -t "Closed.*Details"
```

**Notes**

After verified archive selection: 1 passed, 7 skipped, fresh Vitest browser. Full directly affected story families run separately.
<!-- /run -->

<!-- run -->
### Run 14

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

New test used Playwright-only exact options on Testing Library queries. Removed the unsupported options without changing assertions.
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
pnpm catalog:check; pnpm check:ui-ownership; pnpm exec prettier --check catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte spec/src/current/contracts/ui/04-local-stack-workspace-catalog.md
```

**Notes**

Catalog typecheck: zero errors/warnings; boundaries 466 modules. UI ownership passes. Changed story and contract formatting passes.
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
cua_repl live browser: cold Stacks load, Repository settings, Back to repository, Toggle details; repeat Settings round trip while open
```

**Notes**

Closed rail after return: hidden=true, state=closed, width=0. Explicit open renders Stack workspace views with state=expanded, width=220. Open Settings round trip preserves 220px content. Final view restored closed. No repository command or configuration mutation. Screenshot .local/stacks-details-after.jpg.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-application/SettingsShell.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte
```

**Notes**

All 20 cases across 3 directly affected story families pass with the packaged source repair, including existing Settings and repository navigation interactions/accessibility.
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
pnpm check:progress
```

**Notes**

156 tasks and 846 evidence records pass; bookkeeping only, no milestone promotion.
<!-- /run -->
<!-- /evidence-field -->
