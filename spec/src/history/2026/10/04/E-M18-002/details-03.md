---
schema: 1
id: E-M18-002
---

# E-M18-002 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first (first)
```

**Notes**

Documentation/plan and shared UI change maps also require api, architecture and cli chapters; added scoped presentation/preview descriptions
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress (first)
```

**Notes**

Ledger named-test matcher accepts fn/test declarations, not Story names; retained story coverage in actual run notes and removed unsupported named-story references
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

5 validators;22 chapters;166 requirements
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
pnpm spec:first (after mapped documentation)
```

**Notes**

18 protected paths/6 canonical chapters
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
pnpm check:progress (after named-test correction)
```

**Notes**

135 tasks; historical evidence retained
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

JJ0.45.1;pnpm10.32.1;Codex CLI0.156.1; inherited Design Core local validation source unchanged
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Current AgentStore and capture API unchanged; all393 historical evidence objects preserved byte-for-byte; UI-only settings composition
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Dedicated installation Agents section; shared rows, actions, public Alert/Empty/Field components and optional header refresh. Root aggregate/Rust suites out of scope under targeted UI policy. Visual baselines remain visual-pending; live in-app visual inspection unavailable due browser connection. Existing backend busy/cold performance caveat unchanged. User-requested configured root paths follow as a separate cross-layer slice.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Expose actual configured CLI capture roots in typed status and these shared settings rows; preserve existing backend configuration and authorization.
<!-- /evidence-field -->

