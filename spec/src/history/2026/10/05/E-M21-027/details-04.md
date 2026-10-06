---
schema: 1
id: E-M21-027
---

# E-M21-027 details 4

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
pnpm exec prettier --write spec/src/verification.md
```

**Notes**

stack-apply-policy-verification-format.log: Initial qualification mapping formatted; remains Planned.
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
pnpm exec prettier --write spec/src/verification.md
```

**Notes**

stack-apply-policy-verification-format-final.log: Final typed Published qualification note formatted; remains Planned.
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
pnpm exec prettier --check spec/src/architecture.md spec/src/cli.md spec/src/documentation.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/verification.md docs/implementation-plan.md docs/site/technical/local.md docs/site/reference/configuration.md docs/maintenance/coverage.json
```

**Notes**

stack-apply-policy-doc-format-check.log: Initial changed documents pass; ledger excluded.
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
pnpm exec prettier --check spec/src/architecture.md spec/src/cli.md spec/src/documentation.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/verification.md docs/implementation-plan.md docs/site/technical/local.md docs/site/reference/configuration.md docs/maintenance/coverage.json
```

**Notes**

stack-apply-policy-doc-format-complete.log: Final changed documents pass; ledger excluded.
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
pnpm check:progress
```

**Notes**

stack-apply-policy-progress.log: All 142 task/evidence checks pass; no completion promotion.
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

stack-apply-policy-spec-first.log: 14 protected files/six canonical chapters pass.
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

stack-apply-policy-spec-validation-final.log: Final five validators/23 chapters/173 requirements pass after known Published requirement and append.
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
python3 raw JSON body SHA-256 audit against stack-apply-policy-ledger-before.json
```

**Notes**

stack-apply-policy-ledger-audit.log: Exactly 509 evidence objects; all 508 preceding raw bodies remain byte-identical.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Private unchanged-tree reword execution is not complete Stage 3. General
  tree-changing checkout, durable combination/execution coordination and
  selected after-state, precise blocked/recovery transport,
  prepare/apply/status/discovery endpoints, dropped HTTP responses and cold
  production browser flows remain required.
- Actual combination authority/after-state and retirement remain fixtures.
  Explicit setup, merge/WIP preservation, membership/switching, exact
  whole-file/issued-hunk commits, graph/deletion/undo engines, linked-native
  real-storage and all approved production UI stages remain incomplete. Raw head
  replacement in negative fixtures is not a qualified CLI undo or combination
  engine.
- Unrelated Rust/RustFS/UI/Storybook/root aggregate lanes are out of scope. No
  installed binary/service replacement, existing-repository reset/migration,
  package publication, screenshot comparison or baseline update. Existing
  __eh_frame linker warning remains unsuppressed.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue general guarded physical checkout and durable execution/combination coordination, then real setup/merge-WIP selected state and command transport/UI wiring. Complete all seven approved stages; do not enable mutations from this private reword proof.
<!-- /evidence-field -->

<!-- evidence-field:summary markdown -->
## Summary

138 unique direct cases pass: 83 JJ provider, 34 server integration, nine journal/combination unit and 12 actual registered CLI cases. Current immutable ancestry, unpublished Unknown retry, acknowledgement-only historical publication, typed Published before-witness no-reannouncement, exact source-loss recovery, cancellation/authorization and killed-process original retry are qualified privately. Fixture combination state/retirement and raw-head fault injection do not qualify real engines. All 508 original raw ledger bodies remain unchanged. Production Stacks stays read-only; M21.2 and full integration remain In progress.
<!-- /evidence-field -->

