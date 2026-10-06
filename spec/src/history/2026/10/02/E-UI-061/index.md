---
schema: 1
id: E-UI-061
---

# E-UI-061

<!-- evidence-field:id markdown -->
## Id

E-UI-061
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M16
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

appearance-circular-accent-selection-ring
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change xvyrtsyt
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Clean Appearance workspace xvyrtsyt above svtuyuxl/3e788232; main unrelated work preserved. User reports an oval Neutral selection ring. Live geometry reproduces a 34x24 ring inside its 36x36 button, centered but stretched horizontally. Selected span set height only. Community settings inspected first, then Design Core styling and owned Appearance source/provenance; owning UI/API and M16 reviewed.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/store-appearance/appearance.css
- catalog/components/store-appearance/StoreAppearance.stories.svelte
- spec/src/ui.md
- spec/src/api.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

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
jj --no-pager st; jj describe -m "Keep Appearance accent selection rings circular"; scoped source/spec/reference inspection; jj --no-pager diff --stat
```

**Notes**

Reused clean Appearance workspace with live Vite HMR. No foundation or Community source/dependency changes. Exploratory lookups of spec/verification-map.json and scripts/spec-first.mjs were absent; repository package scripts and canonical verification.md supplied the actual gate/ledger mapping.
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
node /tmp/jj-accent-ring-probe.mjs before
```

**Notes**

Exact live Neutral regression fails: ring34x24, button36x36, center offsets0. No runtime errors. /tmp/jj-accent-ring-before.json/png.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/StoreAppearance.stories.svelte -t AllControlsAndLocalLinks (before CSS fix)
```

**Notes**

New rendered geometry regression reproduces10px aspect mismatch.1 failed story,3 unrelated stories skipped. /tmp/jj-accent-ring-story-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/StoreAppearance.stories.svelte -t AllControlsAndLocalLinks (after CSS fix)
```

**Notes**

1 focused story passes with existing interaction/accessibility guards;3 unrelated stories skipped. Blue, Neutral and Green rings are square/centered; selection changes, saved Green accent and all existing controls retained. /tmp/jj-accent-ring-story-after.log.
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
node /tmp/jj-accent-ring-probe.mjs after
```

**Notes**

All10 accent buttons select exclusively and render centered24x24 rings inside36x36 swatches. Actual hover retains circular geometry and centering; keyboard focus remains visible; light/dark desktop1351x988 and mobile390x844 pass. Settled scoped WCAG axe:0 violations;0 runtime errors. Isolated browser context closed; native fixture and user device preferences unchanged. Desktop/mobile screenshots inspected. /tmp/jj-accent-ring-after.json/png and after-mobile.png.
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
pnpm web:check; pnpm catalog:check; pnpm check:ui-ownership
```

**Notes**

Web/catalog report0 errors and0 warnings;129 catalog modules. Ownership validates21 chapters/147 requirements. /tmp/jj-accent-ring-{web-check,catalog-check,ownership}.log.
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
pnpm spec:first (initial)
```

**Notes**

SPEC-FIRST-MISSING mapped api.md chapter. Added browser accent presentation contract without changing API or preference formats; corrected gate is rerun below.
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
pnpm exec prettier --write web/src/lib/ui/store-appearance/appearance.css catalog/components/store-appearance/StoreAppearance.stories.svelte spec/src/ui.md spec/src/api.md spec/src/verification.md spec/src/milestones.md; pnpm exec prettier --check (same files); pnpm spec:first; pnpm spec:validate; pnpm check:progress (including final ledger update)
```

**Notes**

Focused formatting passes. Corrected spec-first passes2 protected files/3 mapped chapters; canonical validation passes5 validators/21 chapters/147 requirements; progress bookkeeping passes120 tasks. Same-change evidence and verification mapping updated with M16 Partial retained. /tmp/jj-accent-ring-{format,format-check,spec-first,spec-validate,progress}.log.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI-only private Appearance accent geometry and its directly affected story/browser presentation. Source-owned buttons use zero padding and centered grid; selected rings set equal width/height. Rust builds/tests, backend restarts, root aggregates, unrelated stories, native writes, fixture resets, publication, production built-CSP qualification and visual baseline approval are out of scope. M16 remains Partial with existing source-release, registry-only, inherited-suite/composer-CSP and visual gates separate. Vite HMR applies this fix without rebuilding the stack.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node26.9.0; pnpm10.32.1; Vitest4.1.11. Existing Svelte, Vite, Storybook, Playwright and foundation dependencies unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native/API/storage/preference changes. jj-store.appearance.v1 and the10 named accent values are retained. Swatch and ring color tokens, selection callbacks, hover scale and independent focus-visible outline unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue UI iteration on the running Appearance Vite workspace; main landing and visual baseline approval remain separate user actions.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

/tmp/jj-accent-ring-*.log/json/png and isolated live probe script. Persistent development fixture preserved.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


