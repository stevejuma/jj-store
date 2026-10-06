---
schema: 1
id: E-M21-014
---

# E-M21-014 details 3

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
pnpm spec:first
```

**Notes**

stack-foreground-spec-first.log: 12 protected files and five owning canonical chapters pass.
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
pnpm spec:validate
```

**Notes**

stack-foreground-spec-validate.log: five validators, 23 chapters and 173 requirements pass.
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
pnpm check:progress
```

**Notes**

stack-foreground-progress.log and stack-foreground-progress-final.log: 142 tasks pass bookkeeping after final evidence, with M21.2 In progress and result pending for remaining approved work.
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
cargo clippy --locked -p jj-store-jj --all-targets --no-deps -- -D warnings
```

**Notes**

stack-foreground-lint-final.log: final strict owning all-target lint passes after the additional missing-state/editor-byte regression.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- "Stage 3 remains incomplete: real command preparation and retained source
  operation ancestry, exact guarded head publication, safe physical checkout and
  durable recovery, server journal/retainer/coordinator composition, runtime
  request/status/discovery and boundary fault acceptance. A foreground lease is
  metadata coordination, not proof against external editor bytes; no command
  capability enabled."
- Stage 2 complete active-combination Unstaged and watcher/lifecycle acceptance
  remain dependent on durable combination state. Approved Stages 4-7 still
  require merge/WIP combinations, exact file then issued-hunk transfers, graph
  actions and guarded undo, linked-native real-storage and complete cold UI
  flows.
- Two inherited service strict-lint diagnostics from E-M21-010 remain unresolved
  and unwaived; owning JJ no-deps lint is not aggregate lint qualification.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Compose the foreground lease with actual unpublished command preparation, retained exact code and private journal transitions, then prove operation-head publication and guarded checkout recovery before enabling combinations or writable UI.
<!-- /evidence-field -->

<!-- evidence-field:out_of_scope markdown -->
## Out of scope

No UI, Storybook, startup, contract generation or installed service/binary changes. No package publication, real RustFS/native execution, existing repo reset/migration, screenshot comparison/baseline updates or fresh visual review. Root aggregates and unrelated packages/consumers are out of scope under targeted policy.
<!-- /evidence-field -->

<!-- evidence-field:summary markdown -->
## Summary

Qualified nonblocking foreground coordination with real stock JJ and process-loss/cancellation/identity evidence. Complete backend/UI integration remains In progress.
<!-- /evidence-field -->

