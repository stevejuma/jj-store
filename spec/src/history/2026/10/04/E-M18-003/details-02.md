---
schema: 1
id: E-M18-003
---

# E-M18-003 details 2

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
jj --no-pager st; jj --no-pager diff; jj --no-pager log
```

**Notes**

Preview workspace only; no unrelated edits; original main preserved.
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
cargo test -p jj-store-server --test capture_management --locked coverage::harness_coverage_is_bounded_cached_and_reports_configured_roots -- --exact (before)
```

**Notes**

Expected missing configured_roots regression: null instead of exact configured paths.
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
cargo test -p jj-store-server --test capture_management --locked coverage::harness_coverage_is_bounded_cached_and_reports_configured_roots -- --exact (after)
```

**Notes**

1 test; exact CLI/Desktop paths, paused-directory rename and no GET directory I/O; transcript file secrecy retained.
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
cargo test -p jj-store-server --lib --locked configured_roots_retain_default_home_override_and_explicit_selection (first)
```

**Notes**

New fixture used a different pending capacity than configured; corrected fixture to its existing spool capacity.
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
cargo test -p jj-store-server --lib --locked configured_roots_retain_default_home_override_and_explicit_selection (corrected)
```

**Notes**

1 test; default HOME, CODEX_HOME override and explicit roots; no directory creation.
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
cargo test -p jj-store-server --lib --locked capture_runtime
```

**Notes**

4 control/source cached-observation tests passed.
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
cargo test -p jj-store-http --lib capture_management_contract_is_installation_local_and_actor_free --locked
```

**Notes**

1 schema test;16 directory roots and4096 character path bound; actor-free contract retained.
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
pnpm contracts:generate; pnpm contracts:check
```

**Notes**

Generated OpenAPI/TypeScript match;221 actual HTTP/SSE responses conform;7 tooling tests and TypeScript contract consumer passed. Complete contract inventory required by changed public response and fixture registration.
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
cargo check -p jj-store-service -p jj-store-http -p jj-store-server --locked
```

**Notes**

Affected Rust package checks passed.
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
cargo test -p jj-store-server --test capture_management --locked
```

**Notes**

13 passed; import-inspection setup incorrectly unwrapped a cold Unknown acknowledgement.
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
cargo test -p jj-store-server --test capture_management --locked imports::inspection::missing_reader_link_and_unqualified_publication_withhold_completion_without_repair -- --exact (retry before correction)
```

**Notes**

Same Unknown acknowledgement reproduced without changing product code. Fixed setup to observe original request and require exact completed receipt; never resend with a new identity.
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
pnpm web:check (initial and final); pnpm catalog:check; pnpm web:build
```

**Notes**

Web/catalog0 errors/warnings;166 catalog modules; production bundle and asset manifest passed; existing chunk warning retained.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/components/store-application/SettingsNavigation.stories.svelte (first tabs run)
```

**Notes**

Top-level tab sections retained h3 headings; Axe heading-order failures. Corrected Sources/Imports headings to h2.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/components/store-application/SettingsNavigation.stories.svelte (after)
```

**Notes**

14 passed:10 Capture/4 Navigation. Actual root paths, tabs, retained drafts, exact pending retries across tabs, action geometry and accessibility.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/capture-management.test.ts web/tests/vite/capture-freshness.test.ts
```

**Notes**

17 passed; root validation and observation lifecycle.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/capture-management.test.ts (after helper extraction)
```

**Notes**

11 passed; bounded absolute unique paths/counts and absent metadata remain accepted explicitly.
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
pnpm exec playwright test --config .local/integration/playwright-settings.config.ts local-capture-settings.spec.ts (first tabs run)
```

**Notes**

Settled-opacity wait targeted source Import action after explicit import opened Imports. Retargeted Refresh import status;2 other tests passed.
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
pnpm exec playwright test --config .local/integration/playwright-settings.config.ts local-capture-settings.spec.ts (corrected)
```

**Notes**

3 cold built-bundle tests; desktop/mobile Axe0; aligned/stacked rows and no overflow; actual root fixture, draft retention, polling across tabs and teardown. Existing compiled disposable browser host; no substituted product runtime evidence.
<!-- /run -->
<!-- /evidence-field -->

