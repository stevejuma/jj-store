---
schema: 1
id: E-M17-063
---

# E-M17-063 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::scheduled_import --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-scheduled-before.log 2>&1
```

**Notes**

Before state: two missing library dispatch/scheduling methods prevented compilation of the exact end-to-end regression.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::scheduled_import --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-scheduled-first.log 2>&1
```

**Notes**

One test-only CaptureRuntimeControl initializer missed the new bounded mailbox; corrected the fixture. No production check bypassed.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::scheduled_import --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-scheduled-exact.log 2>&1
```

**Notes**

Initial native sanitized publication through retained runtime and existing budgets passed.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::scheduled_import --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-scheduled-expanded.log 2>&1
```

**Notes**

Four passed, one failed. New resume fixture tried admission before the cached paused observation settled. Wait for the documented Running/Ready transition; preserve refusal and deadline.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::scheduled_import --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-scheduled-final-exact.log 2>&1
```

**Notes**

Seven exact scheduling tests passed before adding active/raw handoff and source-owner regression.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::scheduled_import::handoff --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-scheduled-handoff.log 2>&1
```

**Notes**

New partial-EOF fixture incorrectly assumed permanent raw allocation. EOF bytes are reconstructible and compact. Replace input with real retained ingress capacity refusal; preserve handoff assertions.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::scheduled_import::handoff --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-scheduled-handoff-repaired.log 2>&1
```

**Notes**

Both raw-capacity refusal and compact active cursor/alias/native session handoff regressions pass.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_worker::executor::tests::settlement_hook_follows_actual_cancelled_future_and_keeps_original_state --locked -- --exact --test-threads=1 > /tmp/jj-store-agent-capture-m172-scheduled-settlement.log 2>&1
```

**Notes**

Hook follows actual future cancellation; original state resumes, each settlement runs exactly once.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_supervisor::source::tests::cancelled_live_raw_state_overrides_older_compact_snapshot_before_import_handoff --locked -- --exact --test-threads=1 > /tmp/jj-store-agent-capture-m172-scheduled-stale-compact.log 2>&1
```

**Notes**

Actual raw state withholds replacement despite stale cached compact snapshot.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_supervisor::source::tests::cancelled_live_raw_state_overrides_older_compact_snapshot_before_import_handoff --locked -- --exact --test-threads=1 > /tmp/jj-store-agent-capture-m172-scheduled-stale-compact-counterfactual.log 2>&1
```

**Notes**

Temporarily omitted actual owned-state settlement before handoff. Exact assertion fails, proving cached compaction cannot authorize replacement. Original source restored before final validation.
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
cargo fmt --all
```

**Notes**

Repeated after each source/test refinement.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_supervisor::source::tests::cancelled_live_raw_state_overrides_older_compact_snapshot_before_import_handoff --locked -- --exact --test-threads=1 > /tmp/jj-store-agent-capture-m172-scheduled-stale-compact-restored.log 2>&1
```

**Notes**

Restored actual-state settlement; raw-channel regression passes again.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::scheduled_import --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-scheduled-exact-complete.log 2>&1
```

**Notes**

All nine scheduling regressions pass before wider suite found cached completed-prefix invalidation regression.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-scheduled-doc-format.log 2>&1
```
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-scheduled-canonical.log 2>&1
```

**Notes**

Initial pending-slice check passed; repeat after final evidence.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-scheduled-spec-first.log 2>&1
```

**Notes**

Initial pending-slice check passed; repeat after final evidence.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-scheduled-boundaries.log 2>&1
```

**Notes**

Initial pending-slice check passed; repeat after final evidence.
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
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-scheduled-architecture.log 2>&1
```

**Notes**

Initial pending-slice check passed; repeat after final evidence.
<!-- /run -->
<!-- /evidence-field -->

