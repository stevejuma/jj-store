---
schema: 1
id: E-AGENT-RESUME-001
---

# E-AGENT-RESUME-001 details 4

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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-freshness.test.ts -t 'failed publication recapture'
```

**Notes**

Before: a failed recapture retained the row but incorrectly advanced the captured
publication boundary. The scheduled failure-recovery retry then skipped the
authority read. The retained event remained uncommitted. Log:
`.local/agent-resume-recapture-before.log`.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-freshness.test.ts -t 'failed publication recapture'
```

**Notes**

After: the publication boundary advances only after successful adoption; failed
reads retain a reset fence for recovery. One pass, fourteen skipped. Log:
`.local/agent-resume-recapture-after.log`.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-*.test.ts web/tests/vite/inbox-request-lifecycle.test.ts web/tests/vite/local-inbox-threads.test.ts
```

**Notes**

Final shared controller/inbox surface: 22 files and 113 tests pass. Includes the
new retry regression, automatic preparation, exact receipt/selection fences and
event-driven request lifecycle. Log: `.local/agent-resume-final-unit.log`.
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
pnpm web:check
pnpm check:ui-ownership
```

**Notes**

Final UI type check has zero errors/warnings; ownership validation passes.
No further native edits were made after the recorded native qualification.
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
pnpm docs:check
```

**Notes**

Final source review correctly refused stale configuration coverage. Application
coverage was also stale. Updated lifecycle, agent execution and Settings guidance
to describe automatic Send, acknowledged-turn recovery, scoped rejection and
retained history. This failure is repaired below, not waived.
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
pnpm docs:review configuration 'Reviewed acknowledged-turn resume eligibility, scoped preparation rejection and failed-history recovery; lifecycle guidance preserves unknown receipts and prohibits prompt replay.'
pnpm docs:review application 'Documented automatic Send preparation and selected-agent participation, semantic readiness observation and retained transcript recovery; manual actions remain optional and uncertain prompts are never replayed.'
pnpm exec prettier --write docs/maintenance/coverage.json docs/site/technical/lifecycle.md docs/site/guides/settings.md docs/site/guides/agent-execution.md
```

**Notes**

Reviewed only affected coverage records; manual pages now reflect the implemented
behavior. Source fingerprints match the final server and application trees.
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
pnpm docs:check
```

**Notes**

Repaired source review passes. Nine diagrams rendered; strict build and the
141-file documentation closure pass link/anchor validation. Log:
`.local/agent-resume-final-docs-after.log`.
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
pnpm check:progress
pnpm spec:first
pnpm spec:validate
```

**Notes**

Evidence checks before manual edits: 146 tasks, 737 evidence records; 22
protected files and five owning chapters; seven validators, 3503 chapters and
181 requirements.
Both backend and Vite process working directories are the default jj-store
workspace. The staged ACP helper checksum matches the recorded rebuilt binary.
<!-- /run -->

<!-- run -->
### Run 9

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

Manual/source-review edits also require the owning documentation chapter.
Recorded unchanged JST-DOC-001/002/003 delivery gates and the specific manual review
in that chapter; final rerun follows.
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
pnpm spec:first
pnpm check:progress
pnpm spec:validate
```

**Notes**

Final manual-inclusive gate passes: 26 protected files, six owning chapters;
146 tasks and 737 evidence records. Final validation retains seven validators,
3503 chapters and 181 requirements. The unchanged documentation delivery/review
contract and explicit M24 In progress review remain recorded.
<!-- /run -->
<!-- /evidence-field -->
