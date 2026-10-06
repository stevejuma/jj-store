---
schema: 1
id: E-M21-038
---

# E-M21-038 details 3

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
pnpm exec prettier --write docs/implementation-plan.md docs/site/technical/architecture.md docs/site/reference/transport.md docs/site/technical/lifecycle.md spec/src/local-jj.md spec/src/mutations.md spec/src/architecture.md spec/src/cli.md spec/src/documentation.md
```

**Notes**

workspace-discovery-format.log: Changed documents formatted; ledger excluded.
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

workspace-discovery-coverage-format.log: Source review formatting; ledger excluded.
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
pnpm exec prettier --check docs/implementation-plan.md docs/maintenance/coverage.json docs/site/technical/architecture.md docs/site/reference/transport.md docs/site/technical/lifecycle.md spec/src/local-jj.md spec/src/mutations.md spec/src/architecture.md spec/src/cli.md spec/src/documentation.md
```

**Notes**

workspace-discovery-format-check.log: Changed-document formatting passes; ledger excluded.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_registered_preparation -- --test-threads=1
```

**Notes**

workspace-discovery-registered-family.log: All 19 actual registered execution/preparation/recovery parents pass.
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
CARGO_INCREMENTAL=0 cargo clippy --locked -p jj-store-service -p jj-store-server -p jj-store-cli --all-targets --no-deps -- -D warnings
```

**Notes**

workspace-discovery-clippy.log: Strict all-target no-deps checks for all three affected packages pass.
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
cargo fmt --all -- --check
```

**Notes**

workspace-discovery-rust-format.log: Rust formatting passes.
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
CARGO_INCREMENTAL=0 pnpm docs:check
```

**Notes**

workspace-discovery-docs.log: 139 documentation files, source reviews, diagrams, build and links/anchors pass.
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
python3 -u .local/stacks-backend-evidence/workspace-discovery-rust-checks.py
```

**Notes**

tool result: All sequential owning Rust, direct-consumer and documentation checks return exit 0.
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
pnpm spec:first
```

**Notes**

workspace-discovery-spec-first.log: 13 protected files and 6 canonical chapters pass.
<!-- /run -->

<!-- run -->
### Run 10

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

workspace-discovery-spec-validate.log: 5 validators, 23 chapters and 173 requirements pass.
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
pnpm check:progress
```

**Notes**

workspace-discovery-progress.log: 142 task bookkeeping checks pass.
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
python3 .local/stacks-backend-evidence/workspace-discovery-ledger-audit.py
```

**Notes**

tool result: 520 evidence entries; all 519 historical raw bodies byte-identical. Repeated after final entry update.
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

workspace-discovery-progress-after.log: Final bookkeeping validation follows this evidence update.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Owned registered runtime interface/composition must join current physical
  observation, selected catalog and original journal recovery. No HTTP routes,
  generated client contracts, wired production picker, setup lifecycle or
  durable cancellation executor are added.
- "Every remaining approved stage remains required: dirty-work capture, named
  combinations/switching, membership/rebuild, whole-file/server-issued hunk
  transfer, graph/deletion/scoped undo, linked-native real-storage and complete
  cold production UI flows."
- Root aggregate, UI/Storybook, RustFS, transport-generation and publication
  lanes are out of scope for this Rust contract/projection slice. No installed
  executable/service replacement, existing repository reset/migration,
  screenshot comparisons or visual baselines. Storybook 6057 is untouched.
  Existing __eh_frame linker warning is retained. Default-parallel full server
  qualification is not claimed.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Compose the owned registered discovery/status/pending runtime with explicit first-use private-state creation during authorized setup, then implement local HTTP and original production command lifecycle. Continue all approved stages.
<!-- /evidence-field -->

<!-- evidence-field:summary markdown -->
## Summary

55 unique focused Rust parents pass across discovery/recovery/protocol contracts, catalog integration/library and actual registered execution. Strict all-target no-deps checks for service/server/CLI, 50 governance tests, source boundaries, architecture, UI ownership, documentation, formatting, mapped specification and progress checks pass. All 519 historical raw ledger bodies are preserved. Bounded selected-catalog projection is qualified; registered runtime, HTTP and complete backend/UI wiring remain unfinished.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

