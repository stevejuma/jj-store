---
schema: 1
id: E-M21-017
---

# E-M21-017 details 3

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

stack-source-recovery-docs-format-check.log: explicit changed documents pass; historical ledger excluded.
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
cargo test --locked -p jj-store-jj --test stack_source_recovery
```

**Notes**

stack-source-recovery-final.log: all nine exact source cases pass after final strict-lint repair. Process helper is executed through its passing parent; no assertion weakened.
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
cargo check --locked -p jj-store-jj --all-targets
```

**Notes**

stack-source-recovery-check-final.log: final all-target source/tests check passes after fixture and lint repairs.
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
cargo test --locked -p jj-store-cli --test local_browser
```

**Notes**

stack-source-recovery-consumer.log: all 28 direct local reader/registration/current-observation consumers pass; two existing external-large-repo cases remain intentionally ignored. Existing macOS linker unwind-size warnings recorded; no aggregate service lint or linked-native qualification inferred.
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
pnpm docs:check
```

**Notes**

stack-source-recovery-docs-check.log: final local source review, nine offline diagrams, strict manual build and 139-file link/anchor closure pass.
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
pnpm spec:first
```

**Notes**

stack-source-recovery-spec-first.log: 17 protected files and five canonical chapters pass.
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
pnpm spec:validate
```

**Notes**

stack-source-recovery-spec-validate.log: five validators, 23 chapters and 173 requirements pass.
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
```

**Notes**

stack-source-recovery-progress.log: 142 tasks pass evidence bookkeeping with M21.2 In progress, pending result and all remaining approved gates preserved.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Stage 3 still requires authenticated registered-device command policy and
  original-request lookup, durable storage/join of the recovery witness with
  exact intent/effect/journal/retainer state, lost Git-extra and derived-index
  recovery, exact operation-head publication preserving concurrent siblings,
  guarded physical checkout and complete durable fault/process-loss recovery,
  and prepare/apply/status/discovery transport. This provider requires trusted
  composition to authorize the original request and grants no runtime writable
  capability.
- Retention/recovery tests use real unpublished reword histories but isolated
  fixture access and synthetic request/intent/prepared keys, not a server
  prepared plan. Source recovery leaves heads and checkout unchanged; its
  metadata witness is not an external-editor byte lock.
- Complete active-combination Unstaged and watcher/lifecycle acceptance require
  durable combination state. Approved Stages 4-7 still require merge/WIP
  combinations and membership/switching, exact whole-file then server-issued
  hunk transfers, graph actions and guarded undo, linked-native real-storage
  execution and complete cold production UI wiring.
- Two inherited service strict-lint diagnostics from E-M21-010 remain unresolved
  and unwaived. Owning JJ no-deps lint does not qualify aggregate
  service/dependency lint.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Compose original authenticated preparation/journal/retention and saved recovery witness, then qualify extras/index recovery, exact head publication and guarded checkout; continue all seven approved stages before enabling writable controls.
<!-- /evidence-field -->

<!-- evidence-field:out_of_scope markdown -->
## Out of scope

No public transport/client generation, production UI/Storybook/startup changes, fresh visual review, screenshot comparison or visual baselines. No package publication, installed binary/service replacement, existing-repository reset/migration, native/real-RustFS execution, retained-ref cleanup or unrelated root aggregate lanes.
<!-- /evidence-field -->

<!-- evidence-field:summary markdown -->
## Summary

114 unique direct tests pass (29 owning units, 57 local-JJ integrations, 28 CLI consumers), including nine explicit source recovery cases and real partial process-loss retry. Exact immutable op/view restoration leaves heads, index, extras, companion and physical checkout unchanged. Original journal/policy composition, publication and guarded checkout remain required; M21.2 stays In progress.
<!-- /evidence-field -->

