---
schema: 1
id: E-CHAT-FOOTER-001
---

# E-CHAT-FOOTER-001

<!-- evidence-field:id markdown -->
## Id

E-CHAT-FOOTER-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-001
- JST-UI-015
- JST-API-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-09
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/store-agent-chat-body/ChatMessageFooter.svelte
- web/src/lib/ui/community-chat-composer/CommunityComposerToolbar.svelte
- catalog/components/store-application/ConversationFooterLayout.stories.svelte
- spec/src/current/contracts/ui/37-document-conversations.md
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M24/history/M24/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

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

Before change: collapsed toolbar last control extends to x=533 beyond toolbar right=495 at 390px footer width. Log: .local/footer-layout-before.log.
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
pnpm test:storybook catalog/components/store-application/ConversationFooterLayout.stories.svelte
```

**Notes**

After change: 1 interaction/accessibility story passes at 390px, 280px and 760px. Protects collapsed controls, puts workspace and Send below expanded formatting, keeps footer bounded and uses Tab to reach Quote. Log: .local/footer-layout-after.log.
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
pnpm test:storybook catalog/components/store-application/ConversationContinuity.stories.svelte catalog/components/store-application/WorkspaceFiles.stories.svelte -t 'DocumentPrompt|WorkspaceFooter|WorkspaceMenu'
```

**Notes**

3 direct-consumer stories pass, 10 filtered. Covers Document/Classic draft and selection retention plus workspace menu/copy behavior. Log: .local/footer-consumers.log.
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
pnpm web:check
```

**Notes**

0 errors, 0 warnings.
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
pnpm catalog:check
```

**Notes**

0 errors, 0 warnings; 353 catalog modules pass boundaries.
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
pnpm check:ui-ownership
```

**Notes**

Existing owned components retain public Design Core imports; no new upstream copies.
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
cua_repl live session eeb7049d90fcd39c6610f89c2f7102bc: View files, open src/example.ts, toggle Show formatting and Hide formatting
```

**Notes**

With both panels open at a 1280px viewport: expanded toolbar bottom=623, repository top=635, Send top=671. Footer scrollWidth/clientWidth both 365; document scrollWidth=viewport=1280. Collapsed controls remain visible; no warnings/errors in fresh tab logs. Closed temporary tab; no prompt, publication, reset or backend restart. HMR applied the change.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check web/src/lib/ui/store-agent-chat-body/ChatMessageFooter.svelte web/src/lib/ui/community-chat-composer/CommunityComposerToolbar.svelte catalog/components/store-application/ConversationFooterLayout.stories.svelte spec/src/current/contracts/ui/37-document-conversations.md spec/src/current/contracts/api/41-interactive-agent-execution.md
```

**Notes**

New story needed line wrapping; corrected with formatter.
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
pnpm exec prettier --write catalog/components/store-application/ConversationFooterLayout.stories.svelte
```

**Notes**

Only formatting changed.
<!-- /run -->

<!-- run -->
### Run 10

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

Evidence tests field accepts registered test IDs, not story paths. Removed the invalid test reference; the exact Storybook commands and path remain recorded under runs/files.
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
pnpm exec prettier --check web/src/lib/ui/store-agent-chat-body/ChatMessageFooter.svelte web/src/lib/ui/community-chat-composer/CommunityComposerToolbar.svelte catalog/components/store-application/ConversationFooterLayout.stories.svelte spec/src/current/contracts/ui/37-document-conversations.md spec/src/current/contracts/api/41-interactive-agent-execution.md
```

**Notes**

Final changed source/story/chapter formatting passes.
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
pnpm exec spec-validator validate --only governance,summary,fileLimits,book
```

**Notes**

4 source-document validators pass.
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
pnpm check:progress
```

**Notes**

Final evidence schema passes: 146 tasks, 709 evidence records.
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
pnpm spec:first
```

**Notes**

3 protected files, 2 owning canonical chapters.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- M24 remains In progress. Independent workspace publication/recovery and visual
  baseline gates remain open. Existing broader ResizeObserver diagnostics are
  not claimed resolved; none reproduced in the focused layout/live checks.
- Rust, generated transport, root aggregate, full catalog, built-bundle and
  platform qualification lanes are out of scope for this UI-only layout slice.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; pnpm 10.32.1; Vitest 4.1.11; Svelte 5.57.1; macOS ARM64. Evidence revision is the enclosing Jujutsu change.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No storage, API, preference, source-anchor, publication or request identity formats change.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Community toolbar reference inspected first, then public Design Core button sizing. Owned docked footer reserves intrinsic collapsed action width, bounds it to the available panel, and uses the toolbar formatting state to allocate the first row when expanded. Classic and undocked composers retain their existing layout.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-FOOTER-001 details 2](details-02.md)
