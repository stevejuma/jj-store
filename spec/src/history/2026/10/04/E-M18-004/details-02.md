---
schema: 1
id: E-M18-004
---

# E-M18-004 details 2

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
jj rebase capture stack onto eb25eacb; targeted JJ status/log/diff and spec-conflict resolution
```

**Notes**

Rebased nlstmlwq, zmyqspmo and bookmarked kyvxzyzu variant onto clean main. Combined API, governance and verification requirements; preserved all 315 main and 395 capture evidence entries verbatim, 399 unique entries. Main M4.31 and JST-UI-019 retained. Old capture tip bookmarked separately.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 11
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte (before dependency refresh)
```

**Notes**

11 failed plus 11 errors from old cached Design Core NotificationToasts API; refreshed existing temporary local validation dependency.
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
pnpm install --offline --frozen-lockfile --force
```

**Notes**

Existing local pnpmfile hook checksum differs from lockfile; no tracked dependency edits.
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
pnpm install --offline --lockfile=false --ignore-scripts
```

**Notes**

Refreshed existing documented temporary foundation source-validation setup; package manifests/lockfile unchanged, no publication.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte (before implementation, refreshed dependency)
```

**Notes**

10 passed, 1 failed; new scoped-notification regression could not find Capture request unavailable dismissal.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/capture-freshness.test.ts (before)
```

**Notes**

6 passed, 1 failed; complete successful observations did not clear the capture error.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/capture-freshness.test.ts web/tests/vite/capture-management.test.ts web/tests/vite/production-capture-management.test.ts web/tests/vite/notification-service.test.ts
```

**Notes**

27 passed; complete observed recovery, partial failure retention, uncertain original commands, producer transport and scoped notification lifecycle.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/components/store-application/StoreSettingsNavigation.stories.svelte catalog/components/store-application/StoreNotifications.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte
```

**Notes**

Incorrect project name; corrected to storybook and actual SettingsNavigation filename.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/components/store-application/StoreSettingsNavigation.stories.svelte catalog/components/store-application/StoreNotifications.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte
```

**Notes**

25 passed in three existing selected files; nonexistent navigation filename was not selected. Includes all 11 capture stories with portal feedback, no inline duplication, dismissal across refresh/remount and recovery.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/SettingsNavigation.stories.svelte
```

**Notes**

4 passed; actual navigation family separately selected, total 29 direct stories qualified.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-store-bootstrap.test.ts web/tests/vite/production-store-settings.test.ts web/tests/vite/production-store-events.test.ts
```

**Notes**

13 direct controller integration tests passed; total 40 unit tests.
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
pnpm web:check; pnpm catalog:check; pnpm web:build
```

**Notes**

Web/catalog 0 errors and warnings; 169 catalog modules; production asset manifest passes. Existing bundle-size and timing warnings retained.
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
pnpm exec playwright test --config .local/integration/playwright-settings.config.ts local-capture-settings.spec.ts notifications.spec.ts
```

**Notes**

5 cold built-bundle tests passed in fresh disposable browser_preview service: capture controls, capability gating, scoped dismissal/recovery/drafts, observation teardown and generic toasts. Desktop/mobile Axe zero violations and notification geometry within viewport.
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
pnpm check:architecture; pnpm check:boundaries; pnpm check:ui-ownership
```

**Notes**

883 UI modules; 17 package boundaries; ownership validator passed.
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
pnpm exec prettier --write changed UI/tests/specs; pnpm exec prettier --check changed files excluding historical milestones ledger (initial)
```

**Notes**

Only merged verification paragraph formatting flagged; formatted it without changing semantics.
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
node --input-type=module (owned preview Vite-only refresh via existing process helpers)
```

**Notes**

Preview had cached the pre-refresh Design Core API and was blank on first reload. Ownership-checked restart of only Vite cleared its cache; backend PID, fixture, captured transcripts and historical repository route preserved.
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
CUA tab 2: reload; profile > Settings > Agents; Refresh capture; save .local/rebase-notifications/agents-notifications-live.png
```

**Notes**

Live four-tab Agents page loads, configured /Users/stevejuma/.codex/sessions root visible; real intermittent capture busy error appears once as a dismissible toast, with no inline error duplication. No capture mutation.
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
jj diff --from codex/capture-before-notification-rebase --to @ --name-only; evidence-preservation script
```

**Notes**

No Rust source, public schema, generated contract, persistence or capture runtime changes. All 399 historical evidence objects preserved byte-for-byte.
<!-- /run -->
<!-- /evidence-field -->

