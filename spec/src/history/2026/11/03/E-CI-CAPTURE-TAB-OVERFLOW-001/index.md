---
schema: 1
id: E-CI-CAPTURE-TAB-OVERFLOW-001
---

# E-CI-CAPTURE-TAB-OVERFLOW-001

<!-- evidence-field:id markdown -->
## Id

E-CI-CAPTURE-TAB-OVERFLOW-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-API-001
- JST-GOV-003
- JST-GOV-007
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/tests/browser/local-capture-settings.spec.ts
- spec/src/current/contracts/api/31-local-capture-configuration-editing.md
- spec/src/current/contracts/architecture/31-ci-execution.md
- spec/src/current/progress/M30/index.md
- spec/src/history/2026/11/03/E-CI-CAPTURE-TAB-OVERFLOW-001/index.md
- spec/src/history/2026/11/03/index.md
- spec/src/history/2026/11/03/SUMMARY.nav.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/browser/local-capture-settings.spec.ts#cold capture controls preserve intent and snapshot
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
gh run view 41714722392 --json databaseId,headSha,status,conclusion,url,workflowName,event,createdAt,updatedAt,jobs
gh run view 41714722555 --json databaseId,headSha,status,conclusion,url,workflowName,event,createdAt,updatedAt,jobs
```

**Notes**

At exact merged source `e1244f1755682e6c0b6ab881c965977973438914`,
combined Pages run 41714722555 passed its build and deploy jobs. Standard run
41714722392 failed one root job: application-browser shard 2/2, job
126196371664. Its `cold capture controls preserve intent and snapshot` case
timed out while `Sources` moved from the visible responsive tab list into
`More`; Playwright repeatedly resolved a tab that detached before becoming
stable. The failed `required` job was only the aggregate gate. Every other root
job passed. Because the standard run failed, no downstream main-image workflow,
manifest or signature is accepted as publication evidence from this source.
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
npx --yes pnpm@10.32.1 web:build
npx --yes pnpm@10.32.1 exec node scripts/browser-fixtures.mjs
```

**Notes**

The production web build completed. Direct host fixture preparation stopped
before browser execution because the shell's `cargo` shim had no configured
toolchain. No test ran and this is not a product failure. Validation moved to
the repository-owned pinned environment instead of changing global Rust state.
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
npx --yes pnpm@10.32.1 exec node scripts/validation.mjs run web:test:browser -- web/tests/browser/local-capture-settings.spec.ts --grep "cold capture controls preserve intent and snapshot" --workers=1
```

**Notes**

The exact protected case passed 1/1 in 9.3 seconds in the pinned validation
environment after the helper bounded its direct-tab click and followed the
existing `More` path when responsive fitting moved the target. Report:
`.local/validation/runs/jj-store-validation-f6a43182-b1ca-4356-a71a-6238e9fd71b0`.
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
npx --yes pnpm@10.32.1 exec node scripts/validation.mjs run web:test:browser -- web/tests/browser/local-capture-settings.spec.ts --grep "cold capture controls preserve intent and snapshot" --repeat-each=10 --workers=1
```

**Notes**

Ten serial repetitions passed 10/10 in 1.5 minutes with the full existing
capture-control, source, import, recovery, screenshot and accessibility
assertions. Report:
`.local/validation/runs/jj-store-validation-ceabf662-2de4-47f8-8a4c-a541b665addf`.
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
npx --yes pnpm@10.32.1 web:check
```

**Notes**

The repository-owned pinned validation lane passed with zero Svelte errors and
zero warnings. Report:
`.local/validation/runs/jj-store-validation-1fcda48e-60ce-4590-922d-996d7494f5f1`.
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
npx --yes pnpm@10.32.1 exec prettier --check web/tests/browser/local-capture-settings.spec.ts spec/src/current/contracts/architecture/31-ci-execution.md spec/src/current/progress/M30/index.md spec/src/history/2026/11/03/E-CI-CAPTURE-TAB-OVERFLOW-001/index.md spec/src/history/2026/11/03/index.md spec/src/history/2026/11/03/SUMMARY.nav.md
npx --yes pnpm@10.32.1 exec spec-validator validate --only governance,summary,fileLimits,verification --json
npx --yes pnpm@10.32.1 exec spec-validator first --base @- --head @ --json
npx --yes pnpm@10.32.1 check:progress
```

**Notes**

Formatting, the four selected document validators, spec-first and the progress
ledger passed for the focused test, canonical review and same-change evidence.
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

<!-- evidence-field:versions markdown -->
## Versions

Node.js 26.8.2; pnpm 10.32.1; Playwright 1.63.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No persisted, API, manifest, container or release format changed.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

The change only hardens capture browser-test navigation across responsive tab
fitting. The helper first preserves an ordinary user click with a bounded wait,
then follows the already tested `More` menu when the target moved there. A
direct DOM activation is retained only when no overflow control exists and a
visible direct tab was replaced during Playwright actionability checks. Product
source, capture semantics, responsive presentation and publication workflows
are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Land the focused repair after protected checks, rerun the exact-main standard
gate, and only then evaluate the source-bound main image publication.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: The test-only navigation helper keeps the existing responsive tab and capture contracts; it removes a Playwright actionability race without changing product behavior or release policy.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress because standard run 41714722392 failed, so the main-image trigger and all tagged, visual, quarantine, latency, provider, platform and deferred publication gates remain unqualified.
```
<!-- /evidence-field -->

