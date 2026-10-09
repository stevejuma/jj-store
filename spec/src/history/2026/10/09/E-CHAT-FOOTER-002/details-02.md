---
schema: 1
id: E-CHAT-FOOTER-002
---

# E-CHAT-FOOTER-002 details 2

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
pnpm test:storybook catalog/components/store-application/ConversationFooterLayout.stories.svelte
```

**Notes**

Before change: new wrapped-start regression finds a 78px offset between repository and toolbar left edges. Log: .local/footer-rest-before.log.
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
pnpm test:storybook catalog/components/store-application/ConversationFooterLayout.stories.svelte catalog/components/store-application/WorkspaceFiles.stories.svelte -t 'FormattingActions|WorkspaceFooter|WorkspaceMenu'
```

**Notes**

Secondary variant and start alignment: 3 passed, 8 filtered. Existing pointer-hover and menu/copy behavior pass. Log: .local/footer-rest-after.log.
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
pnpm test:storybook catalog/components/store-application/ConversationFooterLayout.stories.svelte catalog/components/store-application/WorkspaceFiles.stories.svelte -t 'FormattingActions|WorkspaceFooter|WorkspaceMenu'
```

**Notes**

After label-shrink change: 2 passed, 1 failed, 8 filtered. New 548px single-row assertion failed with 44px row offset. Both fixture runtime labels are Harness default, longer than the live Medium / 5.6 Luna labels. Log: .local/footer-rest-final.log.
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
pnpm test:storybook catalog/components/store-application/ConversationFooterLayout.stories.svelte
```

**Notes**

Diagnostic confirms the failure is the newly added collapsed 548px assertion. The prior clipping/alignment assertions still pass. Corrected the single-row assertion to 760px, where both long runtime labels fit only if the repository shrinks below its 18rem cap; retained 548/390/280 wrap checks and expanded-row alignment at every width. Log: .local/footer-rest-diagnostic.log.
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
pnpm test:storybook catalog/components/store-application/ConversationFooterLayout.stories.svelte catalog/components/store-application/WorkspaceFiles.stories.svelte -t 'FormattingActions|WorkspaceFooter|WorkspaceMenu'
```

**Notes**

Final: 3 passed, 8 filtered, including interaction/accessibility, native pointer hover, workspace menu/copy, label shrinking, wrapped left alignment, expanded controls, no overflow and Tab access. Fresh Vitest/Vite process. Log: .local/footer-rest-final2.log.
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

0 errors, 0 warnings. Log: .local/footer-rest-web.log.
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

0 errors, 0 warnings; 353 modules pass boundaries. Log: .local/footer-rest-catalog.log.
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

Owned layout uses the public Design Core secondary variant; no upstream override. Log: .local/footer-rest-ownership.log.
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
cua_repl live footer inspection: session eeb7049d90fcd39c6610f89c2f7102bc
```

**Notes**

Actual footer width=548.289px, scrollWidth=548. Toolbar and Send top=939px, height=32px; repository top=943px, height=24px, width=93.984px. All controls share one centered row and label truncates. Rest background rgb(240,240,240); temporary-tab pointer hover changes it to themed oklab(... / 0.8). Expanded repository and toolbar left both 365.711px. Latest user-tab warn/error log is empty. Temporary tab closed, user tab retained. No prompt, publication, reset or restart.
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
pnpm exec prettier --write catalog/components/store-application/ConversationFooterLayout.stories.svelte spec/src/current/contracts/ui/37-document-conversations.md
```

**Notes**

Formatted initial additions; later story diagnostics were formatted again with the same command targeting only that story.
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
pnpm exec prettier --check web/src/lib/ui/store-agent-workspace/StoreWorkspaceButton.svelte catalog/components/store-application/ConversationFooterLayout.stories.svelte catalog/components/store-application/WorkspaceFiles.stories.svelte spec/src/current/contracts/ui/37-document-conversations.md spec/src/current/contracts/api/41-interactive-agent-execution.md
```

**Notes**

Changed source, stories and chapters formatted.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node .local/footer-rest-evidence.mjs
```

**Notes**

Evidence written, but navigation generation refused the M24 index: no navigation room. New review reached 404 physical lines; moved the two footer reviews unchanged into a linked topic page.
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
pnpm exec spec-validator validate --only governance,summary,fileLimits,book
```

**Notes**

1 file-limit violation: M24 index reached 404 lines; resolved by the footer-review topic split.
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

146 tasks, 710 evidence records; bookkeeping passes.
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

3 protected files, 2 canonical chapters.
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
pnpm exec spec-validator validate --only governance,summary,fileLimits,book
```

**Notes**

After topic split: 4 validators pass, 3389 chapters, 181 requirements.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check spec/src/current/progress/M24/footer-review.md
```

**Notes**

New generated-navigation whitespace needed formatting.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress; pnpm spec:first
```

**Notes**

Both rejected missing evidence coverage for the new M24 SUMMARY.nav.md. Added that file to this evidence record.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

