---
schema: 1
id: E-M17-038
---

# E-M17-038 details 2

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor generations:: --locked > /tmp/jj-store-agent-capture-m172-generations-before.log 2>&1
```

**Notes**

Initial fixture cannot compile: SourcePositionV1 is not re-exported by jj_store_service::transport. Corrected with existing serialized cursor fields, without a new dependency.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor generations:: --locked > /tmp/jj-store-agent-capture-m172-generations-before-qualified.log 2>&1
```

**Notes**

Actual before regression fails after 11.52 seconds: unchanged archive advances accepted generation 2/sequence 5/offset 423 to generation 3/sequence 8/offset 467.
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
cargo fmt --all
```

**Notes**

Rust formatting repeated after implementation and meaningful regression additions; no unrelated tracked file changed.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor generations:: --locked > /tmp/jj-store-agent-capture-m172-generations-after.log 2>&1
```

**Notes**

Exact unchanged-file regression passes after scoped selection and channel floors.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor generations:: --locked > /tmp/jj-store-agent-capture-m172-generations-matrix.log 2>&1
```

**Notes**

Two exact cases pass: unchanged archive/current and delayed append, truncation, equal-length prefix rewrite, supervisor/spool restart and monotonic continuation.
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
cargo test -p jj-store-capture --test reader_progress source:: --test source_parking --locked > /tmp/jj-store-agent-capture-m172-generations-runtime-exact.log 2>&1
```

**Notes**

One passes; automatic admission fixture exhausts its 4 MiB padded preparation capacity at the fourth pending entry. Raised only this fixture to 8 MiB, retaining all assertions. Cargo aborts before source_parking, so it supplies no parking evidence.
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
cargo test -p jj-store-capture --test reader_progress source:: --locked > /tmp/jj-store-agent-capture-m172-generations-runtime-corrected.log 2>&1
```

**Notes**

Both snapshot and automatic admission cases pass after fixture capacity correction.
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
cargo test -p jj-store-capture --test source_parking independent_generation_advancement_withholds_an_exact_parked_raw_allocation --locked > /tmp/jj-store-agent-capture-m172-generations-raw-exact.log 2>&1
```

**Notes**

One exact memory-only raw case passes; allocation refusal preserves batch/cursor/sequence and Arc identity and releases fresh leases.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_parking generations:: --locked > /tmp/jj-store-agent-capture-m172-generations-parked-exact.log 2>&1
```

**Notes**

One native fixture fails before admission: scanner paths are canonical but requested macOS temporary path is not. Canonicalized the exact requested path without weakening source or queue assertions.
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
cargo fmt -p jj-store-capture -p jj-store-server
```

**Notes**

Final affected Rust sources and regressions formatted; repeated after final additions.
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
cargo test -p jj-store-capture --test reader_progress source:: --locked > /tmp/jj-store-agent-capture-m172-generations-runtime-qualified.log 2>&1
```

**Notes**

Both exact cases pass with generic sanitized pending floors included. Snapshot selection spans partitions, journal repair and restart; atomic admission rejects stale allocations across independent handles while preserving accepted retries.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_parking generations:: --locked > /tmp/jj-store-agent-capture-m172-generations-parked-qualified.log 2>&1
```

**Notes**

One native two-physical-reader case passes: conflict repeatedly withholds the parked raw frame without enqueue, queue/cursor mutation or fresh lease leakage.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-generations-boundaries.log 2>&1
```

**Notes**

Owning architecture dependency boundaries pass.
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
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-generations-architecture.log 2>&1
```

**Notes**

Repository architecture checks pass; changed behavioral production modules remain below 300 lines.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-cli --test agent_capture_spool --test agent_capture_publication --test local_agent_recovery --locked > /tmp/jj-store-agent-capture-m172-generations-cli-final.log 2>&1
```

**Notes**

31 direct native/local CLI spool, publication and recovery cases pass. Two process-entry helpers remain explicitly ignored in parent runs.
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
cargo test -p jj-store-capture --locked > /tmp/jj-store-agent-capture-m172-generations-runtime-final.log 2>&1
```

**Notes**

Complete capture crate passes: 95 main-process cases plus one visible fresh-process privacy helper. Three child-entry helpers remain explicitly ignored in parent runs; no ignored case counts as an executed pass.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor --test capture_source --test capture_pipeline --test capture_recovery --test capture_parking --test capture_reanchor --test capture_resume --locked > /tmp/jj-store-agent-capture-m172-generations-server-final.log 2>&1
```

**Notes**

53 direct server cases pass; existing blocked_directory_keeps_its_owner_while_another_source_delivers times out at its unchanged ten-second B-delivery deadline (blocked.rs:170) while other qualification suites run concurrently. No assertion, timeout or test is changed. Exact isolation and same complete command rerun follow; this run is not claimed as a suite pass.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor blocked::blocked_directory_keeps_its_owner_while_another_source_delivers --locked > /tmp/jj-store-agent-capture-m172-generations-blocked-exact.log 2>&1
```

**Notes**

The exact blocked-directory fault passes unchanged in 9.48 seconds, retaining its real concurrent A/B work, ten-second delivery deadline, overdue/pause/draining ownership, native publication and cleanup assertions.
<!-- /run -->
<!-- /evidence-field -->

