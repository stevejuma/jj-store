---
schema: 1
id: E-M17-007
---

# E-M17-007 details 2

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
cargo test -p jj-store-capture --test reader_progress reader_cursor_is_durable_before_acknowledgement_and_survives_reopening --locked -- --exact > /tmp/jj-store-agent-capture-m172-cursor-before.log 2>&1
```

**Notes**

Initial seam failed once before durable reader progress implementation.
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
cargo test -p jj-store-capture --test reader_progress reader_cursor_is_durable_before_acknowledgement_and_survives_reopening --locked -- --exact > /tmp/jj-store-agent-capture-m172-cursor.log 2>&1
```

**Notes**

Initial durable cursor implementation passes the exact regression.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-capture --test reader_progress inconsistent_or_regressive_readers_never_admit_another_pending_batch --locked -- --exact > /tmp/jj-store-agent-capture-m172-cursor-missing-before.log 2>&1
```

**Notes**

Live handle accepted a changed parser after cursor loss despite retained sanitized evidence.
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
cargo test -p jj-store-capture --test reader_progress --locked > /tmp/jj-store-agent-capture-m172-cursor-expanded.log 2>&1
```

**Notes**

Seven initial crash/order/privacy regressions pass after retained-evidence admission fix.
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
cargo test -p jj-store-capture --test reader_progress --locked > /tmp/jj-store-agent-capture-m172-cursor-final.log 2>&1
```

**Notes**

All nine cursor durability, capacity and concurrency regressions pass.
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
cargo test -p jj-store-capture --test reader_progress reader_cursor_is_durable_before_acknowledgement_and_survives_reopening --locked -- --exact > /tmp/jj-store-agent-capture-m172-cursor-exact-final.log 2>&1
```

**Notes**

Final exact initial regression passes after bounded recovery refinement.
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
cargo test -p jj-store-capture --test reader_progress inconsistent_or_regressive_readers_never_admit_another_pending_batch --locked -- --exact > /tmp/jj-store-agent-capture-m172-cursor-missing-final.log 2>&1
```

**Notes**

Final exact missing-journal/parser regression passes.
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
cargo test -p jj-store-capture --lib --test reader_progress --locked > /tmp/jj-store-agent-capture-m172-cursor-runtime.log 2>&1
```

**Notes**

Nine direct reader tests and one shared lease regression pass.
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
cargo test -p jj-store-cli --test agent_capture_spool --test agent_capture_publication --test local_agent_recovery --locked > /tmp/jj-store-agent-capture-m172-cursor-consumers.log 2>&1
```

**Notes**

All 21 direct native/local spool, publication and recovery regressions pass; two subprocess fixture helpers ignored.
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
cargo test -p jj-store-cli --test agent_capture_spool --test agent_capture_publication --test local_agent_recovery --locked > /tmp/jj-store-agent-capture-m172-cursor-consumers-final.log 2>&1
```

**Notes**

Final direct consumers pass after journal recovery refinement: 21 tests, two subprocess fixture helpers ignored.
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
cargo check -p jj-store-capture -p jj-store-cli -p jj-store-server --locked > /tmp/jj-store-agent-capture-m172-cursor-check.log 2>&1
```

**Notes**

Affected runtime and direct consumer packages check.
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
cargo clippy --no-deps -p jj-store-capture -p jj-store-cli -p jj-store-server --lib --test reader_progress --test agent_capture_spool --test agent_capture_publication --test local_agent_recovery --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-cursor-clippy.log 2>&1
```

**Notes**

Strict clippy passes for affected libraries and exact runtime/consumer test binaries.
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
cargo fmt --all
```

**Notes**

Rust formatting applied.
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
pnpm exec prettier --write spec/src/cli.md spec/src/collaboration.md spec/src/compatibility.md
```

**Notes**

Changed canonical chapter formatting applied.
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
pnpm spec:validate
```

**Notes**

Five validators, 21 chapters and 160 requirements pass.
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
pnpm check:boundaries
```

**Notes**

All 17-package boundaries pass.
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
pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

Ledger formatting applied.
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

129 tasks validate with all 261 parent entries preserved.
<!-- /run -->
<!-- /evidence-field -->

