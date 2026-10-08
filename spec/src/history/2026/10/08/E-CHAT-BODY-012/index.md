---
schema: 1
id: E-CHAT-BODY-012
---

# E-CHAT-BODY-012

<!-- evidence-field:id markdown -->
## Id

E-CHAT-BODY-012
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

2026-10-08
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Reversible captured-code and diff middle folds
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change pqmyoyot above yvqxmrms in jj-store-chat-body
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Expanding a middle fold removed its control entirely. The narrow LargeCodeAndDiffMiddleFolding regression failed once because Show less in Large captured code was absent.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

Expanded captures retain a block widget at the original fold boundary with Show less, a 16px decorative Lucide ChevronDown and aria-expanded=true. The collapsed control uses Lucide ChevronRight at the same size. The toggle gains a theme-derived hover background while retaining its focus ring. Icon components are unmounted when their CodeMirror widget is destroyed. Collapsing restores the same leading/trailing lines and Show N more lines with aria-expanded=false. Widget equality includes the expanded state so its label and handler update correctly. Both states remain revision-keyed and survive disclosure/body remounts. Focus returns to the immutable reader; collapse places the caret on the preceding visible line. Existing exact-anchor protection still reveals ranges containing active or submitted discussions. Captured text, Unicode offsets and unrelated file folds remain unchanged. Community and Design Core reference searches found no applicable middle-fold control; the existing owned CodeMirror widget is extended without dependency changes.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/fixtures/delta-chat/large-capture-native.mjs
- catalog/fixtures/delta-chat/large-capture-play.ts
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/ui/36-commentable-chat-body-prototype.md
- spec/src/current/progress/M24/index.md
- spec/src/current/verification/ui-02.md
- spec/src/current/verification/ui.md
- spec/src/history/2026/10/08/E-CHAT-BODY-012/SUMMARY.nav.md
- spec/src/history/2026/10/08/E-CHAT-BODY-012/details-02.md
- spec/src/history/2026/10/08/E-CHAT-BODY-012/index.md
- spec/src/history/2026/10/08/SUMMARY.nav.md
- spec/src/history/2026/10/08/index.md
- web/src/lib/ui/store-agent-chat-body/middle-fold.ts
- web/src/lib/ui/store-agent-chat-body/text-reader.css
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t LargeCodeAndDiffMiddleFolding
```

**Notes**

Missing Show less control after expansion; 1 failed, 20 skipped. .local/chat-refold-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t LargeCodeAndDiffMiddleFolding
```

**Notes**

Pointer expand/collapse/re-expand for code and patches preserves the trailing text, independent fold state, comments and remount behavior. 1 passed, 20 skipped. .local/chat-refold-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit --project storybook catalog/fixtures/delta-chat/middle-fold.test.ts catalog/components/store-application/AgentChatBody.stories.svelte
```

**Notes**

3 folding/Unicode/anchor units and 21 affected chat interaction/accessibility stories pass (24 total), rerun after the Lucide icon lifecycle and hover refinement. .local/chat-refold-tests.log.
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

0 errors, 0 warnings. .local/chat-refold-web.log.
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

0 errors, 0 warnings; 338-module catalog boundaries pass. .local/chat-refold-catalog.log.
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

Owned UI boundary passes. .local/chat-refold-ownership.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Evidence test references used bare story/file names instead of structured references. Removed the invalid optional list; exact test names, paths and commands remain in the run records.
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
pnpm spec:first
```

**Notes**

The same invalid evidence references prevented the change-set gate; corrected the evidence metadata.
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
pnpm exec spec-validator validate --only summary,fileLimits,governance,verification,book
```

**Notes**

Prettier table padding pushed ui-02.md to 33,099 bytes above its 32,768-byte limit. Preserved the existing table and added the new evidence as a following paragraph.
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
node catalog/fixtures/delta-chat/browser.acceptance.mjs > .local/chat-refold-cold.log 2>&1
```

**Notes**

Cold keyboard Space expand, Enter collapse, collapsed-state remount and re-expand pass, along with existing source/comment navigation, gutter, resizing and footer acceptance. Private browser/server cleaned up; user preview retained.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:progress
```
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm spec:first
```
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec spec-validator validate --only summary,fileLimits,governance,verification,book
```
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --check <changed authored files>
```
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Cold Vite development logs include seven ResizeObserver-loop notifications in
  .local/chat-body-cold.log. Browser assertions pass; removal of these
  previously recorded runtime diagnostics remains unqualified.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node v26.9.0; pnpm 10.32.1; jj 0.45.1; Vitest 4.1.11. Released Design Core/Mira dependencies unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No captured fixture, UTF-16 anchor, persisted format, HTTP or ACP changes. Disclosure state remains UI-local and keyed by exact block ID and revision.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-BODY-012 details 2](details-02.md)
