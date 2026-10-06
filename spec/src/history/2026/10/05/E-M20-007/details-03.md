---
schema: 1
id: E-M20-007
---

# E-M20-007 details 3

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
pnpm storybook --port 6057
```

**Notes**

.local/fixed-columns-evidence/storybook.log: Restarted only the owned consumer Storybook after refresh; live app-shell browser verification completed.
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
pnpm check:progress
```

**Notes**

.local/fixed-columns-evidence/progress.log: 141 tasks pass; evidence bookkeeping only.
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
pnpm spec:first
```

**Notes**

.local/fixed-columns-evidence/spec-first-recorded.log: 3 protected files, 3 canonical chapters pass after story extraction and ledger update.
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
pnpm spec:validate
```

**Notes**

.local/fixed-columns-evidence/spec-validate-recorded.log: 5 validators, 23 chapters, 173 requirements pass with new evidence.
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
pnpm exec prettier --check catalog/components/store-stack-workspace/StoreStackWorkspacePinning.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte web/src/lib/ui/store-stack-workspace/StoreStackUnstaged.svelte spec/src/api.md spec/src/ui.md spec/src/verification.md
```

**Notes**

.local/fixed-columns-evidence/format-final.log: all changed source and canonical prose is formatted; historical ledger excluded.
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
pnpm checks
```

**Notes**

Run in /Users/stevejuma/code/lapismd/design-core; .local/fixed-columns-evidence/source/checks-confirmed.log. Required full nonvisual gate passes; no screenshot comparison or baseline changes.
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
pnpm spec:check
```

**Notes**

Run in /Users/stevejuma/code/lapismd/design-core after final evidence edits; .local/fixed-columns-evidence/source/spec-check-final.log passes validate, mdbook and spec-first.
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
pnpm check:progress
```

**Notes**

.local/fixed-columns-evidence/progress-final.log: 141 tasks pass after final source evidence was recorded.
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
pnpm spec:first
```

**Notes**

.local/fixed-columns-evidence/spec-first-final-ledger.log: 3 protected files and 3 canonical chapters pass.
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
pnpm spec:validate
```

**Notes**

.local/fixed-columns-evidence/spec-validate-final.log: 5 validators, 23 chapters and 173 requirements pass.
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
pnpm exec prettier --write spec/src/verification.md
```

**Notes**

.local/fixed-columns-evidence/format-verification-final.log: Final source revision reference formatted.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:stories list -->
## Stories

- catalog/components/store-stack-workspace/StoreStackWorkspacePinning.stories.svelte#Unstaged
  header and styled canvas
<!-- /evidence-field -->

<!-- evidence-field:browser_acceptance markdown -->
## Browser acceptance

Owned Storybook restarted on literal loopback port 6057. In-app browser confirms pinned Unstaged at x=289 with retained 330px expanded width while feature/workspace is visible after horizontal scrolling. Screenshot: .local/fixed-columns-evidence/pinned-unstaged.jpg. No screenshot comparison or baseline updates.
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- UI review and visual approval remain separate.
- Registry-resolved Design Core qualification requires a separately approved
  release; manifests remain portable.
- Stack backend execution, physical checkout and durable combinations require
  explicit approval after UI review.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review UI; proceed with backend integration only after explicit approval.
<!-- /evidence-field -->

<!-- evidence-field:source_verification markdown -->
## Source verification

Design Core committed separately as f19b990e (mvvwvwon). Full pnpm checks passes with 722 units, 663 stories, 14 Workspace/11 Shell/3 AI/48 shadcn browser passes and one existing platform-specific skip. Existing autocomplete story failed in one aggregate run, passed unchanged in isolation, then passed in the final complete aggregate. Source evidence copied under .local/fixed-columns-evidence/source; source specification/record owns detailed intermediate failures and fixes.
<!-- /evidence-field -->

