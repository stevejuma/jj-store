---
schema: 1
id: E-M17-070
---

# E-M17-070 details 3

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
pnpm exec prettier --write web/tests/browser/local-capture-settings.spec.ts catalog/fixtures/{capture-management.ts,CaptureSettingsFixture.svelte} web/src/lib/controller/capture-types.ts
```

**Notes**

Production-browser and controller type formatting.
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
pnpm exec prettier --write web/tests/vite/production-capture-management.test.ts
```

**Notes**

Installation composition regressions formatted.
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
pnpm exec prettier --write web/src/lib/controller/{capture-validation,production-capture-management,production-store-controller.svelte}.ts web/tests/vite/capture-management.test.ts catalog/fixtures/{capture-management.ts,CaptureSettingsFixture.svelte} web/src/lib/ui/store-settings/StoreCaptureSettings.svelte
```

**Notes**

Final response semantics, fixture and composition formatting.
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
pnpm exec prettier --write web/tests/vite/{capture-management,production-capture-management}.test.ts web/tests/browser/local-capture-settings.spec.ts
```

**Notes**

Final exact regression formatting.
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
pnpm exec prettier --write spec/src/{api,verification}.md
```

**Notes**

Changed-document formatting.
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
pnpm check:progress
```

**Notes**

Final same-change ledger validation; all 324 historical evidence objects and their raw JSON prefix preserved.
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
pnpm exec prettier --check catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/fixtures/CaptureSettingsFixture.svelte catalog/fixtures/capture-management.ts spec/src/api.md spec/src/collaboration.md spec/src/milestones.md spec/src/ui.md spec/src/verification.md web/src/lib/controller/capture-errors.ts web/src/lib/controller/capture-http.ts web/src/lib/controller/capture-management.svelte.ts web/src/lib/controller/capture-types.ts web/src/lib/controller/capture-validation.ts web/src/lib/controller/contracts.ts web/src/lib/controller/production-capture-management.ts web/src/lib/controller/production-store-controller.svelte.ts web/src/lib/controller/production-store-lifecycle.svelte.ts web/src/lib/controller/production-store-state.svelte.ts web/src/lib/controller/production-store-view.ts web/src/lib/ui/store-settings/StoreCaptureDiagnostics.svelte web/src/lib/ui/store-settings/StoreCaptureImport.svelte web/src/lib/ui/store-settings/StoreCaptureSettings.svelte web/src/lib/ui/store-settings/StoreCaptureSources.svelte web/src/lib/ui/store-settings/StoreServiceSettings.svelte web/tests/browser/local-capture-settings.spec.ts web/tests/vite/capture-management.test.ts web/tests/vite/production-capture-management.test.ts
```

**Notes**

Missing blank line before the ledger JSON fence after formatting its top section separately. Corrected without modifying historical evidence; final changed-file check passes.
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
pnpm exec prettier --check catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/fixtures/CaptureSettingsFixture.svelte catalog/fixtures/capture-management.ts spec/src/api.md spec/src/collaboration.md spec/src/milestones.md spec/src/ui.md spec/src/verification.md web/src/lib/controller/capture-errors.ts web/src/lib/controller/capture-http.ts web/src/lib/controller/capture-management.svelte.ts web/src/lib/controller/capture-types.ts web/src/lib/controller/capture-validation.ts web/src/lib/controller/contracts.ts web/src/lib/controller/production-capture-management.ts web/src/lib/controller/production-store-controller.svelte.ts web/src/lib/controller/production-store-lifecycle.svelte.ts web/src/lib/controller/production-store-state.svelte.ts web/src/lib/controller/production-store-view.ts web/src/lib/ui/store-settings/StoreCaptureDiagnostics.svelte web/src/lib/ui/store-settings/StoreCaptureImport.svelte web/src/lib/ui/store-settings/StoreCaptureSettings.svelte web/src/lib/ui/store-settings/StoreCaptureSources.svelte web/src/lib/ui/store-settings/StoreServiceSettings.svelte web/tests/browser/local-capture-settings.spec.ts web/tests/vite/capture-management.test.ts web/tests/vite/production-capture-management.test.ts
```

**Notes**

Final explicit changed-file formatting check, including the appended ledger.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node 26.9.0; Vitest 4.1.11; Storybook 10.5.7; pinned Playwright Chromium and released Design Core; macOS ARM64; synthetic Codex CLI 0.156.1 coverage metadata.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing version-1 installation capture HTTP contracts and exact uint64 revisions; no wire, repository, spool or journal format change. Installation state is browser-memory owned; original request IDs remain copyable and previous import IDs explicitly inspectable.
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

Browser management slice only: bootstrap-gated LocalTrusted installation transport, explicit status/source refresh, pause/resume/rescan and inactive history import, unknown-intent reconciliation, bounded runtime source paging and distinct acceptance/publication/tail presentation. Final selected execution set is 13 unit/direct-consumer tests, six interaction/axe stories and two cold built-browser tests. API responses in browser acceptance are controlled fixtures; no real model edit or ingestion/storage claim is made by this UI lane. E-M17-069 owns configured real-storage startup evidence. The installation controller survives repository disconnect and settings navigation, aborts at teardown and does not refresh displayed repository or AgentStore roots. All historical ledger objects/raw prefix preserved. Stories remain visual-pending; no baseline approved. Root aggregates, Rust builds/tests, contract regeneration and unrelated suites are out of scope under the UI-only targeted policy. M17/M17.2 stay In progress: remaining readers are next, followed by M17.3-M17.7. Full goal remains active.
<!-- /evidence-field -->

