---
schema: 1
id: E-M17-062
---

# E-M17-062 details 2

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
jj --no-pager st
```

**Notes**

lwmqylok empty atop b0d4bcc6; prior commit session 26481 confirmed exit 0. No overlap.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
jj describe -m 'Connect explicit history consent to native source admission and guarded pipelines'
```
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::native_import --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-native-import-before.log 2>&1
```

**Notes**

Before target matched no tests; records absent coverage, not a functional acceptance pass.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo check -p jj-store-server --lib --locked > /tmp/jj-store-agent-capture-m172-native-import-check-initial.log 2>&1
```

**Notes**

Initial compilation found non-Copy spool bindings moved through borrowed frames. Repaired to borrowed comparison; no suppression.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::native_import --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-native-import-exact.log 2>&1
```

**Notes**

Initial fixture compile found one remaining binding borrow mismatch after the initial five non-Copy binding moves were repaired.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::native_import --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-native-import-exact-repaired.log 2>&1
```

**Notes**

Initial seven native import regressions passed. Subsequent review added retained-target checks and exact selected-boundary reconciliation after unknown enrollment.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::native_import --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-native-import-exact-final.log 2>&1
```

**Notes**

All nine native import regressions passed, including raw-channel retention and caller cancellation across actual pre/post-enrollment native awaits.
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
cargo fmt --all
```

**Notes**

Repeated after source, test and review refinements.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-native-import-doc-format.log 2>&1
```
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --test capture_admission --test capture_source --test capture_resume --test capture_parking --test capture_reanchor --test capture_recovery --test capture_pipeline --test capture_pipeline_worker --test capture_supervisor --test capture_runtime --test capture_management --test capture_control_state --test capture_import_state --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-native-import-direct.log 2>&1
```

**Notes**

Owning library completed 70 passed / 1 failed; Cargo did not run the remaining selected binaries. Raw-capacity fixture reopened the live spool for publication, losing its original fork FIFO group and racing the runtime handle OS mutation lock. Restore saved original shared handle. No production lock was weakened or errors ignored.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::native_import::ownership --locked -- --test-threads=1 --nocapture > /tmp/jj-store-agent-capture-m172-native-import-ownership-diagnostic.log 2>&1
```

**Notes**

One passed and one failed. Temporary test-only error print identified Config("another process holds this state or policy lock") from independently reopened live spool. Diagnostic removed before final validation. Existing assertions and deadlines retained.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::native_import::ownership --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-native-import-ownership-repaired.log 2>&1
```

**Notes**

Both owned admission and raw-frame regressions pass with original shared publication handle. Original-intent retry after unknown enrollment plus append also preserves accepted boundary.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-native-import-canonical.log 2>&1
```

**Notes**

Initial pending-slice governance; repeat after publisher review and final ledger updates.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-native-import-spec-first.log 2>&1
```

**Notes**

Initial pending-slice governance; repeat after publisher review and final ledger updates.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-native-import-boundaries.log 2>&1
```

**Notes**

Initial pending-slice governance; repeat after publisher review and final ledger updates.
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
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-native-import-architecture.log 2>&1
```

**Notes**

Initial pending-slice governance; repeat after publisher review and final ledger updates.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-native-import-progress.log 2>&1
```

**Notes**

Initial pending-slice governance; repeat after publisher review and final ledger updates.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-native-import-rust-format.log 2>&1
```

**Notes**

Initial pending-slice governance; repeat after publisher review and final ledger updates.
<!-- /run -->
<!-- /evidence-field -->

