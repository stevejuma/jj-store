---
schema: 1
id: E-UI-CI-003
---

# E-UI-CI-003

<!-- evidence-field:id markdown -->
## Id

E-UI-CI-003
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-15
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-016
- JST-UI-019
- JST-API-001
- JST-ARCH-005
- JST-DOC-002
- JST-DOC-003
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change xxopusyq
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Repair the Search save-error/status-refresh race exposed after PR #8 merged. The production Search settings controller and its direct browser, settings-freshness and notification consumers are the impact surface. Run the original browser case first, then the complete failed application-browser shard for neighboring interaction coverage. Rust, storage, native, cold-application and unrelated UI suites are not required locally for this single-controller correction; full hosted PR checks remain required.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .changeset/search-save-notice.md
- docs/maintenance/coverage.json
- docs/site/guides/settings.md
- spec/src/current/contracts/api/24-local-workspace-links.md
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/documentation/03-delivery-and-validation.md
- spec/src/current/contracts/ui/10-local-management-delivery.md
- spec/src/current/progress/M30/index.md
- spec/src/current/progress/M30/history/M30/index.md
- web/src/lib/controller/production-search-settings.ts
- web/tests/browser/local-search-settings.spec.ts
- web/tests/vite/production-search-settings.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/browser/local-search-settings.spec.ts#Search tabs preserve drafts
  and separate Save from Apply
- web/tests/vite/production-search-settings.test.ts#a successful status read
  cannot resolve a failed save with a retained draft
- web/tests/vite/production-search-settings.test.ts#a successful explicit save
  resolves its retained failure
<!-- /evidence-field -->

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
gh run view 38773656817 --repo stevejuma/jj-store --job 116563044700 --log-failed; gh run view 38773928270 --repo stevejuma/jj-store --job 116564439965 --log-failed; gh run download 38773928270 --repo stevejuma/jj-store --name qualification-results-ui-browser-4-1 --dir /tmp/jj-browser-qualification-failure
```

**Notes**

Main standard run and manually dispatched qualification both fail the same Search settings browser case at merged main 3bc66f6d21940480a5c2a9daa7904770b4f7613b: each affected shard has 53 passes/one failure. Trace shows POST /local/search/save returning 409 at 47077.055ms and GET /local/search returning 200 at 47126.717ms; the latter clears the save failure despite the retained private draft. Full qualification correctly blocks all heavy suites at its failed standard gate. These are two failing executions, not two distinct defects.
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts local-search-settings.spec.ts --repeat-each=3
```

**Notes**

Original built application reproduces the missing search settings changed notification locally: one pass, one failure, one not run due the existing maximum-failure setting. /tmp/jj-search-save-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-search-settings.test.ts
```

**Notes**

Initial new tests used generic Error, which the public error formatter intentionally sanitizes. Both diagnostic runs failed; the post-fix failure exposed the unsuitable mock. Replaced only the test error with the actual ApiError(search_settings_changed), preserving the intended public message. /tmp/jj-search-controller-before.log and /tmp/jj-search-controller-after.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-search-settings.test.ts
```

**Notes**

With typed API errors, temporarily load only production-search-settings.ts from the unchanged parent under a finally-restored file backup. Both new regression cases fail because successful reads clear the save error; eight existing cases pass. /tmp/jj-search-controller-baseline.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-search-settings.test.ts
```

**Notes**

All ten tests pass after retaining the draft save failure separately from read state. Successful explicit save and Cancel resolve it; clear resets controller ownership. /tmp/jj-search-controller-fixed.log.
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
pnpm docs:review application "Search save failures remain visible through status refresh until the draft is cancelled or an explicit save succeeds."; JJ_STORE_DOC_REFERENCE=$PWD/.local/build-targets/public-release/debug/examples/docs_reference pnpm web:build
```

**Notes**

Fresh application assets built using retained documentation/preview tools, without a Rust rebuild. Existing chunk-size and Tauri dynamic-import diagnostics remain warnings. /tmp/jj-search-web-build.log.
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts local-search-settings.spec.ts --repeat-each=3
```

**Notes**

All three fresh browser runs pass in 28.9s. The strengthened test also requires the notification after manual Refresh and its removal on Cancel. Existing revision, draft, Save/Apply separation, navigation, geometry and axe assertions remain. /tmp/jj-search-browser-fixed.log.
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
pnpm web:check; pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-search-settings.test.ts web/tests/vite/settings-refresh.test.ts web/tests/vite/notification-service.test.ts; pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-search-settings.test.ts web/tests/vite/settings-freshness.test.ts web/tests/vite/notification-service.test.ts web/tests/vite/repository-settings-restoration.test.ts
```

**Notes**

Web type check zero errors/warnings. First focused filter ran 17 tests across two files (settings-refresh.test.ts was not a file). Correct direct-consumer selection passed all 21 tests across four files, including freshness and repository restoration. /tmp/jj-search-web-check.log, /tmp/jj-search-focused-unit.log, /tmp/jj-search-consumers.log.
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
CI=true JJ_STORE_CI_PREBUILT=1 JJ_STORE_CI_REPORTS=1 PLAYWRIGHT_JUNIT_OUTPUT_FILE=/tmp/jj-search-shard.xml pnpm exec playwright test --config web/playwright.config.ts --shard=2/2
```

**Notes**

The complete originally failing application-browser shard passed: 54 tests, zero failures or skips, 3.0m. Fresh isolated preview process, no retries and original assertions retained. /tmp/jj-search-shard.log and XML.
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
pnpm docs:check
```

**Notes**

Stopped this task-owned invocation (exit 143) after it selected the default managed Cargo docs example and waited on the target lock. No cleanup or other service was performed. Reran with the retained explicit documentation executable.
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
JJ_STORE_DOC_REFERENCE=$PWD/.local/build-targets/public-release/debug/examples/docs_reference pnpm docs:check
```

**Notes**

Nine Mermaid diagrams rendered in both themes, documentation built, all 143 Help files closed with valid links/anchors. /tmp/jj-search-docs-check-prebuilt.log.
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
pnpm check:progress; pnpm spec:first; pnpm spec:validate; pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

Progress: 155 tasks and 820 evidence records. Spec-first: six protected files/four canonical chapters. Spec validation: seven validators, 3900 chapters, 198 requirements. Architecture: 1921 modules; UI ownership passed. Wider Rust, storage, native and full heavy qualification are outside this focused local correction.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Fresh hosted PR verification remains required. The heavy run on merged main
  stopped at standard UI acceptance; previous backend/cold failures remain
  unresolved.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node v24.10.0, pnpm 10.32.1, Playwright 1.63.0, Vitest 4.1.11. Local macOS ARM64 built-browser execution; GitHub failures were Ubuntu x64. Existing registered preview/documentation tools retained.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No public Rust, HTTP, persisted settings or artifact format changes. Save and Apply remain explicit; draftRevision continues to pin the original edit revision. Notification lifetime follows the retained failed draft.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Push the locally verified correction through an ordinary PR and verify hosted UI and required checks. Heavy qualification remains incomplete and requires a fresh run after integration; no publication is authorized.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: "Clarify JST-UI-016/019 recovery semantics: a successful status read
  does not establish save success and cannot clear an unresolved draft save
  failure. JST-API-001 and JST-ARCH-005 remain unchanged; document recovery
  under JST-DOC-002/003."

```
<!-- /evidence-field -->


<!-- generated-navigation -->

## Pages

- [E-UI-CI-003 details 2](details-02.md)
