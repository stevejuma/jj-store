---
schema: 1
id: E-UI-CI-002
---

# E-UI-CI-002

<!-- evidence-field:id markdown -->
## Id

E-UI-CI-002
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

- JST-API-001
- JST-GOV-003
- JST-GOV-004
- JST-GOV-011
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change qrvzoqsz
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Local correction of the capture receipt recovery browser test after hosted run 38757087128. This evidence qualifies only the test correction and its complete five-test capture file, using the existing built application. The hosted aggregate is failed at this revision and requires a fresh PR run. No production implementation, shared fixture, workflow or test inventory changes; unrelated Rust, Storybook, Help and heavy suites are outside this follow-up.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/tests/browser/local-capture-settings.spec.ts
- spec/src/current/contracts/api/29-installation-capture-source-coverage-and-import.md
- spec/src/current/progress/M30/index.md
- spec/src/current/progress/M30/history/M30/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/browser/local-capture-settings.spec.ts#cold capture controls
  preserve intent and snapshot
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
gh run view 38757087128 --repo stevejuma/jj-store --json status,conclusion,jobs
```

**Notes**

Completed hosted run at ec5075c657fd32fb37e16cc521243e76b64dacf7: three Storybook shards, application browser shard 1/2, static catalog, Help Chromium/WebKit and all standard checks passed. Application browser shard 2/2 had 53 passing cases and one failure: inspections remained 2 while the test expected an unrequested subsequent receipt observation within five seconds. The aggregate required check correctly failed. Artifact qualification-results-ui-browser-4-1 retains Playwright XML, trace and DOM evidence; full qualification was not scheduled.
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts local-capture-settings.spec.ts --grep 'cold capture controls preserve intent and snapshot'
```

**Notes**

Before correction, the exact case passed locally once in 7.1s; this does not negate the hosted failure. /tmp/jj-ui-capture-before.log.
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts local-capture-settings.spec.ts --grep 'cold capture controls preserve intent and snapshot' --repeat-each=3
```

**Notes**

After explicitly clicking the retained Refresh import status control, all three fresh runs passed in 21.3s. The fixture emits no receipt-change event; recovery no longer relies on an incidental background observation or a disappearing unknown-outcome control. Exact request identity, no duplicate import, completed receipt, desktop/mobile geometry and axe assertions remain. /tmp/jj-ui-capture-after.log.
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts local-capture-settings.spec.ts
```

**Notes**

All five capture browser tests passed with a fresh preview process in 15.9s: controls and unknown outcomes, capability gating, notification dismissal and drafts, event-driven refresh and closure, configuration restart feedback. /tmp/jj-ui-capture-file.log.
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
pnpm check:progress; pnpm spec:first; pnpm spec:validate; pnpm exec prettier --check web/tests/browser/local-capture-settings.spec.ts spec/src/current/contracts/api/29-installation-capture-source-coverage-and-import.md spec/src/current/progress/M30/index.md spec/src/current/progress/M30/history/M30/index.md
```

**Notes**

Progress: 155 tasks/819 evidence records. Spec-first: one protected file/one owning chapter. Specification: seven validators, 3898 chapters, 198 requirements. Changed files formatted. Main administrator enforcement and strict required check confirmed enabled; PR #8 remains open.
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

Local Node v24.10.0, pnpm 10.32.1, Playwright 1.63.0. Existing final application build and task-owned browser_preview reused; every Playwright invocation starts an isolated fresh preview process.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No production API, stored format, test counts or public contract changes. Receipt observation retains the original request and never replays the import command.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Push the focused test correction to PR #8 and verify all hosted UI jobs plus required. Full qualification remains idle; E-CI-SUITES-MEASURE-001 backend and cold-application failures still block release. This local pass is not a hosted aggregate or release qualification pass.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-API-001 keeps explicit receipt inspection and exact unknown command
  custody. Correct only the test trigger after the missing-receipt state;
  preserve request identity, single import, completed boundary and accessibility
  assertions. JST-GOV-003/004/011 retain original evidence and narrow local
  proof.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 stays In progress. Local capture-file proof does not promote failed
  hosted aggregation, unresolved heavy suites, publication or visual baselines.

```
<!-- /evidence-field -->

