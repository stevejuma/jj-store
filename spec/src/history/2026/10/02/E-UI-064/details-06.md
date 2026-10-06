---
schema: 1
id: E-UI-064
---

# E-UI-064 details 6

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
pnpm spec:validate
```

**Notes**

Final canonical spec and provenance validation passed; no app root aggregate commands.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/NavigationPersistence.stories.svelte
```

**Notes**

Interrupted family exact retry 1/1 in 10.76 seconds, no production/story changes.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=1/3
```

**Notes**

Same full suite, first fresh-session shard: 18/18 files, 65/65 tests, 45.45 seconds. No exclusions or baseline writes.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=2/3
```

**Notes**

Same full suite, second fresh-session shard: 18/18 files, 69/69 tests, 68.07 seconds. No exclusions or baseline writes.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=3/3
```

**Notes**

Same full suite, final fresh-session shard: 17/17 files, 65/65 tests, 74.00 seconds. Combined sequential shards cover every 53/53 files and 199/199 tests; no exclusions, skipped tests or baseline writes. Unsharded lifetime interruptions remain recorded above.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:boundaries
```

**Notes**

Actual workspace metadata detected missing allowlist edge jj-collab to jj-work. Canonical architecture already permits owned WorkStore model types; updated precise boundary rule and negative coverage.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node --test --test-name-pattern collaboration_targets_use_owned_work_types_without_reverse_or_runtime_edges scripts/governance.test.mjs
```

**Notes**

Before narrow edge update: 1 exact regression failed on invalid layer dependency jj-work.
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
pnpm exec prettier --write scripts/check-boundaries.mjs scripts/governance.test.mjs spec/src/spec-governance.md; node --test scripts/governance.test.mjs; pnpm check:boundaries
```

**Notes**

Owning governance file 25/25; actual Cargo metadata 16 packages passes. Reverse jj-work to jj-collab and jj-collab publication/upstream JJ imports still rejected. No UI/Rust implementation change after final passing browser run.
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
pnpm exec prettier --write spec/src/milestones.md; pnpm spec:first; pnpm spec:validate
```

**Notes**

Final boundary/ledger changes: 37 protected files, 12 canonical chapters; 5 validators, 21 chapters, 147 requirements.
<!-- /run -->

<!-- run -->
### Run 10

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

Ledger tests array only recognizes fn/def/test definitions; new Story name cannot use that named-test field. Story remains recorded in changed files and executed Storybook runs. Removed unsupported Story fragment without changing the validator or product tests.
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
pnpm exec prettier --write spec/src/milestones.md; pnpm check:progress
```

**Notes**

Final corrected ledger: 120 tasks; named Rust/Node/browser regressions resolve. Story evidence retained in files and actual run results. Functional acceptance complete; separate gaps preserved and milestone statuses unchanged.
<!-- /run -->
<!-- /evidence-field -->

