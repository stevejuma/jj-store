---
schema: 1
id: E-UI-095
---

# E-UI-095 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config .local/settings-playwright.config.ts web/tests/browser/local-workspaces.spec.ts web/tests/browser/history-query.spec.ts
```

**Notes**

Initial cold run: 1 passed; search disappeared on route refresh because disclosure reset depended on the entire route. Reset now tracks only stable repository/page context. .local/polish-browser.log.
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
pnpm exec playwright test --config .local/settings-playwright.config.ts web/tests/browser/local-workspaces.spec.ts web/tests/browser/history-query.spec.ts
```

**Notes**

3/3 cold flows pass after stable-context fix. .local/polish-browser2.log.
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
pnpm exec playwright test --config .local/settings-playwright.config.ts web/tests/browser/local-workspaces.spec.ts web/tests/browser/history-query.spec.ts
```

**Notes**

Later native completion interaction pressed Enter after option visibility but before selection. Strengthened the precondition to aria-selected=true and retained the exact mutable() final assertion; 2 passed. .local/polish-browser-final.log.
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
pnpm exec playwright test --config .local/settings-playwright.config.ts web/tests/browser/local-workspaces.spec.ts web/tests/browser/history-query.spec.ts
```

**Notes**

Final 3/3 cold production-asset browser flows, fresh strict-CSP native preview on 7458, isolated pinned native/local transport fixtures and axe. Verifies hidden search toggles, retained drafts/applied queries, both grammars, focus border/ring, invalid-query feedback and independent Settings copy/Open navigation. .local/polish-browser-last.log and final .local/polish-browser-palette.log. Follow-up found that the original Settings ancestry selector matched the action span rather than the entire row; the strengthened geometric pointer regression and correction below supersede that copy-hit assertion.
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
pnpm exec prettier --write changed files; pnpm exec prettier --check changed files
```

**Notes**

Initial filename extraction inherited rg line-number configuration; no product failure. Corrected to rg --no-line-number. .local/polish-format.log.
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
pnpm exec prettier --write changed files
```

**Notes**

Corrected filename extraction passed in .local/polish-format2.log; final history CSS/story/UI chapter formatted in .local/history-palette-format.log.
<!-- /run -->

<!-- run -->
### Run 7

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

Mapped API chapter was missing; added canonical read-only copy/navigation and controller-owned query presentation requirements. .local/polish-spec-first.log.
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
Live in-app browser on 127.0.0.1:7460, temporary tab 8
```

**Notes**

Inspected full-row Thread ID/Revision copy targets, matching labels, rename form, full folder control width and 1px input border, JJ Log/Op Log hidden/pressed search, darker header-grey circle and query Field group. Final info icon center offsets are 0px on both pages. Rest search border resolves --ui-input-border-color, focus uses the exact standard --ui-input-focus-ring-color at 50% with a transparent border; no inset accent shadow. Saved .local/history-settings-polish.png. No form save/fixture mutation; temporary tab closed and main HMR left running. Initial geometry/synthetic clicks did not reveal the active-translation bug; subsequent real pointer verification below closes that gap.
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

Final after the icon-centering story change: zero errors/warnings and 164 catalog modules. .local/polish-catalog-palette.log.
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
pnpm exec prettier --write spec/src/milestones.md; pnpm exec prettier --check changed files; pnpm spec:first; pnpm spec:validate; pnpm check:progress
```

**Notes**

39 changed files formatted; spec-first covers 36 protected files and 3 canonical chapters. 5 validators, 22 chapters and 152 requirements pass. Progress bookkeeping passes for125tasks; no milestone promotion. .local/polish-ledger-format.log, .local/polish-format-final.log, .local/polish-spec-first-final.log, .local/polish-spec-validate-final.log and .local/polish-progress-final.log.
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
pnpm exec playwright test --config .local/settings-playwright.config.ts web/tests/browser/threads-alignment.spec.ts
```

**Notes**

New pointer regression first timed out: ancestor contains(store-readonly-row) matched store-readonly-row__actions. Corrected both thread and Settings tests to match the complete class token, preserving the label/value assertions. .local/copy-row-before.log.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config .local/settings-playwright.config.ts web/tests/browser/threads-alignment.spec.ts web/tests/browser/local-workspaces.spec.ts
```

**Notes**

Corrected exact row selectors reproduce both defects: Settings whitespace does not set data-copied; Thread held press loses the copy button under elementFromPoint. The default 1px active translation establishes a new containing block for the stretched pseudo-element. .local/copy-row-before2.log.
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
pnpm exec prettier --write web/src/lib/ui/store-settings/StoreReadOnlyRow.svelte spec/src/ui.md; pnpm web:build
```

**Notes**

Disabled translation only on the composed stretched copy control so its hit area stays anchored to the row. Each successful copy remounts the tick for a 160ms scale animation, disabled for reduced motion. Fresh built assets pass. .local/copy-row-format2.log and .local/copy-row-build.log.
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
pnpm exec playwright test --config .local/settings-playwright.config.ts web/tests/browser/threads-alignment.spec.ts web/tests/browser/local-workspaces.spec.ts
```

**Notes**

Settings regression passes and Thread label/value/independent Open assertions pass; later existing toolbar-border assertion expected 1px while the unchanged StoreThreadHeader in parent90d119e0 explicitly has border-block-end:0. Aligned that stale assertion with the established contract; Thread settings header now separately asserts the newly requested0px. .local/copy-row-after.log.
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
pnpm exec playwright test --config .local/settings-playwright.config.ts web/tests/browser/threads-alignment.spec.ts web/tests/browser/local-workspaces.spec.ts
```

**Notes**

2/2 cold browser flows. Actual mouse-down and release verifies the complete labels and values remain copy hit targets; exact clipboard contents and call count prove Open does not copy. Includes existing scoped-search/reply, rename, Settings draft/mobile and axe flows. .local/copy-row-after2.log.
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
pnpm test:storybook catalog/components/store-application/StoreSettingsIdentifier.stories.svelte catalog/components/store-threads/StoreThreadAlignment.stories.svelte; pnpm web:check
```

**Notes**

7/7 affected copy/Thread interactions and accessibility; web types0errors/0warnings. .local/copy-row-stories.log and .local/copy-row-web.log.
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
Live in-app browser pointer clicks on sample Thread ID and Revision labels
```

**Notes**

Both labels now report data-copied=true; computed SVG animation is copy-confirm and circular tick shown. Screenshot .local/thread-copy-row-polish.png. AX-only click was inconclusive; actual pointer clicks confirmed behavior. Temporary tab9closed; user tab and main HMR remain.
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
pnpm exec prettier --check changed files; pnpm check:architecture; pnpm spec:first; pnpm spec:validate; pnpm check:progress
```

**Notes**

Final follow-up checks pass:40changed files,851architecture modules,37protected files/3canonical chapters,5validators/22chapters/152requirements and125task evidence bookkeeping. .local/copy-row-{format-final,architecture,spec-first,spec-validate,progress}.log.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

