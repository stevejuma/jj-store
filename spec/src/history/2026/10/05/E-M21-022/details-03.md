---
schema: 1
id: E-M21-022
---

# E-M21-022 details 3

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
pnpm exec prettier --write docs/maintenance/coverage.json
```

**Notes**

stack-source-indexes-coverage-format.log: Local source-review acknowledgement formatted.
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
pnpm exec prettier --check spec/src/architecture.md spec/src/documentation.md spec/src/local-jj.md spec/src/jj-integration.md spec/src/verification.md docs/implementation-plan.md docs/site/technical/local.md docs/maintenance/coverage.json
```

**Notes**

stack-source-indexes-doc-format-check.log: Changed canonical/manual documents pass.
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
pnpm docs:review local 'Reviewed explicit archive-only index reconstruction with the existing bounded builder, private native decoding under denied fallback, complete indexed change and parent identities, original graph visibility and preserved valid CLI bytes. Manual separates interrupted exact association retry from head publication, guarded checkout and unavailable writable capabilities.'
```

**Notes**

stack-source-indexes-docs-review.log: Manual reviewed; source fingerprint refreshed.
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
cargo fmt --all
```

**Notes**

stack-source-indexes-source.log: Formatted new provider/test modules.
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
CARGO_INCREMENTAL=0 pnpm docs:check
```

**Notes**

stack-source-indexes-docs-check.log: 139-file closure, links/anchors and nine diagrams in both themes pass.
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

stack-source-indexes-spec-first.log: 13 protected files/five canonical chapters pass.
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

stack-source-indexes-spec-validation.log: Five validators/23 chapters/173 requirements pass.
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
pnpm check:progress
```

**Notes**

stack-source-indexes-progress.log: New parent E021 aggregate result was marked pass despite explicit remaining gates. Corrected only that newly written field to pending in its own Jujutsu change; all 502 preceding bodies and passing run records unchanged. Checker and completion rules retained.
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
pnpm check:progress
```

**Notes**

stack-source-indexes-progress-after.log: 142 tasks pass after same-slice E021 bookkeeping correction; M21.2 remains In progress.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Index restoration is explicit private provider behavior; it neither imports
  source extras nor selects operation heads, changes companion state, writes
  checkout files or advertises writable Stacks capabilities.
- Stage 3 still requires exact operation-head publication preserving concurrent
  siblings, guarded physical checkout/durable fault recovery and
  prepare/apply/status/discovery production composition.
- "Stages 4-7 remain approved and required: actual
  setup/merge/WIP/membership/switching, exact file and issued-hunk
  transfers/commit, graph actions/deletion policy/guarded undo, linked-native
  real-storage recovery and full cold production UI flows."
- Two inherited service strict-lint diagnostics remain unresolved/unwaived.
  Owning no-deps lint is not aggregate dependency qualification.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Implement safe original operation publication, then guarded checkout and runtime command composition; continue every remaining approved stage.
<!-- /evidence-field -->

<!-- evidence-field:out_of_scope markdown -->
## Out of scope

No production UI/startup/generated transport changes or new browser/visual review, screenshot/baseline updates, publication, installed executable/service replacement, existing repository reset/migration, reserved-ref cleanup, linked-native/real-RustFS commands or unrelated root aggregate lanes.
<!-- /evidence-field -->

<!-- evidence-field:summary markdown -->
## Summary

41 unique direct cases pass: 26 source recovery (eight new index cases), eight foreground and seven archive. Four ignored process helpers run through passing parents. Missing indexes restore without source extras/import; native change/parent graphs preserve original versus future visibility and valid CLI index bytes. Owning package check/strict no-deps lint, boundaries/architecture, formatting, documentation and mapped specification/progress checks pass. All 503 parent evidence bodies remain byte-identical after the documented same-slice E021 aggregate-status correction. Safe publication, guarded checkout, runtime commands and Stages 4-7 remain incomplete; no writable capability is advertised.
<!-- /evidence-field -->

