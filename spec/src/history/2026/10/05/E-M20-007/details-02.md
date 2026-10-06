---
schema: 1
id: E-M20-007
---

# E-M20-007 details 2

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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte -t UnstagedHeaderAndStyledCanvas
```

**Notes**

.local/fixed-columns-evidence/before.log: Import failed before tests: existing local file dependency had changed source hardlinks but omitted the newly added pinning helper; refreshed with the package manager.
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
pnpm install --offline --force --no-frozen-lockfile --ignore-scripts
```

**Notes**

.local/fixed-columns-evidence/dependency-refresh.log: Refreshed existing local Design Core validation dependency. Restored byte-identical tracked pnpm-lock.yaml from the pre-install copy; package.json unchanged. No installed dependency source edited.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte -t UnstagedHeaderAndStyledCanvas
```

**Notes**

.local/fixed-columns-evidence/before-refreshed.log: Valid regression before consumer edit: Unstaged has no data-fixed attribute.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte -t UnstagedHeaderAndStyledCanvas
```

**Notes**

.local/fixed-columns-evidence/after.log: 1 passed, 5 skipped; full width and collapsed pinning, resize, file controls, adjacent diff and staging into feature/workspace.
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
pnpm test:storybook catalog/components/store-stack-workspace/*.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog*.stories.svelte
```

**Notes**

.local/fixed-columns-evidence/consumer-stories.log: 99 passed in 20 files; direct-consumer interaction and accessibility acceptance. Fresh browser process.
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
pnpm web:check
```

**Notes**

.local/fixed-columns-evidence/web-check.log: 0 errors and 0 warnings.
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

.local/fixed-columns-evidence/catalog-check.log: 0 errors and 0 warnings; 219 modules before story extraction.
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

.local/fixed-columns-evidence/ownership.log: Pass: production remains repository-owned, no catalog/source-reference dependency.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

.local/fixed-columns-evidence/architecture.log: Expanded presentation story exceeded the 400-line module bound; moved the pinning scenario into a colocated story file under the same title.
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
pnpm spec:first
```

**Notes**

.local/fixed-columns-evidence/spec-first.log: Requires mapped API chapter alongside UI chapter; documented that pinning adds no transport or adapter fields.
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
pnpm spec:validate
```

**Notes**

.local/fixed-columns-evidence/spec-validate.log: 5 validators, 23 chapters, 173 requirements pass.
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
pnpm spec:first
```

**Notes**

.local/fixed-columns-evidence/spec-first-final.log: 2 protected files and 2 canonical chapters pass before story extraction.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspacePinning.stories.svelte
```

**Notes**

.local/fixed-columns-evidence/pinning-final.log: Extracted identical pinning scenario passes 1/1 interactions/accessibility.
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
pnpm check:architecture
```

**Notes**

.local/fixed-columns-evidence/architecture-final.log: 1025 modules pass.
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
pnpm catalog:check
```

**Notes**

.local/fixed-columns-evidence/catalog-check-final.log: 0 errors/warnings; 220 modules pass after extraction.
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
pnpm exec prettier --write web/src/lib/ui/store-stack-workspace/StoreStackUnstaged.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte spec/src/ui.md spec/src/verification.md
```

**Notes**

.local/fixed-columns-evidence/format.log: Formatted current source and canonical updates; milestones never formatted.
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
pnpm exec prettier --write spec/src/api.md
```

**Notes**

.local/fixed-columns-evidence/format-api.log: Formatted boundary note.
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
pnpm exec prettier --write catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePinning.stories.svelte
```

**Notes**

.local/fixed-columns-evidence/format-split.log: Formatted extracted scenario.
<!-- /run -->
<!-- /evidence-field -->

