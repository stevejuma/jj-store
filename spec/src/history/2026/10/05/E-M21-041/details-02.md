---
schema: 1
id: E-M21-041
---

# E-M21-041 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSetup.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte -t 'Unconfigured workspace|Workspace in app shell|Branches in app shell'
```

**Notes**

before-stories.log: no tests selected because Storybook uses exported PascalCase test names; all ten skipped. This is not acceptance.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSetup.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte
```

**Notes**

before-stories-full.log: 4 expected spacing failures and 6 passes.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSetup.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePinning.stories.svelte
```

**Notes**

after-stories.log: 11/11 interaction/accessibility stories before adding Branches fixed coverage.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspacePinning.stories.svelte
```

**Notes**

before-branches-fixed.log: new story rejects the missing fixed property; existing Unstaged passes.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSetup.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePinning.stories.svelte
```

**Notes**

final-stories.log: 11 pass; new test incorrectly requested README.md from branch files. Corrected to the actual src/api.ts fixture.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSetup.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePinning.stories.svelte
```

**Notes**

final-stories-corrected.log: 11 pass; new test retained the DOM element removed by collapse. Corrected to measure the re-expanded element.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSetup.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePinning.stories.svelte
```

**Notes**

verified-stories.log: 12/12. Setup margins and canvas inset geometry; Branches and Unstaged expanded/collapsed positions and widths within one CSS pixel, adjacent diffs and drag behavior.
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

web-check.log and web-check-final.log: zero errors/warnings.
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

catalog-check.log and catalog-check-final.log: zero errors/warnings; 227 module boundaries pass.
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
pnpm check:ui-ownership
```

**Notes**

ownership.log: owning presentation boundary passes.
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
CUA read-only live browser computed-style inspection at http://127.0.0.1:7476/
```

**Notes**

Existing Workspace view reports canvas padding-inline-start 0px, immediate data-scroll-area-content parent 12px; user repository selection was preserved.
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
pnpm exec prettier --write (changed supported source and spec paths only)
```

**Notes**

Only six changed supported files formatted, then Pinning/Canvas/ui chapter formatted. Historical ledger entries are not reformatted.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Initial ledger used story/helper references in tests, which accepts only native named fn/def/test definitions. Story acceptance remains in runs and files; invalid references removed.
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
pnpm spec:first
```

**Notes**

Initial UI slice lacked api.md required by the existing web change map; added explicit client-presentation boundary note.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspacePinning.stories.svelte
```

**Notes**

before-persistent-gutter.log: both tabs pin flush to viewport edge, 12px short of required outer gutter.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSetup.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePinning.stories.svelte
```

**Notes**

gutter-stories.log: 12/12. Both fixed columns measure 12px from outer Scroll Area edge while expanded, collapsed and restored; gutter hit tests exclude column content. Setup four-sided margins and initial non-doubled inset pass.
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
pnpm web:check; pnpm catalog:check; pnpm check:ui-ownership
```

**Notes**

gutter-*-check and gutter-ownership logs: zero diagnostics, 227 catalog boundaries and ownership pass.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress
```

**Notes**

Mapped spec-first, 23 chapters / 173 requirements and 142-task progress bookkeeping pass.
<!-- /run -->
<!-- /evidence-field -->

