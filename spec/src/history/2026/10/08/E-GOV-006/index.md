---
schema: 1
id: E-GOV-006
---

# E-GOV-006

<!-- evidence-field:id markdown -->
## Id

E-GOV-006
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- Bootstrap
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-GOV-005
- JST-GOV-007
- JST-GOV-011
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-08
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Complete three previously unverified task browser audit cases
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change syprklnw above startup repair 73346908 and original audit 6f323d19
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

E-GOV-005 left two automatic-updates cases and one task-activity-layout case unverified because the original audit wrapper omitted LocalTrusted admission headers. This addendum preserves E-GOV-005 byte-for-byte and closes only those three execution gaps.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

A disposable native embedded-RustFS fixture on loopback port 7490, with a real project, task and initial comment, ran all three unchanged browser tests. Both automatic-updates cases pass. task comments use main Activity with a persistent bottom composer fails at task-activity-layout.spec.ts:78: composer y=706 is above the expected content bottom 738.03125. The complete three-case fixture invocation reproduces the same failure, with two passing cases. No product source, assertion, visual baseline or user development state was changed. Owned processes and temporary state were cleaned after each attempt.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/current/progress/Bootstrap/index.md
- spec/src/current/progress/Bootstrap/history/Bootstrap/index.md
- spec/src/history/2026/10/08/E-GOV-006/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/real/automatic-updates.spec.ts#saved and external task comments
  appear automatically while drafts survive
- web/tests/real/automatic-updates.spec.ts#open task forms keep their original
  version guards after incoming metadata
- web/tests/real/task-activity-layout.spec.ts#task comments use main Activity
  with a persistent bottom composer
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
node .local/test-audit-2026-10-08/task-browser-fixture.mjs (initial lightweight wrapper)
```

**Notes**

The wrapper added Content-Type application/json to a bodyless GET and received 400 from bootstrap; no repository tests executed. Corrected the wrapper to set that header only for POST. Fixture cleaned. Log: task-browser-completion.log.
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
node .local/test-audit-2026-10-08/task-browser-fixture.mjs
```

**Notes**

2 passed, 1 failed, 0 skipped. Wrapper creates isolated native fixture using scripts/dev-ui-native-fixture.mjs, uses admitted local requests, creates a project/task/comment and invokes pnpm exec playwright test --config web/playwright.real.config.ts automatic-updates.spec.ts task-activity-layout.spec.ts --output .local/test-audit-2026-10-08/task-browser-results with explicit JJ_STORE_REAL_BASE_URL, JJ_STORE_REAL_REPOSITORY, JJ_STORE_REAL_PROJECT and JJ_STORE_REAL_TASK. Exact failure: task-activity-layout.spec.ts:78. Log: task-browser-completion-recheck.log.
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
node .local/test-audit-2026-10-08/task-browser-fixture.mjs (fresh fixture confirmation)
```

**Notes**

Same three-case sequence repeated to retain the preceding native task mutations when confirming the layout assertion. Same two passes and one failure at line 78; expected &gt;=738.03125, received 706. Log: task-browser-confirm.log.
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
pnpm exec prettier --write spec/src/current/progress/Bootstrap/index.md spec/src/current/progress/Bootstrap/history/Bootstrap/index.md
pnpm check:progress
pnpm spec:validate
```

**Notes**

Progress passed with 146 tasks and 663 evidence records. Specification validation passed with 7 validators, 3211 chapters and 181 requirements. No owned test listener remains on 7480, 7482 or 7490. The user's preserved server on 7460 still returns 200 with local_agent_execution=true and runtime ready, retaining one setup and two sessions.
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
pnpm spec:first
```

**Notes**

The unchanged-contract confirmation gate rejects a new evidence record unless its entire result passes with no remaining gaps. This audit deliberately records a reproduced failing product test, so it cannot qualify as a passing refactor confirmation. No protected implementation is changed. The failure is retained without relabeling the audit as pass or changing the contract. Log: .local/test-audit-2026-10-08/addendum-first.log.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

fail
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- The task Activity/composer layout assertion remains failing. The mobile, later
  keyboard and Axe steps in that test were not reached.
- All other reproduced failures and external/ignored qualification prerequisites
  remain as recorded in E-GOV-005; this addendum is not a full-suite rerun or
  green repository claim.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Node 24.10.0; pnpm 10.32.1; Playwright 1.63.0; current built jj-store/native JJ and embedded RustFS.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing LocalTrusted format-2 isolated fixture; no schema, authority, publication, saved-agent state or user fixture changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Previously skipped three-case browser audit gap only, explicitly authorized by the original request for all repository tests. Broader suites are out of scope for this evidence-only addendum; they remain documented in E-GOV-005.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Triage the Activity/composer geometry regression with its UI owner and rerun the exact case. Retain other audit failures and deferred prerequisites.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-GOV-005/JST-GOV-007/JST-GOV-011 evidence-only audit completion,
  running unchanged assertions against disposable native state. No protected
  product behavior or test contract changed.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: Bootstrap and all product milestone statuses and gate selections remain
  unchanged. Two passing browser cases do not close the failing layout case or
  other audit gates.

```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


