---
schema: 1
id: E-PROJECT-AGENTS-001
---

# E-PROJECT-AGENTS-001 details 4

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
pnpm exec spec-validator validate --only governance,summary,fileLimits
```

**Notes**

Initial document check catches 401 lines in the interactive-session chapter and
431 in formatted evidence. Chapter prose compacted by one line and evidence
split at field/run boundaries. /tmp/jj-project-agents-docs.log.
<!-- /run -->

<!-- run -->

### Run 2

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown
```

```shell
pnpm check:progress
```

**Notes**

Only five inherited E-NPM-REGISTRY-001 errors: deleted-hook file bookkeeping,
prose file/test references, invalid run result and overall result. Same errors
reproduced on the preserved package-adoption parent; no new slice error.
/tmp/jj-project-agents-progress.log; /tmp/jj-unavailable-parent-progress.log.
<!-- /run -->

<!-- run -->

### Run 3

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown
```

```shell
pnpm spec:first
```

**Notes**

Same inherited E-NPM-REGISTRY-001 diagnostics prevent trustworthy change-set
discovery. /tmp/jj-project-agents-spec-first.log;
/tmp/jj-unavailable-parent-spec-first.log. Parent evidence preserved.
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

Node 26.9.0; pnpm 10.32.1; JJ 0.45.1; Design Core 0.3.1; Svelte 5; Storybook
10.5.7; Vitest 4.1.11.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->

## Formats

Owned Svelte avatar/collection presentation and TypeScript selection helper;
unchanged AgentStatus/session bindings and recorded AgentStore API schemas;
Markdown evidence schema 1.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->

## Next

Retain M11.4 native/cold browser and visual gates, and M24
adapter/permission/cancellation/platform gates. Existing E-NPM-REGISTRY-001
evidence bookkeeping needs correction by its owner before repository-wide
progress/spec-first can pass.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->

## Contract review

```yaml
status: changed
reason:
  "JST-UI-015 installation-hub and interactive-session chapters now require
  repository-grouped avatar rows and omit duplicate Online badges. JST-API-001
  explicitly unchanged in the execution chapter: presentation adds no
  permission, transport generation, captured read, command, publication or
  identity change."
```

<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->

## Milestone review

```yaml
status: unchanged
reason:
  M11.4 and M24 remain In progress. Five unit tests and 18 distinct focused
  Storybook consumer tests qualify this grouping/presentation slice; they do not
  close native/cold runtime, adapter, live permission/cancellation or desktop
  distribution gates. Stories remain visual-pending and baseline approval is
  separate.
```

<!-- /evidence-field -->

<!-- evidence-field:excluded_scope markdown -->

## Excluded scope

Root aggregate, full Storybook, Rust, builds, release, installed service
restart, reset and visual baseline updates are out of scope under the UI-only
targeted policy. Existing package-adoption evidence failures remain separate.
<!-- /evidence-field -->

<!-- evidence-field:document_checks markdown -->

## Document checks

After the chapter was compacted to 400 physical lines and evidence split into
four bounded pages, pnpm exec spec-validator validate --only
governance,summary,fileLimits passes (3 validators, 3572 chapters, 181
requirements). Targeted pnpm exec prettier --ignore-path /dev/null --check with
explicit changed paths passes. Final pnpm check:progress and pnpm spec:first
still report only the same five inherited E-NPM-REGISTRY-001 diagnostics; no
task-specific coverage or requirement error remains. Logs:
/tmp/jj-project-agents-docs-fixed.log, /tmp/jj-project-agents-format.log,
/tmp/jj-project-agents-progress-final.log and
/tmp/jj-project-agents-spec-first-final.log. No aggregate or Rust lane was run.
<!-- /evidence-field -->
