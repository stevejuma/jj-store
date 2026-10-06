---
schema: 1
id: E-M21-016
---

# E-M21-016 details 3

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
pnpm exec prettier --check spec/src/architecture.md spec/src/documentation.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/verification.md docs/implementation-plan.md docs/site/technical/local.md docs/maintenance/coverage.json
```

**Notes**

stack-archive-format-check.log and stack-archive-format-check-complete.log: explicit changed docs pass; ledger excluded.
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
pnpm docs:check
```

**Notes**

stack-archive-docs-check.log: nine offline light/dark Mermaid diagrams, strict manual build and 139-file link/anchor closure pass. Final source/manual check is recorded after completion, separately from this earlier archive-only manual run.
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
pnpm docs:check
```

**Notes**

stack-archive-docs-check-complete.log: final complete source/manual review, nine offline diagrams, strict build and 139-file link/anchor closure pass.
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

stack-archive-spec-first.log: 21 protected files and five owning canonical chapters pass.
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

stack-archive-spec-validate.log: five validators, 23 chapters and 173 requirements pass.
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

stack-archive-progress.log: 142 tasks pass bookkeeping; remaining approved gates remain explicit and evidence result pending.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Stage 3 still requires authenticated registered-device command policy and
  original-request lookup, exact intent/effect/journal/retainer composition,
  source operation/extra restoration, exact head publication preserving
  concurrent siblings, safe guarded checkout and durable fault/process-loss
  recovery, and prepare/apply/status/discovery transport. Private
  materialization is not source restoration or publication; no writable
  capability enabled.
- Retention tests still use isolated fixture access and synthetic
  request/intent/prepared keys, not an actual server prepared plan. The
  foreground metadata lock does not guard external editor bytes.
- Complete active-combination Unstaged and watcher/lifecycle acceptance remain
  dependent on durable combination state. Approved Stages 4-7 still require
  merge/WIP combinations and membership/switching, exact whole-file then
  server-issued hunk transfers, graph actions and guarded undo, linked-native
  real-storage and complete cold UI wiring.
- Two inherited service strict-lint diagnostics from E-M21-010 remain unresolved
  and unwaived. Owning JJ no-deps strict lint does not qualify aggregate lint.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Compose real preparation/history retention with authenticated server policy and durable exact journal; qualify source restoration, exact publication and guarded checkout recovery before enabling combinations and writable UI.
<!-- /evidence-field -->

<!-- evidence-field:out_of_scope markdown -->
## Out of scope

No HTTP/client generation or production UI/Storybook/startup changes; no fresh visual review, screenshot comparison or baseline updates. No package publication, installed service/binary replacement, existing-repository reset/migration, automatic retained-ref cleanup, native/real-RustFS execution or unrelated root aggregate lanes. All seven approved stages remain required.
<!-- /evidence-field -->

<!-- evidence-field:summary markdown -->
## Summary

Complete operation/history/code recovery input now survives source metadata loss and GC, with exact private reconstruction and immutable replay. Real reword rejects lost originals before import and returns full candidate ancestry. M21.2 and complete backend/UI implementation remain In progress.
<!-- /evidence-field -->

