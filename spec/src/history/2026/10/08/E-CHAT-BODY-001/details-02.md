---
schema: 1
id: E-CHAT-BODY-001
---

# E-CHAT-BODY-001 details 2

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
pnpm install --frozen-lockfile
```

**Notes**

New workspace dependencies. No package manifest/lockfile changes. Inherited pnpmfile resolves Design Core to sibling file source; Mira resolves released 0.0.2.
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
/Applications/Delta.app/Contents/MacOS/delta thread data ksQQH9ApWKbqSVOy24cbEDlr05LNsrWSAcQQhG3eRXMDSBKZ2xnydif2Vg
```

**Notes**

Untouched export remains ignored in .local/delta-chat-reference.json. Main thread only, 39 native records including an excluded composer draft.
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
/Applications/Delta.app/Contents/MacOS/delta thread show --format full ksQQH9ApWKbqSVOy24cbEDlr05LNsrWSAcQQhG3eRXMDSBKZ2xnydif2Vg
```

**Notes**

Read-only source comparison. No thread replay, current-file reads or child-session import.
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
python3 scripts/delta-chat-fixture.py .local/delta-chat-reference.json catalog/fixtures/delta-chat/reference.json
```

**Notes**

38 sanitized records, 65,180 buffer characters, 30 successful tools. Device paths/user names normalized and byte anchors recalculated; only captured timestamps retained. Composer/runtime/provider metadata omitted.
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
python3 scripts/delta-chat-fixture.py .local/delta-chat-reference.json .local/delta-chat-regenerated.json && cmp catalog/fixtures/delta-chat/reference.json .local/delta-chat-regenerated.json
```

**Notes**

Projection reproduced byte-for-byte before timestamp-field extension; final reproduction is recorded below.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config .local/chat-vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte
```

**Notes**

Initial optimizer-only workaround used the same plays and axe checks. Subsequent results: 3/7 then 1/7 failing, followed by 8/8 passing with the original config. The original runner first failed before tests on aria-query elementRoles. Temporary config remains ignored.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t KeyboardSelection
```

**Notes**

Exact rapid Escape/Cancel/paste focus regression: 1 failure to 1 passing, 9 skipped. A queued reader-focus return must not steal focus from a newly opened composer. Native selection is separately exercised by the cold browser harness.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte
```

**Notes**

11 stories passed after raw-icon/file-disclosure fixes. The navigation screenshot refinement then exposed one count-color contrast failure and a CSS compatibility warning; final named acceptance is recorded below. Earlier shell refinement failures (10/10), a rune name collision (9/10 plus 10 errors), first-character caret ordering and asynchronous collapse/focus were fixed. A newly optimized icon caused one runner reload; final code reuses the existing optimized public code-view icon and changes no shared test infrastructure.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit catalog/fixtures/delta-chat/adapter.test.ts
```

**Notes**

7 tests: all 30 tools, separate assistant segments, Unicode byte conversion, patch hunks without fabricated coordinates, exact immutable anchors, empty first-line target and one actual conversation turn.
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
node catalog/fixtures/delta-chat/browser.acceptance.mjs
```

**Notes**

Cold ephemeral Storybook plus fresh Chromium passes native pointer/keyboard selection, immutable deletion/undo, first input once, multiline comments, focus return, custom controls, scroll/resize/remount, mobile captured-read pane and turn navigation. Pointer fixture waits for fonts and selects the full visible phrase; exact quote assertions remain. Every owned test server/browser is cleaned up.
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
pnpm check:architecture
```

**Notes**

Only scripts/dev-ui.acceptance.mjs has 311 counted lines over the 300 limit. Reproduced from the committed parent using jj file show and countedLines; it is unchanged. New navigation is split into rail/card owners to keep their own source budgets. Broader audit qualification remains open.
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
pnpm check:ui-ownership
```

**Notes**

Production has no catalog/Community source imports. Existing owned Community components are reused; no new copied Community source or provenance change.
<!-- /run -->

<!-- run -->
### Run 13

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

Extracting shared acceptance plays reduced the story from 468 counted lines to below its 400 limit; extraction left two stray closing-brace attributes. They were removed. The original UI source-budget failures for rail/card and story were resolved by separating owners; no budgets were loosened.
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
pnpm catalog:check
```

**Notes**

Final corrected story extraction: 0 errors, 0 warnings; catalog boundaries pass for 313 modules.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte
```

**Notes**

Before the final extraction, 10/11 passed; synthetic userEvent hover does not set native CSS :hover, so a visibility assertion failed. The story now verifies actual DOM focus and the cold native harness separately verifies real hover. No hover requirement was removed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte
```

**Notes**

Final story extraction and transparent headers: 11/11 interaction/accessibility stories pass. The native rail-to-card correction is validated separately below.
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
node catalog/fixtures/delta-chat/browser.acceptance.mjs
```

**Notes**

Native group/tool/file headers all remained transparent at rest and on real hover. On the narrow layout the turn card disappeared while Playwright moved from the marker to its scroll control. Pointer exit now has a 150ms grace period, cancelled on re-entry and disposed at teardown; keyboard focus remains independent.
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
python3 scripts/delta-chat-fixture.py .local/delta-chat-reference.json .local/delta-chat-regenerated.json && cmp catalog/fixtures/delta-chat/reference.json .local/delta-chat-regenerated.json
```

**Notes**

Final projection including safe captured timestamps reproduces byte-for-byte: 38 records, 65,180 characters.
<!-- /run -->
<!-- /evidence-field -->

