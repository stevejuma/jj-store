---
schema: 1
id: E-COMPUTE-006
---

# E-COMPUTE-006 details 3

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
cp -cR /Users/stevejuma/code/lapismd/jj-store/node_modules /tmp/jj-store-mention-parent/node_modules
```

**Notes**

Copied the identical installed runtime into the disposable parent with copy-on-write, avoiding dependency changes.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentPromptMarkup.stories.svelte
```

**Notes**

Unchanged parent: 9 passed. Broad-run Copy message failure is intermittent and did not reproduce in the isolated current or parent runs; no product fix or weakened assertion applied.
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
pnpm progress:history
```

**Notes**

Regenerated evidence navigation and M25 history; restored unrelated generated formatting changes.
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
pnpm check:progress
```

**Notes**

Initial draft record failed validation: named story/template-test anchors unsupported, run failure counts missing, pending result text invalid, unrelated generator formatting churn. Corrected the record and restored the unrelated files before final verification.
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

Initial draft record failed for the same evidence metadata issues; corrected before final verification.
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

Final evidence: 146 tasks, 665 evidence records validated. M25 remains In progress.
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
pnpm spec:first
```

**Notes**

19 protected files, 4 canonical chapters; protected behavior and manual changes covered.
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
pnpm exec spec-validator validate --only governance,summary,fileLimits,verification
```

**Notes**

4 validators, 3219 chapters, 181 requirements passed.
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
jj workspace forget mention-parent; trash /tmp/jj-store-mention-parent
```

**Notes**

Parent was clean; removed the disposable workspace registration and moved its files to Trash. A forced-remove cleanup command was rejected before execution, so recoverable Trash was used. Forget already retired the empty parent working change; separate abandon reported no revision. Main user state preserved.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Independent M24 live adapter and M25 packaged macOS/Linux gates remain open.
- Existing architecture size violation and earlier E-GOV-005/E-GOV-006 audit
  gaps are not repaired by this UI slice.
- OpenCode Copy message menu failed once in the wider run and passed isolated;
  do not describe this as an unconditional full-suite pass.
- Visual baselines remain unapproved and unchanged.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Node 24.10.0, pnpm 10.32.1, Svelte 5.57.1, Vitest 4.1.11, Storybook 10.5.7, Bits UI 2.19.4. Validation uses the already installed Design Core dependency; the pre-existing pnpm hook/lock resolves a sibling file package. This change does not modify dependencies or qualification claims.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing AgentSettings conditional command and exact request/receipt shape unchanged. Browser audience v1 continues to store only bounded exact IDs; installation preference supersedes its legacy enabled flag. No repository, WAL, schema, credential or ACP format changes.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain M25 In progress; qualify independent distributions and triage the separately recorded broad audit gaps.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-012 and JST-COLLAB-036 now share one immediately persisted
  mention preference across Settings and composers, with scoped retained
  recipient pins and shadcn Settings dropdowns. JST-API-001
  schema/authorization/exact retry semantics unchanged; JST-UI-015 functional
  keyboard/disabled behavior covered. JST-DOC-001/002 manual updated.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M25 remains In progress; M24 real adapter/cold production and M25 signed
  macOS/WKWebView/Linux distribution gates remain independent. UI fixture and
  fresh development validation do not complete them.

```
<!-- /evidence-field -->

