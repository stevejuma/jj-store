---
schema: 1
id: E-M17-049
---

# E-M17-049 details 2

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
cargo fmt --all
```

**Notes**

Pinned formatting before/after journal implementation and leaf regression repairs; repeated after extraction.
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
cargo test -p jj-store-server --test capture_control_state pause_receipts_survive_reopen_and_old_replay_cannot_reverse_newer_resume --locked > /tmp/jj-store-agent-capture-m172-control-state-before.log 2>&1
```

**Notes**

Feature-absence compile baseline: one E0432 missing journal module import, not a behavior failure.
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
cargo test -p jj-store-server --test capture_control_state pause_receipts_survive_reopen_and_old_replay_cannot_reverse_newer_resume --locked > /tmp/jj-store-agent-capture-m172-control-state-after.log 2>&1
```

**Notes**

Initial unfinished implementation has one E0277 IO error conversion failure; route tempfile construction through the journal error mapper.
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
cargo test -p jj-store-server --test capture_control_state pause_receipts_survive_reopen_and_old_replay_cannot_reverse_newer_resume --locked > /tmp/jj-store-agent-capture-m172-control-state-compile-fixed.log 2>&1
```

**Notes**

Original receipt/reopen/latest-policy regression passes after compile repair.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-server --test capture_control_state an_open_handle_refuses_a_replaced_lock_or_directory --locked > /tmp/jj-store-agent-capture-m172-control-lock-before.log 2>&1
```

**Notes**

Two E0583 integration leaf module path errors; fix explicit paths without assertion changes.
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
cargo test -p jj-store-server --test capture_control_state an_open_handle_refuses_a_replaced_lock_or_directory --locked > /tmp/jj-store-agent-capture-m172-control-lock-before2.log 2>&1
```

**Notes**

One genuine functional failure: existing handle accepts a new physical lock. Root previously reselected lock identity on each operation.
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
cargo test -p jj-store-server --test capture_control_state an_open_handle_refuses_a_replaced_lock_or_directory --locked > /tmp/jj-store-agent-capture-m172-control-lock-after.log 2>&1
```

**Notes**

Exact before regression passes after pinning original physical lock per handle. Restoring original lock/directory restores exact policy; replacement generation cannot inherit old requests.
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
cargo test -p jj-store-server --test capture_control_state --lib capture_control_state --locked > /tmp/jj-store-agent-capture-m172-control-state-suite.log 2>&1
```

**Notes**

One owning unit test passes: real lock loss after selected file replacement returns unknown and exact replay recovers. Name filter selects no integration entries, so this is not the integration suite.
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
cargo test -p jj-store-server --test capture_control_state --locked > /tmp/jj-store-agent-capture-m172-control-state-integration.log 2>&1
```

**Notes**

All nine integration entries pass; direct journal total is ten with post-replacement unit test. Full capacity uses one valid bounded fixture, not thousands of writes.
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
cargo check -p jj-store-server --all-targets --locked > /tmp/jj-store-agent-capture-m172-control-state-check.log 2>&1
```

**Notes**

Direct gate passes; formatting/canonical/progress checks repeated after final sealing.
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
cargo clippy -p jj-store-server --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-control-state-clippy.log 2>&1
```

**Notes**

Direct gate passes; formatting/canonical/progress checks repeated after final sealing.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-control-state-doc-format.log 2>&1
```

**Notes**

Direct gate passes; formatting/canonical/progress checks repeated after final sealing.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-control-state-rustfmt.log 2>&1
```

**Notes**

Direct gate passes; formatting/canonical/progress checks repeated after final sealing.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-control-state-boundaries.log 2>&1
```

**Notes**

Direct gate passes; formatting/canonical/progress checks repeated after final sealing.
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
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-control-state-architecture.log 2>&1
```

**Notes**

Direct gate passes; formatting/canonical/progress checks repeated after final sealing.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-control-state-spec-validate.log 2>&1
```

**Notes**

Direct gate passes; formatting/canonical/progress checks repeated after final sealing.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-control-state-spec-first.log 2>&1
```

**Notes**

Direct gate passes; formatting/canonical/progress checks repeated after final sealing.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-control-state-progress.log 2>&1
```

**Notes**

Direct gate passes; formatting/canonical/progress checks repeated after final sealing.
<!-- /run -->
<!-- /evidence-field -->

