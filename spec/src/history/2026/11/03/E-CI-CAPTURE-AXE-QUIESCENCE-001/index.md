---
schema: 1
id: E-CI-CAPTURE-AXE-QUIESCENCE-001
---

# E-CI-CAPTURE-AXE-QUIESCENCE-001

<!-- evidence-field:id markdown -->
## Id

E-CI-CAPTURE-AXE-QUIESCENCE-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-UI-016
- JST-UI-019
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
- spec/src/history/2026/11/03/E-CI-CAPTURE-AXE-QUIESCENCE-001/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/browser/local-capture-settings.spec.ts#capture notifications retain dismissal and drafts until observed recovery
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
gh workflow run ci-heavy.yml --ref main -f ref=201be1b237e9de61b090e19714250c4f31bc111f
gh api --allow-escape-sequences repos/stevejuma/jj-store/actions/jobs/125945243568/logs
```

**Notes**

Complete qualification run 41642914060 failed application-browser shard 2/2
in the protected capture-notification case. Axe observed the disabled outline
button during its opacity transition: foreground `#7e8184` on white measured
3.91:1 against the required 4.5:1. The same run's other UI acceptance lanes,
including all Storybook shards, passed.
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
node scripts/validation.mjs run web:test:browser -- web/tests/browser/local-capture-settings.spec.ts --grep "capture notifications retain dismissal and drafts until observed recovery" --workers=1
```

**Notes**

The exact cold Docker browser case passed 1/1 in 5.7 seconds after the test
waited for the final refresh to re-enable and for capture traffic to remain
quiet across the established transition-settle window. Report:
`.local/validation/runs/jj-store-validation-7e520182-9d8f-415b-b207-e800f32be4f8`.
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
node scripts/validation.mjs run web:test:browser -- web/tests/browser/local-capture-settings.spec.ts --grep "capture notifications retain dismissal and drafts until observed recovery" --repeat-each=10 --workers=1
```

**Notes**

Ten serial repetitions passed 10/10 in 55.3 seconds. Each retained the existing
toast, dismissal, draft, mobile geometry and axe assertions after the observed
recovery and repeated-failure episodes. Report:
`.local/validation/runs/jj-store-validation-78f065ab-566d-4b0a-a27d-3e9e13c7c090`.
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
npx --yes pnpm@10.32.1 web:check
```

**Notes**

The direct UI type-check lane passed with zero Svelte errors and zero warnings
in the pinned validation image. No production source or generated contract was
changed.
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
npx --yes pnpm@10.32.1 exec prettier --check web/tests/browser/local-capture-settings.spec.ts spec/src/history/2026/11/03/E-CI-CAPTURE-AXE-QUIESCENCE-001/index.md spec/src/history/2026/11/03/index.md spec/src/history/2026/11/03/SUMMARY.nav.md
node node_modules/@lapismd/spec-validator/dist/cli.js validate --only governance,summary,fileLimits,verification --json
node node_modules/@lapismd/spec-validator/dist/cli.js first --base @- --head @ --json
node scripts/check-progress.mjs
```

**Notes**

Formatting passed. Four selected specification validators checked 4,602
chapters and 211 requirements, spec-first accepted the protected browser test
and evidence pair, and progress accepted 186 tasks and 1,024 evidence records.
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

Node.js 26.8.2; pnpm 10.32.1; Playwright 1.63.0; axe-core 4.13.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No production source, API, wire, cache, generated asset or persisted format
changed.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Application-browser capture-notification acceptance only. The production
notification service, `StoreNotice`, capture requests, drafts and visual styles
are unchanged. The test now observes the same bounded quiescence already used
by adjacent capture refresh cases before sampling settled accessibility state.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Land the focused acceptance repair, then rerun complete integrated
qualification on the merged stack. Visual baseline approval remains separate.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-UI-016 and JST-UI-019 still require accessible operational feedback,
  retained dismissal and preserved drafts. All existing protected assertions
  remain; the browser test only waits until the final request and Design Core
  button transition have settled before the axe snapshot.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. The focused browser and type-check lanes pass,
  while complete merged qualification and independent visual approval retain
  their existing status.
```
<!-- /evidence-field -->
