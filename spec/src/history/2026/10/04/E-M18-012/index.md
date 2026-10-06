---
schema: 1
id: E-M18-012
---

# E-M18-012

<!-- evidence-field:id markdown -->
## Id

E-M18-012
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M18.8
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change mpsxrmoz
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Compact source repository navigation, retained-refresh footer spinner and fair bounded coverage observations
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Svelte5.57.1; Storybook10.5.7; Vitest4.1.11; Rust1.98.1/macOS ARM64; released Design Core Table/Spinner; existing browser_preview and development RustFS executables.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

UI first-cell repository links and3-column colspan only; fair one-second acquisition of existing management mutex for status/source/receipt reads. Commands, held physical journals, owned blocking cancellation, authorization, wire schemas, native roots and source formats unchanged. Browser observation freshness remains accepted-read time.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreCaptureSettings.stories.svelte
- catalog/components/store-application/capture-source-table-play.ts
- catalog/fixtures/CaptureSettingsFixture.svelte
- crates/jj-store-server/src/capture_management/imports.rs
- crates/jj-store-server/src/capture_management/observation.rs
- crates/jj-store-server/src/capture_management/sources.rs
- crates/jj-store-server/src/capture_management/tests/observations.rs
- crates/jj-store-server/src/capture_management/tests.rs
- crates/jj-store-server/src/capture_management.rs
- docs/capture-preview.md
- spec/src/api.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/ui.md
- web/src/lib/ui/store-settings/StoreAgentSettings.svelte
- web/src/lib/ui/store-settings/StoreCaptureSettings.svelte
- web/src/lib/ui/store-settings/StoreCaptureSourceFooter.svelte
- web/src/lib/ui/store-settings/StoreCaptureSourceIdentity.svelte
- web/src/lib/ui/store-settings/StoreCaptureSources.svelte
- web/src/lib/ui/store-settings/StoreSettings.svelte
- web/tests/browser/local-capture-loading.spec.ts
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/src/capture_management/tests/observations.rs#cached_observations_queue_before_later_import_work
- crates/jj-store-server/src/capture_management/tests/observations.rs#queued_observation_rechecks_current_authority
- crates/jj-store-server/src/capture_management/tests/observations.rs#timed_out_or_cancelled_observation_releases_its_queue_place
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
jj --no-pager st; read canonical UI/API/CLI/collaboration and milestone chapters; inspect Community then released Design Core Table/Spinner; CDP live capture request timings
```

**Notes**

Clean main mpsxrmoz after4eff7363. Sources are worker watch-channel catalogs, not request-triggered transcript scans. Persisted private source identities/cursors are separate from reconstructed runtime coverage. Live settings reads8-10ms; status/sources repeatedly409 Busy with150/300/450/600ms backoff and5s fallback polling.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
cargo test --locked -p jj-store-server --lib capture_management::tests::observations (before)
```

**Notes**

rust-before.log: all3 new queue, authority and cancellation regressions fail because reads try-lock immediately.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte (before)
```

**Notes**

stories-before.log:18 pass/2 fail; separate repository header persists and refreshing footer has no spinner.
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
cargo test --locked -p jj-store-server --lib capture_management::tests::observations (after and final); cargo check --locked -p jj-store-server
```

**Notes**

rust-regression.log and rust-regression-final.log:3 pass. FIFO tests cover status/source/receipt reads preceding later import waiters, current authority after queue admission, one-second Busy and cancellation queue cleanup. rust-check.log passes. Existing linker unwind warning retained.
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
cargo test --locked -p jj-store-server --test capture_management source_pages_
```

**Notes**

rust-http-sources.log:2 pass for bounded cached pages and current actor/configuration/installation/physical-journal withholding. No wire/schema/generated response change.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte (final)
```

**Notes**

