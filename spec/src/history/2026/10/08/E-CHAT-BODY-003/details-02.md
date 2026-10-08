---
schema: 1
id: E-CHAT-BODY-003
---

# E-CHAT-BODY-003 details 2

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
jj --no-pager st
```

**Notes**

Clean working change before this review; existing prototype commits retained.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cua.getApp(/Applications/Delta.app)
```

**Notes**

Native app inspection timed out at 15 seconds and reset the kernel. No retry or app-state mutation.
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
Chronicle availability check
```

**Notes**

Screen recorder could not be verified and was not used. No screen recording or historical memory retrieval.
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
cua.getTab(reference URL, browser iab)
```

**Notes**

Browser security policy rejected direct inspection of the existing 6048 preview. No workaround or alternate browser was used for visual inspection. Independent product-owned tests run separately against disposable catalog fixtures; they are functional evidence, not live comparison.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit catalog/fixtures/delta-chat/patch-presentation.test.ts
```

**Notes**

Exact before regression: 0 passed, 1 failed; triple-prefix content was excluded from captured totals. Output logged in .local/chat-review-unit-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit catalog/fixtures/delta-chat/patch-presentation.test.ts catalog/fixtures/delta-chat/adapter.test.ts catalog/fixtures/delta-chat/middle-fold.test.ts
```

**Notes**

After repair and final explicit +175/-31 reference assertion: 13/13 pass. Covers header/content classification, Unicode offsets, per-file resets, source fixture identity and existing folds/anchors. Logged in chat-review-unit-after.log and chat-review-unit-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte
```

**Notes**

Initial and final affected-family runs both pass 14/14 interaction/accessibility stories. Fresh headless fixture checks verify immutable comments, file disclosures, read pane/focus, navigation, XML/reactions and multiline next draft/fixed footer. Not a manual visit or visual baseline comparison.
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
pnpm web:check
```

**Notes**

Initial type check rejected unsupported aria-description on public Button/Collapsible attributes. Replaced it with supported aria-describedby and a unique hidden status element; no dependency patch.
<!-- /run -->

<!-- run -->
### Run 9

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

Same two unsupported attributes in the shared production component, with zero warnings.
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
pnpm web:check
```

**Notes**

Final 0 errors, 0 warnings; chat-review-web-final.log.
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
pnpm catalog:check
```

**Notes**

Final 0 errors, 0 warnings; 321 catalog modules pass boundaries.
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
node catalog/fixtures/delta-chat/browser.acceptance.mjs
```

**Notes**

Existing cold automated harness passes with a fresh ephemeral Storybook and headless browser, zero page errors, and cleanup. Covers actual pointer/keyboard input, source immutability, discussions/focus, real CSS hover, custom sections, scroll/resize/remount, mobile reads, middle folds, XML/reactions and multiline draft/fixed footer. No manual screenshot capture or blocked-preview inspection.
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
pnpm check:ui-ownership
```

**Notes**

Owned Community compositions inspected before public Design Core diff tokens. No Community/catalog production dependency or provenance copy; released Mira integration unchanged.
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
pnpm check:architecture
```

**Notes**

Only inherited scripts/dev-ui.acceptance.mjs:311 counted lines over 300. New/changed sources satisfy budgets. Read-only jj file show comparison proves failing source is byte-identical to parent fdf28eca; no baseline weakening or unrelated edits.
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
/Applications/Delta.app/Contents/MacOS/delta thread data ksQQH9ApWKbqSVOy24cbEDlr05LNsrWSAcQQhG3eRXMDSBKZ2xnydif2Vg > .local/delta-chat-review.json
```

**Notes**

Fresh read-only native export retained only under ignored .local.
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
python3 scripts/delta-chat-fixture.py .local/delta-chat-review.json .local/delta-chat-review-projection.json
```

**Notes**

38 records, 65180 characters. Read-only comparison confirms buffer/records/draftStart equality with committed demo; projection remains ignored.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit catalog/fixtures/delta-chat/patch-presentation.test.ts
```

**Notes**

Exact original file command rerun after repair: 3/3 pass, including original failing triple-prefix count and added Unicode/reset coverage.
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

146 tasks, 670 records. Fresh evidence schema/coverage passes; M24 unchanged.
<!-- /run -->
<!-- /evidence-field -->

