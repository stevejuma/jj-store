---
schema: 1
id: E-CHAT-WORKSPACE-005
---

# E-CHAT-WORKSPACE-005 details 2

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
pnpm check:progress
```

**Notes**

Final ledger coverage passes for 146 tasks and 703 evidence records.
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
pnpm spec:first
```

**Notes**

Final protected UI/controller/provenance coverage passes: 12 protected files, three canonical chapters.
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
pnpm spec:validate
```

**Notes**

Seven document validators pass for the final evidence and canonical chapters.
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
pnpm check:progress
```

**Notes**

146 tasks and 703 evidence records pass after correcting Storybook references and regenerated-history coverage.
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
pnpm spec:first
```

**Notes**

Change map additionally requires API and governance chapters for conversation-controller source and provenance edits. Added explicit unchanged JST-API-001 and JST-GOV-008 confirmations.
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
pnpm check:progress
```

**Notes**

Evidence referenced generated Storybook IDs as named unit functions; the ledger only accepts source test()/fn/def references. Kept exact Storybook names in run records instead and included regenerated M24 history in the fresh file coverage.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Blocked by the same initial evidence reference and generated-history coverage errors; corrected evidence without changing checks.
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
pnpm spec:validate
```

**Notes**

Seven document validators pass.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/WorkspaceFiles.stories.svelte
```

**Notes**

Before the skeleton/toggle additions: three cold stories pass, including inbox to Projects/repository/other-session navigation and restoration of both open and closed original rail state.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/WorkspaceFiles.stories.svelte
```

**Notes**

Four stories pass after the header toggle and skeleton additions. Svelte emitted three unused child selectors in the extracted skeleton; corrected selectors to target the owned public Skeleton children.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/WorkspaceFiles.stories.svelte catalog/components/store-application/RepositoryLayout.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte -t 'Workspace|Inbox header|Standalone footer|Overview|File detail'
```

**Notes**

Three pass, two fail, nineteen skipped. CamelCase generated story names caused two intended cases to be filtered out; corrected the filter. Both failing OverviewBodyInset checks expect at least 12px where actual repository body padding is 0px. No assertion weakened.
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
jj workspace add /tmp/jj-store-rail-baseline-20261009 --name rail-baseline-20261009 -r @- -m "Disposable before-state rail regression verification"
```

**Notes**

Isolated prior-revision workspace: notqskxt / 7126e22a, before this UI correction. Main checkout and live demo were untouched.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryLayout.stories.svelte
```

**Notes**

First isolated baseline attempt used a node_modules symlink; Storybook dynamic setup import failed before any tests. This is not baseline behavior evidence.
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
pnpm install --offline --frozen-lockfile --ignore-scripts
```

**Notes**

Isolated baseline installation refused the existing local Design Core lockfile/manifest mismatch. Neither file was changed.
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
cp -cR /Users/stevejuma/code/lapismd/jj-store-chat-body/node_modules /tmp/jj-store-rail-baseline-20261009/node_modules
```

**Notes**

Reused the exact installed dependencies as filesystem clones after removing only the temporary symlink.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryLayout.stories.svelte
```

**Notes**

With resolved dependencies, both prior-revision OverviewBodyInset and OverviewMobileBodyInset fail at the same existing assertion: expected 0 to be greater than or equal to 12. Owner: repository body layout stories/spacing contract. Retained for separate review, not waived as a release gate.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/WorkspaceFiles.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte -t 'StandaloneFooter|WorkspaceRefresh|WorkspacePanels|InboxHeader|FileDetail'
```

**Notes**

Final cold run: 5 pass, 17 explicitly filtered out. Covers independent rail layouts, cross-route and cross-session scoping, direct header open/close/reopen, positive skeleton geometry, requested path header, loading-to-content replacement and existing repository Markdown skeleton. Interaction/accessibility checks enabled.
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
pnpm web:check
```

**Notes**

Zero errors/warnings for the affected web surface.
<!-- /run -->
<!-- /evidence-field -->

