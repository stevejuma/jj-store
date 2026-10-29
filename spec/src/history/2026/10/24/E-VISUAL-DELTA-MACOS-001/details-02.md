---
schema: 1
id: E-VISUAL-DELTA-MACOS-001
---

# E-VISUAL-DELTA-MACOS-001 details 2

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
pnpm exec visual-delta doctor --json
```

**Notes**

Before integration: command not found.
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
pnpm add -Dw --ignore-pnpmfile @lapismd/storybook-addon-visual-delta@^0.1.0
```

**Notes**

Pinned Node 24.15.0/pnpm 10.32.1 driver; registry manifest and lockfile. No sibling Visual Delta alias or source copy.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 23
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Registry Design Core 0.5.1 lacks shell APIs already used by 12 unchanged catalog/production files. The checkout previously used its existing .local/shell-layout Design Core 0.6.0 archive. This is a pre-existing foundation release gap, not an addon type error.
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
pnpm install --lockfile=false --config.pnpmfile=.local/shell-layout/pnpmfile.cjs
```

**Notes**

Restored the pre-existing documented packed Design Core development environment using its unchanged ignored hook. The committed manifest/lock remain registry-only. Acceptance below uses this temporary environment and does not qualify registry Design Core delivery.
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
pnpm add -Dw --ignore-pnpmfile --save-exact playwright@1.63.0 @playwright/test@1.63.0
```

**Notes**

Explicit matching runtime/test peers make the installed Playwright accessible from the project root. The prior test version was already 1.63.0. Restored the same development archive after installation.
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
pnpm --silent visual-delta doctor --runner --json
```

**Notes**

After registration and direct Playwright pairing: 16 pass, 0 warnings, 0 errors against a complete build. Local selection is native Chromium at profiles/native/darwin-arm64. CI=true separately resolves shared Docker defaults and ignores the local file. Thresholds remain the package defaults.
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

0 Svelte errors/warnings; catalog boundaries pass for 473 modules with the original packed Design Core development dependency.
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
pnpm catalog:test
```

**Notes**

188 files; 1205 tests pass, 1 existing skip.
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
pnpm test:storybook catalog/components/community-chip/CommunityChip.stories.svelte
```

**Notes**

The exact Chip story passes interaction/accessibility with the released addon registered.
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
pnpm build-storybook
```

**Notes**

Cold build emits complete index/iframe plus .visual-delta/cache/preview-stats.json. Static read-only addon registration builds successfully.
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
pnpm visual-delta test --story-id jj-store-components-chip--bookmark --snapshot-dir .local/visual-delta-setup/native-proof --fresh --failure-mode strict --skip-build
```

**Notes**

Initial native attempt lacked the full Chromium executable although headless-shell probes worked. Resolved by installing the selected browser below.
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
pnpm exec playwright install chromium
```

**Notes**

Installed full Chromium 153.0.8010.12 (v1243) for project Playwright 1.63.0.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm visual-delta test --story-id jj-store-components-chip--bookmark --snapshot-dir .local/visual-delta-setup/native-proof --fresh --failure-mode strict --skip-build
```

**Notes**

Expected strict missing-baseline result. Real native capture persists an actual PNG and schema-4 sidecar with schema-2 Darwin ARM64 provenance, Node 26.9.0 and Playwright 1.63.0. No story source or committed PNG is written.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node --test scripts/catalog-browser.test.mjs
```

**Notes**

Cold catalog failure waits for the exact src/lib.rs button at line 198.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node --test .local/visual-delta-setup/parent-cold.test.mjs
```

**Notes**

The same cold failure reproduces against a separately built parent Storybook without Visual Delta. Only the static output path and helper import were adapted in the ignored harness.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config .local/visual-delta-setup/parent-vitest.config.ts catalog/components/store-application/AgentChatBody.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte -t "MultipleAdjacentDiffs|CapturedLineNumberGutter|ResizableReadSidebar|MessageReactionsAndXMLContext"
```

**Notes**

All four initially selected failures reproduce without the addon; 27 unrelated cases are intentionally not selected. The parent harness uses the exact parent main configuration and the unchanged preview/setup/Vite configuration. Early alternate-directory discovery attempts were discarded after the harness reported no test files.
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
pnpm exec prettier --check .storybook/main.ts .visual-delta/config.json .visual-delta/config.local.example.json playwright.config.ts tests/visual/storybook.spec.ts docs/catalog.md spec/src/current/contracts/ui/01-storybook-presentation-catalog.md spec/src/current/contracts/architecture/06-browser-presentation-ownership.md spec/src/current/governance/09-catalog-validation.md package.json
```

**Notes**

Owned code/configuration/document formatting passes.
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
pnpm visual-delta test --story-id jj-store-components-chip--bookmark --snapshot-dir .local/visual-delta-setup/native-proof --fresh --failure-mode strict --skip-build
```

**Notes**

Fresh native capture passes unchanged comparison. The ignored fixture baseline SHA-256 remains identical. Actual PNG and result persist under the native Darwin ARM64 artifact namespace with truthful Node 26.9.0/Playwright 1.63.0/Chromium 153.0.8010.12 provenance. No committed baseline or story source changes.
<!-- /run -->
<!-- /evidence-field -->

