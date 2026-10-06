---
schema: 1
id: E-M17-002
---

# E-M17-002 details 3

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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo check -p jj-agent -p jj-store-wal -p jj-store-service --locked
```

**Notes**

Affected packages compile with locked dependencies.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo clippy -p jj-agent -p jj-store-wal -p jj-store-service --lib --test agent_catalog --test local_agent --locked -- -D warnings
```

**Notes**

Existing redundant_locals lint in jj-store-jj/src/local_browser/operations.rs:202. Before cleanup the complete file is byte-identical to the parent specification commit. Removed the no-op parent binding without changing ancestry, membership, budgets or identity; record separately from capture behavior.
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
pnpm spec:first
```

**Notes**

9 protected files covered by 5 canonical chapters before the one-line ancestry lint cleanup.
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
pnpm format:rust:check
```

**Notes**

Workspace and embedded-runtime formatting pass.
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
pnpm check:boundaries
```

**Notes**

16 package boundaries pass.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/local-jj.md spec/src/milestones.md spec/src/mutations.md spec/src/publication.md spec/src/verification.md
```

**Notes**

Documentation formatting including the partial requirement evidence update.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo clippy -p jj-agent -p jj-store-wal -p jj-store-service --lib --test agent_catalog --test local_agent --locked -- -D warnings
```

**Notes**

After ancestry cleanup, three existing collapsible_if lints in content_search/extract.rs. Before cleanup the file is byte-identical to the parent specification commit. Flattened equivalent section/root and recent-message guards; direct content_search tests added to verification scope.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo clippy -p jj-agent -p jj-store-wal -p jj-store-service --lib --test agent_catalog --test local_agent --locked -- -D warnings
```

**Notes**

Final affected libraries and changed test targets pass strict clippy after four no-op dependency lint cleanups.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --test content_search --locked
```

**Notes**

6/6 direct content-search regressions pass after equivalent guard flattening; source limits, current revisions, recent activity, pagination and revocation remain qualified.
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
cargo fmt --all
```

**Notes**

Final Rust formatting after equivalent guard flattening.
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
pnpm exec prettier --write spec/src/adapters.md spec/src/collaboration.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/milestones.md spec/src/mutations.md spec/src/publication.md spec/src/verification.md
```

**Notes**

Owning canonical document formatting after lint cleanup.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo check -p jj-agent -p jj-store-wal -p jj-store-service -p jj-store-jj --locked
```

**Notes**

Final source state, including no-op direct dependency cleanups, compiles.
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
pnpm spec:validate
```

**Notes**

Final chapters: 5 validators, 21 chapters and 160 requirements.
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
pnpm spec:first
```

**Notes**

Final source scope: 11 protected files and 7 canonical chapters.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm format:rust:check
```

**Notes**

Final Rust state formats cleanly, including embedded runtime.
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
pnpm exec prettier --check spec/src/adapters.md spec/src/collaboration.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/milestones.md spec/src/mutations.md spec/src/publication.md spec/src/verification.md
```

**Notes**

All changed canonical documents pass formatting.
<!-- /run -->

<!-- run -->
### Run 17

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

Pass evidence cannot retain remaining gaps. Overall M17.1 gate corrected to pending while actual focused passing results stay recorded; bounded append and worker gates remain explicit.
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
pnpm check:progress
```

**Notes**

Corrected final bookkeeping: 129 tasks. Passing focused runtime evidence is retained while the M17.1 gate remains pending for bounded append; no milestone completion claim.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