stories-accepted.log:21 pass including interaction/accessibility. First-cell icon repository link preserves callback identity; no repository column;3-column footer; spinner retains rows and clears on ready; unsaved Agent draft blocks navigation without clearing the draft.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/capture-freshness.test.ts web/tests/vite/capture-management.test.ts web/tests/vite/capture-settings.test.ts web/tests/vite/capture-observation-http.test.ts web/tests/vite/production-capture-management.test.ts
```

**Notes**

units.log:33 pass across5 direct capture files, preserving pending request identity, accepted source freshness and cancellation.
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
pnpm exec playwright test --config .local/capture-source-table/playwright.config.ts local-capture-settings.spec.ts local-capture-loading.spec.ts (initial)
```

**Notes**

browser.log:7 pass/1 fail. New navigation fixture omitted repository from bootstrap, so the expected named repository was absent. Added explicit linked bootstrap only for the navigation scenario; retained actual route and authorization assertions.
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
pnpm exec playwright test --config .local/capture-source-table/playwright.config.ts local-capture-loading.spec.ts -g "cold source refresh retains coverage"; pnpm exec playwright test --config .local/capture-source-table/playwright.config.ts local-capture-settings.spec.ts local-capture-loading.spec.ts (final)
```

**Notes**

browser-regression.log:1 pass; browser-final.log:8 pass cold. Actual production Settings repository link closes Settings and selects authorized files page; gated source refresh retains rows and accepted datetime, shows footer spinner and clears it. Existing loading/busy, drafts, exact imports, notification dismissal, polling lifetime, desktop/mobile/Axe and narrow containment remain qualified. Isolated7458 service stopped; no Rust rebuild for browser fixture.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm check:boundaries; pnpm check:ui-ownership; focused Prettier and rustfmt --check
```

**Notes**

web-check-final.log/catalog-check.log: zero errors/warnings,179-module catalog boundary. architecture-final.log:917 modules; boundaries.log:17 packages; ownership.log and format-check.log pass. Initial redundant link role warning and rustfmt module-order diagnostics corrected. Ledger never whole-file formatted.
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
pnpm web:build (final); pnpm spec:first; pnpm spec:validate; pnpm dev:backend:build
```

**Notes**

build-final.log passes with existing chunk-size warning. spec-first.log:17 protected files/4 owning chapters; spec-validate.log:22 chapters/167 requirements. backend-build.log passes with existing linker warning; checkout-only CLI/server binaries, no installed executable replacement.
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
jj rebase -r uqwlnutu -d mpsxrmoz; jj workspace update-stale; explicit development executable/runtime paths pnpm dev:capture:restart in preview; CDP timing/DOM/screenshot
```

**Notes**

Preview7470/7472 backend restarted using main target/debug/jj-store and existing embedded runtime. Vite retained, isolated fixture/history preserved, main service/desktop untouched. Live4 source pages all200 with80/109/137/138ms response latency; whole observed settings/status/source chains about130-200ms. This is warm read evidence, not a cold-import guarantee. Real compact table shows repo links under titles and3-column footer. after.jpg and live-timings.json saved; temporary proof tab closed and user tab retained. No real Import/Pause/Rescan/config save submitted.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress; pnpm exec prettier --check docs/capture-preview.md spec/src/api.md spec/src/cli.md spec/src/collaboration.md spec/src/ui.md (final after plan-note restoration)
```

**Notes**

17 protected files/5 mapped chapters,22 chapters/167 requirements and136 tasks validated. Focused documentation formatting passes. All409 prior evidence objects and raw JSON prefix preserved; only E-M18-012/E-M18-013 updated. Extra broad diagnostic failures remain explicit; root aggregates, other harness/release qualification and visual baseline approval remain outside this narrow acceptance.
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

Narrow source-table/coverage-read slice only. No milestone promotion, new reader, historical evidence rewrite, wire generation, installed binary/service/main-runtime changes, real import/config command, reset, publication or visual baseline approval. Additional server-library survey failures are recorded independently in E-M18-013; this entry does not claim a full Rust-suite pass. Cold sanitizer/import preparation remains explicitly Deferred in the agreed preview plan.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Use compact Sources in preview7470; retain focused queue/navigation/spinner coverage. Broader cold sanitizer and timed import qualification stays in E-M18-013 and the deferred roadmap.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


