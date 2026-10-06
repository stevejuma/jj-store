---
schema: 1
id: E-M21-021
---

# E-M21-021 details 3

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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_local_preparation -p jj-store-cli --test stack_registered_preparation
```

**Notes**

stack-source-extras-consumers.log: Eight actual preparation and three registered-folder consumers pass; process-loss helper executes through parents. Existing large-binary linker warning remains unsuppressed.
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
pnpm spec:first
```

**Notes**

stack-source-extras-spec-first.log: 18 protected files/five canonical chapters pass.
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
pnpm spec:validate
```

**Notes**

stack-source-extras-spec-validation.log: Five validators/23 chapters/173 requirements pass.
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
pnpm check:progress
```

**Notes**

stack-source-extras-progress.log: 142 tasks pass with M21.2 in progress.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Explicit extra restoration is private provider behavior; no writable Stacks
  capability, operation-head selection, derived index or physical checkout is
  enabled.
- Stage 3 still requires derived-index restoration, exact publication preserving
  concurrent sibling heads, guarded checkout/durable fault recovery and
  prepare/apply/status/discovery composition.
- "All seven approved stages remain required: real
  setup/merge/WIP/membership/switching; exact file then issued-hunk
  transfer/commit; graph commands/deletion policy/guarded undo; linked-native
  real-storage recovery and complete cold production UI flows."
- Two inherited service strict-lint diagnostics remain unresolved/unwaived.
  Owning no-deps lint is not aggregate service qualification. Native server-test
  linker emits __eh_frame size warning without suppression.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Finish derived-index recovery and safe original operation publication, then guarded checkout and production command composition; continue every remaining approved engine and UI stage.
<!-- /evidence-field -->

<!-- evidence-field:out_of_scope markdown -->
## Out of scope

No production UI/startup or generated transport changes, fresh visual review, screenshots/baselines, publication, installed executable/service replacement, existing repository reset/migration, reserved-ref cleanup, linked-native/real-RustFS command execution or unrelated root aggregate lanes.
<!-- /evidence-field -->

<!-- evidence-field:summary markdown -->
## Summary

58 unique direct cases pass: 18 source recovery (nine new), eight foreground, eight reword, seven archive, six retention, eight actual preparation and three registered consumers. Ignored subprocess helpers execute through passing parents. Original exact extras target passes after all changes. Owning all-target check/strict no-deps lint, formatting, boundaries, architecture, documentation and mapped specification/progress checks pass. All 502 preceding evidence bodies remain byte-identical. Source index, safe operation publication, guarded checkout, runtime commands and Stages 4-7 remain incomplete; production mutations remain unavailable.
<!-- /evidence-field -->

