---
schema: 1
id: E-M20-008
---

# E-M20-008

<!-- evidence-field:id markdown -->
## Id

E-M20-008
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M20
- M20.1
- M20.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change wvopppny
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

ui-alignment-review-and-backend-proposal
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1, Node 26.9.0, pnpm 10.32.1; reviewed UI parent 687f056b; default checkout d1a25261; live catalog on literal loopback port 6057
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Documentation only. No API, RPC, native, companion, checkout, UI controller or persisted format changes.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- docs/implementation-plan.md
- spec/src/architecture.md
- spec/src/ui.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

UI workspace clean at wvopppny/7e24214b on 687f056b. M20.2 Deferred. UI has no production adapter; default local capabilities still disable code mutations. Original two-phase boundary remains authoritative.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Review current Storybook and owning UI/API/local-JJ contracts against the original UI-first request; compare current default read/freshness foundations and Changeyard/JJButler semantics. Record a proposed bounded-read, durable-execution, merge/WIP-combination and production-adapter sequence. Preserve all 441 historical evidence objects byte-for-byte. No backend authorization inferred.
<!-- /evidence-field -->

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
jj --no-pager st; jj --no-pager log -n 3
```

**Notes**

Clean isolated UI workspace; no unrelated changes.
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
pnpm exec prettier --write docs/implementation-plan.md spec/src/architecture.md spec/src/ui.md
```

**Notes**

Formatted only changed planning/specification prose; ledger not formatted.
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
pnpm exec prettier --check docs/implementation-plan.md spec/src/architecture.md spec/src/ui.md
```

**Notes**

.local/stack-backend-plan-evidence/format.log: Changed Markdown passes; historical ledger excluded from formatter.
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
pnpm spec:first
```

**Notes**

.local/stack-backend-plan-evidence/spec-first.log: 1 protected file, 3 canonical chapters pass.
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
pnpm spec:validate
```

**Notes**

.local/stack-backend-plan-evidence/spec-validate.log: 5 validators, 23 chapters, 173 requirements pass.
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

.local/stack-backend-plan-evidence/progress.log: 141 tasks; evidence bookkeeping passes.
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
python3 .local/stack-backend-plan-evidence/verify-ledger.py
```

**Notes**

441 historical evidence objects remain byte-identical; M20.2 remains Deferred.
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
pnpm exec prettier --write docs/implementation-plan.md
```

**Notes**

Formatted final preview/admission clarifications.
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
pnpm docs:check
```

**Notes**

Intentionally stopped this invocation and its child processes after discovering it rebuilds the unrelated Rust CLI docs_reference example before the documentation site. No diagnostic/product test failure was observed; full docs build is unverified and outside this planning surface. Focused Markdown, owning specs and progress checks pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:review markdown -->
## Review

In-app browser tab 15: inspected Workspace/app shell, applied/local branch groups, feature/search body Apply and conflict preview, cancelled preview, selected main target history, returned to Workspace. No command applied, visual comparison or baseline change. One ambiguous Workspace tab locator was corrected by scoping it to Local repository views.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Backend implementation requires explicit user approval; this proposal does not
  open M20.2.
- Registry foundation qualification and visual baseline approval remain
  separate; not performed in this review.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the proposed delivery order and approve backend scope explicitly before implementation.
<!-- /evidence-field -->

<!-- evidence-field:out_of_scope markdown -->
## Out of scope

Backend/Rust tests, UI interaction/a11y suite reruns, root aggregates, backend execution, package publication and screenshot baselines. Full docs:check was attempted but cancelled when it entered Rust CLI reference rebuilding; no full docs build pass is claimed.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


