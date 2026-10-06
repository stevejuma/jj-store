---
schema: 1
id: E-M18-009
---

# E-M18-009 details 2

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
jj --no-pager st; read tracked/parent guidance, JJ skill, canonical UI/API/collaboration chapters and milestones; inspect Community Markdown presentation and Design Core Button/highlight tokens
```

**Notes**

Main change xrpomkus continues the authorized XML slice. Preview is a clean descendant. All implementation belongs to main. No unrelated changes, server resets or installed executables touched.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentPromptMarkup.stories.svelte (initial compact regression)
```

**Notes**

before.log: six existing stories pass; compact/highlighting story fails once before implementation.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentPromptMarkup.stories.svelte (initial after)
```

**Notes**

stories.log: fragmented token spans changed accessible names for attribute-bearing triggers. Four pass; fixed by explicit aria-label preserving the exact original opening tag.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentPromptMarkup.stories.svelte (accessible-name correction)
```

**Notes**

stories-corrected.log: all seven interaction/accessibility stories pass before subsequent indentation/border/attribute requests.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config .local/agent-xml-compact/playwright.config.ts agent-prompt-markup.spec.ts (initial cold build)
```

**Notes**

browser.log: desktop/mobile fail Axe contrast, tag name #22863a on inherited inline-code background #f4f5f7, ratio4.24. Inert/malformed scenario passes. Copy, Markdown, keyboard and virtualization assertions pass before Axe.
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
pnpm check:architecture (first)
```

**Notes**

architecture.log: preceding slice left AgentToolGroups story at413 counted lines after its late icon assertions, exceeding400. Historical E-M18-008 remains unchanged. Extracted existing duration play function into owned catalog helper; behavior unchanged.
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
pnpm spec:first (first)
```

**Notes**

spec-first.log: UI change requires mapped api.md alongside ui.md. Updated mapped API paragraph without transport/schema changes; corrected run passes.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentPromptMarkup.stories.svelte (indentation regression)
```

**Notes**

indent-before.log: six pass, new hierarchy assertion fails. Live before measurement independently confirms parent label1056px, compact child1046px, disclosed child1067px; only10px body padding and1px row border.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentPromptMarkup.stories.svelte catalog/components/store-application/AgentToolGroups.stories.svelte (first indentation after)
```

**Notes**

stories-final.log:12 pass, compact geometry and attribute contrast fail. Exact tool family all seven passes. Neutral attribute palette changed to distinct constant token; mix toward foreground fixes lighter story surface contrast.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentPromptMarkup.stories.svelte (subsequent indentation checks)
```

**Notes**

Each of stories-indent-final.log, stories-verified.log and stories-diagnostic.log records one compact geometry failure and six passes. Diagnostics show parent label586.1875px versus child346px: catalog lacked justify-start utility output. Replaced utility-dependent trigger layout with explicit owned CSS and retained16px geometry assertion; temporary diagnostic logging removed.
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
pnpm exec playwright test --config .local/agent-xml-compact/playwright.config.ts agent-prompt-markup.spec.ts (first hierarchy build)
```

**Notes**

browser-latest.log: desktop observes15px due transparent1px Button border; mobile asynchronous separate bounding-box reads encounter remount. Removed trigger border and made geometry read atomic within current subtree; assertions remain16px plus aligned sibling labels.
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
pnpm exec playwright test --config .local/agent-xml-compact/playwright.config.ts agent-prompt-markup.spec.ts (first borderless build)
```

**Notes**

browser-verified.log: functional/copy/geometry checks pass; Axe rejects tag color on highlighted message surface (#e5e5e6 ratio3.67) and mobile hover surface. Names mix20% toward foreground, attributes15%; no global palette change or weakened accessibility checks.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/prompt-elements.test.ts
```

**Notes**

units.log and units-final.log: eight parser/token units pass. Generic arbitrary/Unicode names, inert attributes, exact character reconstruction, malformed syntax, nesting/code/reference boundaries preserved. No added dependency.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentPromptMarkup.stories.svelte (final)
```

**Notes**

stories-accepted.log: all seven interactions and Axe checks pass. Compact children and deeper leaf labels indent16px per level, align with disclosed siblings, keep closing tags/inline Markdown without buttons, retain &lt;=4px sibling gaps and borderless rows; distinct attribute/string/name colors. Nested and multiline disclosures remain keyboard accessible.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentToolGroups.stories.svelte
```

**Notes**

tool-stories.log and stories-final.log: exact seven direct tool stories pass after duration play helper extraction, preserving prior icon padding and timing behavior. No broader tool changes.
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
pnpm web:check; pnpm catalog:check (initial/final)
```

**Notes**

Both affected type checks pass with zero errors/warnings; latest catalog176-module boundary. web-check-verified.log and catalog-check-verified.log. No Rust source, schema or contract generation changed.
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
pnpm web:build (initial/final)
```

**Notes**

All builds pass, including build-accepted.log. Existing chunk-size warning retained. Production asset manifest generated; no Rust binary rebuild. Required cold browser verification consumes final built UI via existing explicit browser_preview executable on isolated7458.
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
pnpm exec playwright test --config .local/agent-xml-compact/playwright.config.ts agent-prompt-markup.spec.ts (final cold build)
```

**Notes**

browser-contrast.log and final browser-accepted.log: three scenarios pass, covering desktop activity, touch-mobile recorded chunks and inert/malformed sections. Real built-asset router, exact clipboard source, safe links/code, keyboard expand/collapse, virtual-row resizing, no overlap/viewport overflow, child indentation and Axe all qualify this surface. Fresh isolated service stopped by Playwright.
<!-- /run -->
<!-- /evidence-field -->

