---
schema: 1
id: E-M21-018
---

# E-M21-018 details 3

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
pnpm exec prettier --write docs/site/technical/lifecycle.md
```

**Notes**

stack-local-preparation-lifecycle-format.log: Owning server lifecycle manual formatting.
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
pnpm exec prettier --write docs/maintenance/coverage.json
```

**Notes**

stack-local-preparation-coverage-format.log: Local/server source review acknowledgements formatted.
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
pnpm exec prettier --check spec/src/architecture.md spec/src/documentation.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/verification.md docs/implementation-plan.md docs/site/technical/local.md docs/site/technical/lifecycle.md docs/maintenance/coverage.json
```

**Notes**

stack-local-preparation-docs-format-check.log: Changed document formatting passes; ledger excluded.
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
pnpm docs:review local 'Documented saved candidate archive retention through the foreground lease and recovery witness, with exact original-key replay after source loss and no source restoration or publication. Server pending journal selection precedes retention and acknowledged commit/digest replaces only the pending archive body; runtime authority, extras/index repair, publication and checkout remain required.'
```

**Notes**

stack-local-preparation-docs-review-local.log: owning source acknowledgement records the exact pending-retention join and remaining runtime gates.
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
pnpm docs:review configuration 'Reviewed exact local reword preparation composition, principal/request/intent lookup before live context, trusted combination selection, pending archive selection before retention, exact commit/archive acknowledgement, 128-record/8-MiB capacity and immutable legacy encoding. Cancellation, process loss and source loss resume original previews without reprepare. Installed authority, command transport, publication and guarded checkout remain unfinished.'
```

**Notes**

stack-local-preparation-docs-review-server.log: owning source acknowledgement records the exact pending-retention join and remaining runtime gates.
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
Five repetitions of cargo test --locked -p jj-store-server --test stack_local_preparation admission::authorization_stale_physical_selection_and_conflicting_identity_do_not_prepare -- --exact
```

**Notes**

stack-local-preparation-admission-repeat.log: five independent exact reruns pass with diagnostics, investigating the initial uncaptured admission failure; no assertion weakened.
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

stack-local-preparation-docs-check.log: owning source review, nine offline diagrams, strict manual build and 139-file link/anchor closure pass.
<!-- /run -->

<!-- run -->
### Run 8

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

stack-local-preparation-spec-first.log: missing cli.md mapping for changed server code; owning server command semantics added before final rerun.
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
pnpm spec:validate
```

**Notes**

stack-local-preparation-spec-validate.log: five validators, 23 chapters and 173 requirements pass.
<!-- /run -->

<!-- run -->
### Run 10

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

stack-local-preparation-progress.log: five evidence references lacked required symbol anchors; corrected to existing exact tests without altering product or historic entries.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/cli.md
```

**Notes**

stack-local-preparation-cli-spec-format.log: owning server-command chapter formatted; historical ledger excluded.
<!-- /run -->

<!-- run -->
### Run 12

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

stack-local-preparation-spec-first-final.log: 20 protected files and six canonical chapters pass after owning CLI/server chapter update.
<!-- /run -->

<!-- run -->
### Run 13

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

stack-local-preparation-progress-final.log: 142 tasks pass bookkeeping; M21.2 remains In progress, result pending and historical evidence unchanged.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Stage 3 still requires real registered-device command/combination authority,
  lost extras/index recovery, exact operation-head publication preserving
  concurrent siblings, guarded checkout and durable recovery, and
  prepare/apply/status/discovery transport. Current preparation is a trusted
  composition seam, not installed authorization.
- Real candidates, previews and original retention keys are qualified through
  fixture authority. Physical fingerprints and combination selection remain
  synthetic fixtures; no real installed ACL/scanner/durable combination
  acceptance inferred.
- "Stages 4-7 remain approved and required: durable merge/WIP combinations and
  apply/unapply/switching, whole-file then server-issued hunk transfers, graph
  actions/deletion policy/guarded undo, linked-native real-storage execution and
  complete cold production UI wiring."
- Two inherited service strict-lint diagnostics remain unresolved/unwaived from
  E-M21-010. Owning server/JJ no-deps lint does not qualify aggregate
  service/dependency lint.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Complete runtime registered-device authority, source-extra/index recovery, exact publication and guarded checkout; continue all seven approved stages before enabling writable controls.
<!-- /evidence-field -->

<!-- evidence-field:out_of_scope markdown -->
## Out of scope

No HTTP/schema/client/startup/production UI or Storybook changes, fresh visual review, screenshot comparisons/baselines, publication, installed binaries/services, existing-repository reset/migration, native/real-RustFS execution, retained-ref cleanup or unrelated root aggregate lanes.
<!-- /evidence-field -->

<!-- evidence-field:summary markdown -->
## Summary

111 unique direct tests pass (86 JJ source cases; 13 journal integrations; four journal units; eight real preparation integrations). Actual previews survive source loss/GC, process loss before/after retention, cancellation and malformed-evidence rejection. Five exact admission repetitions pass; the initial uncaptured failure is retained in evidence. Runtime authority, publication, guarded checkout and all later approved stages remain incomplete.
<!-- /evidence-field -->

