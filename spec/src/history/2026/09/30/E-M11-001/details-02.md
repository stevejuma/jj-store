---
schema: 1
id: E-M11-001
---

# E-M11-001 details 2

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
jj git init --colocate .
```

**Notes**

App-managed Git worktrees cannot host colocated JJ. Archived the empty managed worktree and used native jj workspace add.
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
jj workspace add /Users/stevejuma/code/lapismd/jj-store-native-project-tasks --name native-project-tasks -r uqvnuppm
```

**Notes**

Clean native workspace from integrated base 4a224b91; original active work preserved.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-wal --test work_store --locked workstore_receipt_survives_checkpoint_and_reconstruction -- --exact
```

**Notes**

Before state: missing WorkStore imports/publication method; acceptance cannot compile.
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
cargo check -p jj-work
```

**Notes**

Neutral domain package compiles; updates workspace lock with local crate.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 36
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-wal --test work_store workstore_receipt_survives_checkpoint_and_reconstruction -- --exact
```

**Notes**

Intermediate ambiguous glob imports; replaced them with explicit owned type exports.
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
cargo test -p jj-store-wal --test work_store --locked workstore_receipt_survives_checkpoint_and_reconstruction -- --exact
```

**Notes**

One exact durable receipt/cold reconstruction acceptance passes.
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
cargo test -p jj-store-wal --test work_store --locked
```

**Notes**

Intermediate test used private WriterEpoch tuple constructor; corrected to fixed-width native new().
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
cargo test -p jj-store-wal --test work_store --locked
```

**Notes**

Five focused native WorkStore tests pass.
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
cargo test -p jj-work --test commands --locked
```

**Notes**

One model acceptance passes, covering cross-project/cross-task rejection and admission/actors.
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
cargo fmt -p jj-work -p jj-store-wal
```

**Notes**

Affected Rust crates formatted.
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
cargo test -p jj-store-wal --locked
```

**Notes**

57 actual tests pass, zero failures. Escalation: shared manifest/history/copy format affects native, AgentStore and ReviewStore authority.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm install --frozen-lockfile --lockfile=false --ignore-scripts
```

**Notes**

Existing pnpmfile checksum makes frozen local validation incompatible; no tracked lockfile changed.
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
pnpm install --lockfile=false --ignore-scripts
```

**Notes**

Private local validation dependencies installed with ignored generated node_modules; registry manifest/lock retained.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/projects.md spec/src/SUMMARY.md spec/src/verification.md spec/src/publication.md spec/src/architecture.md spec/src/replication.md spec/src/spec-governance.md spec-validator.config.mjs scripts/check-boundaries.mjs
```

**Notes**

Changed canonical documents and governance formatted.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate
```

**Notes**

New verification rows were outside the existing Markdown table; moved into that table.
<!-- /run -->

<!-- run -->
### Run 16

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

24 protected files map to six changed owning chapters.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:boundaries
```

**Notes**

15 packages preserve neutral domain and owned JJ dependency boundaries.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/verification.md spec/src/milestones.md
```

**Notes**

New verification and milestone rows moved inside their existing Markdown tables and formatted.
<!-- /run -->
<!-- /evidence-field -->

