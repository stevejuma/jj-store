---
schema: 1
id: E-M21-009
---

# E-M21-009 details 3

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
pnpm exec prettier --check docs/implementation-plan.md docs/site/technical/architecture.md docs/site/technical/local.md docs/maintenance/coverage.json spec/src/local-jj.md spec/src/mutations.md spec/src/jj-integration.md spec/src/architecture.md spec/src/documentation.md spec/src/ui.md spec/src/verification.md
```

**Notes**

working-current-format-docs-check.log: review coverage JSON required formatting.
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

working-current-format-coverage.log: review JSON formatting repaired.
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
pnpm exec prettier --check docs/implementation-plan.md docs/site/technical/architecture.md docs/site/technical/local.md docs/maintenance/coverage.json spec/src/local-jj.md spec/src/mutations.md spec/src/jj-integration.md spec/src/architecture.md spec/src/documentation.md spec/src/ui.md spec/src/verification.md
```

**Notes**

working-current-format-docs-final.log: all targeted documents pass.
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

working-current-spec-first.log: 18 protected files and seven canonical chapters pass.
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

working-current-spec.log: five validators, 23 chapters and 173 requirements pass.
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

working-current-progress.log: 142 tasks pass; evidence bookkeeping only.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- "Stage 2 and M21.2 remain incomplete: scan transport, production read
  integration, cold registered-folder/browser acceptance and active-combination
  Unstaged assignment remain to implement. Private observations do not expose
  commands or populate Unstaged."
- "Strict lint qualification remains incomplete: four unchanged JJ and two
  unchanged service diagnostics reproduced, not suppressed or waived. Complete
  compile/focused tests do not qualify failed lint lanes."
- Observation is not filesystem CAS or a checkout lock. Whole-tree
  ignored/outside-sparse entries count toward physical limits; missing,
  hardlinked, symlink/special metadata and unsafe policy files fail explicitly.
  Recovery/checkout require foreground locking and fresh preconditions.
- Recoverable exact unpublished preparation/publication/guarded checkout,
  durable combinations and merge/WIP retention, file/hunk and graph commands,
  guarded undo, linked-native lifecycle and complete cold acceptance remain
  approved work.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Deliver the bounded registered-folder observation transport and real UI read integration, then qualify recoverable prepared execution before enabling writable controls.
<!-- /evidence-field -->

<!-- evidence-field:out_of_scope markdown -->
## Out of scope

UI type/Storybook/browser lanes (no UI implementation or bundle/lifecycle changes), generated wire contracts (no new route), root aggregate Rust/UI lanes, package publication, installed-service changes, real checkout execution, fresh live visual review and screenshot/baseline updates. Storybook remains port 6057/PID 36264.
<!-- /evidence-field -->

