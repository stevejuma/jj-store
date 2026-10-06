---
schema: 1
id: E-M18-010
---

# E-M18-010 details 2

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
jj --no-pager st; jj --no-pager workspace list; read tracked/parent AGENTS, JJ and shadcn-svelte skills, owning canonical UI/API/collaboration chapters and milestone ledger; inspect Community settings and released Design Core primitives
```

**Notes**

Main continued authorized slice truyzyxr; clean preview uqwlnutu. Memory quick lookup gave no task-specific implementation evidence. All tracked edits belong to this slice.
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
Admitted live GET /v1/local/capture/settings, /v1/local/capture and /v1/local/capture/sources?limit=100 with protocol and same-origin Fetch Metadata headers
```

**Notes**

Saved/effective settings are valid. Initial status/source409 capture_control_busy is transient worker coordination; four admitted busy reads precede200 within approximately400ms. Missing admission headers correctly403; no bypass. Artifacts .local/agent-settings-loading/{settings-before,status-before,sources-before,status-admitted}.json.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte (loading before)
```

**Notes**

stories-before.log:14 existing stories pass, two new loading/busy regressions fail before implementation.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/capture-observation-http.test.ts web/tests/vite/capture-freshness.test.ts (before)
```

**Notes**

units-before.log: seven tests pass; four retry/loading regressions fail. Two expected pre-implementation rejected-read diagnostics recorded; after runs have no unhandled errors.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte (loading after)
```

**Notes**

stories-after.log:16 interactions/accessibility stories pass; initial fixture captured-prop warnings subsequently corrected with untrack.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte (harness before)
```

**Notes**

harness-before.log:15 pass. Group/table regressions and updated table-layout expectation fail before implementation.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte (intermediate harness after)
```

**Notes**

harness-after.log:16 pass. Corrected test assumptions about collapse animation and Select button/listbox semantics; no forced interactions or weakened assertions.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte (intermediate verified/accepted)
```

**Notes**

stories-verified.log and stories-accepted.log:17 pass, one fails from pointer-events lock during Select closing animation. Wait for restored pointer interaction before next click; test still proves exact selected import source.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte (final)
```

**Notes**

stories-final.log:18 stories pass. Loading and real refusal notifications, draft retention, keyboard Codex disclosure/avatar, semantic source table, mixed harness filters/stable labels, styled counts, empty filter coverage, qualified import guards and exact selector all pass. Visual-pending remains.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/capture-observation-http.test.ts web/tests/vite/capture-freshness.test.ts web/tests/vite/capture-management.test.ts web/tests/vite/capture-settings.test.ts web/tests/vite/production-capture-management.test.ts (after/verified)
```

**Notes**

units-after.log and units-verified.log:31 tests across five files pass, no unhandled diagnostics. Retries are bounded, GET-only and cancellable; real refusals/unknown commands retain guarded state and stable identity.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check; pnpm catalog:check (initial/intermediate)
```

**Notes**

Initial web check passed; catalog had two fixture prop-capture warnings. Intermediate web had one $state shadowing error; catalog had five Testing Library API errors and one nonreactive fixture warning. Corrected captureState naming, documented role/label APIs, fixture untrack and reactive accepted-command state. Recorded logs retained.
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
pnpm web:check; pnpm catalog:check (accepted)
```

**Notes**

web-check-verified.log and catalog-check-accepted.log: zero errors/warnings; catalog178-module boundaries pass. Production stays independent of catalog. No contract generation required.
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
pnpm check:architecture (initial)
```

**Notes**

architecture.log:407 counted story lines exceed400. Existing layout/loading plays extracted into owned catalog helpers; all behavior and assertions retained.
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
pnpm check:architecture (verified/final); pnpm check:ui-ownership; pnpm check:boundaries; pnpm spec:first; pnpm spec:validate
```

**Notes**

architecture-final.log:912 modules pass. Ownership, package boundaries and protected-source/spec mappings pass; canonical validation has22 chapters167 requirements. Focused formatting uses changed UI/controller/test/doc files; historical ledger never whole-file formatted.
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
pnpm web:build
```

**Notes**

build.log: fresh production bundle and asset manifest pass; existing chunk-size warning retained. No Rust build. Cold tests use existing explicit target/debug/examples/browser_preview on isolated7458, then Playwright stops it.
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
pnpm exec playwright test --config .local/agent-settings-loading/playwright.config.ts local-capture-settings.spec.ts local-capture-loading.spec.ts
```

**Notes**

browser.log: all seven cold tests pass. Real built router verifies delayed initial loads, skeletons and visible disabled controls, two Busy refusals then recovery for each read with no unavailable toast; Codex collapse/draft/global controls; table border/radius, mixed filters/counts/stable exact-selector imports; mobile scroll/no viewport overflow; notification dismissal and pending intent; lifecycle polling and capability gate; Axe passes.
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
jj rebase -r uqwlnutu -d truyzyxr; jj workspace update-stale (clean preview); Cua live DOM and screenshots
```

**Notes**

Preview7470 source refreshed without backend/main-runtime restart. Real local configuration shows global settings plus avatar-bearing Codex CLI/Desktop disclosure. Sources show4 loaded/observed; Cursor filter shows0 rows while retaining total4. No false unavailable toast; no real config/control/import submitted. Screenshot .local/agent-settings-loading/sources-preview.png. Temporary proof tab is closed after inspection; user tab retained.
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
pnpm check:progress; pnpm catalog:check (final); focused pnpm exec prettier --check; final historical evidence-prefix assertions
```

**Notes**

progress.log:136 tasks pass. catalog-check-final-accepted.log: zero errors/warnings,178-module boundaries pass. format-check.log: all21 changed source/test/chapter files pass; append-only ledger excluded from whole-file formatting. All407 prior evidence objects and their raw JSON prefix retained. configuration-preview.png also records collapsed Codex/global separation. Temporary proof tab closed; user tab remains open.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

