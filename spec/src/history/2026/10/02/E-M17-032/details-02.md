---
schema: 1
id: E-M17-032
---

# E-M17-032 details 2

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
cargo test -p jj-store-capture --test reader_progress headroom::admitted_reader_reserves_maximum_native_preparation_across_restart_and_shared_writers --locked -- --exact > /tmp/jj-store-agent-capture-m172-headroom-before.log 2>&1
```

**Notes**

Initial new fixture used a crate-root module path without an explicit path attribute; fixed before testing the regression.
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
cargo test -p jj-store-capture --test reader_progress headroom::admitted_reader_reserves_maximum_native_preparation_across_restart_and_shared_writers --locked -- --exact > /tmp/jj-store-agent-capture-m172-headroom-before-fixed-fixture.log 2>&1
```

**Notes**

Fixture assumed Clone on ReaderCapture and constructors on CollabObjectRef; replaced with explicit inputs and owned fields. Three compile errors, no production dependency or assertion change.
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
cargo test -p jj-store-capture --test reader_progress headroom::admitted_reader_reserves_maximum_native_preparation_across_restart_and_shared_writers --locked -- --exact > /tmp/jj-store-agent-capture-m172-headroom-before-regression.log 2>&1
```

**Notes**

Before: real one-case regression fails because captured physical bytes cannot hold a permitted 4096-edit prepared request.
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
cargo fmt -p jj-store-capture
```

**Notes**

Affected source/test formatting; rerun after fixture changes.
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
cargo test -p jj-store-capture --test reader_progress headroom:: --locked > /tmp/jj-store-agent-capture-m172-headroom-exact.log 2>&1
```

**Notes**

Both native maximum-delta/shared-writer/restart and local large-payload/binding regressions pass.
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
cargo fmt -p jj-store-server
```

**Notes**

Affected native regression and fixture formatting; rerun after fixture changes.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor headroom::full_captured_queue_drains_ordered_native_heads_without_capacity_increase --locked -- --exact > /tmp/jj-store-agent-capture-m172-headroom-native.log 2>&1
```

**Notes**

Actual native MemoryStore AgentStore holds all three cumulative sanitized transcript additions after exact full captured capacity; same-partition preauthoring and rejected ingress remain withheld.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --locked > /tmp/jj-store-agent-capture-m172-headroom-capture.log 2>&1
```

**Notes**

Broader capture lane exposed a 1 MiB offline-admission fixture below the new native preparation reservation. Changed only its fixture/reopened capacity; original recovery assertions retained.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --test codex_activity admitted_offline_tail_resumes_in_a_fresh_process_after_sanitized_progress --locked -- --exact > /tmp/jj-store-agent-capture-m172-headroom-activity.log 2>&1
```

**Notes**

Original failing offline child-process recovery case passes with a capacity that admits the new reserved encoding.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --locked > /tmp/jj-store-agent-capture-m172-headroom-capture-expanded.log 2>&1
```

**Notes**

Expanded capture lane exposed a long-prefix fixture retaining many captures in 4 MiB. Increased only this non-pressure fixture capacity to 16 MiB; kept every prefix/rewrite/proof assertion.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --test codex_recovery long_prefix_recovery_is_bounded_and_restarts_after_append_or_rewrite --locked -- --exact > /tmp/jj-store-agent-capture-m172-headroom-long-recovery.log 2>&1
```

**Notes**

Original long-prefix recovery target passes after fixture sizing.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --locked > /tmp/jj-store-agent-capture-m172-headroom-capture-final.log 2>&1
```

**Notes**

Complete shared capture runtime passes 88 parent cases plus the launched privacy child. Before/after failure count: two discovered fixture failures resolved; no product assertion weakened. Physical-capacity concurrency fixture now measures actual reserved bytes.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor --test capture_source --test capture_partition --test capture_executor --test capture_reanchor --locked > /tmp/jj-store-agent-capture-m172-headroom-server.log 2>&1
```

**Notes**

Invocation used two nonexistent test binaries; no tests ran. Corrected to actual registered direct-consumer targets.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_activation --test capture_activity --test capture_admission --test capture_authority --test capture_parking --test capture_pipeline --test capture_pipeline_worker --test capture_reanchor --test capture_recovery --test capture_resume --test capture_source --test capture_supervisor --locked > /tmp/jj-store-agent-capture-m172-headroom-server-corrected.log 2>&1
```

**Notes**

Activation/activity/admission/authority cases pass. Parking fixture that retains multiple oversized-record omission batches needed 16 MiB for reserved encodings; its lease/proof/cursor assertions remain unchanged.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_parking parked_pipeline_retains_verification_and_releases_leases --locked -- --exact > /tmp/jj-store-agent-capture-m172-headroom-parking.log 2>&1
```

**Notes**

Original exact parked verification target passes after fixture capacity sizing.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_pipeline --test capture_pipeline_worker --test capture_reanchor --test capture_recovery --test capture_resume --test capture_source --test capture_supervisor --locked > /tmp/jj-store-agent-capture-m172-headroom-server-remaining.log 2>&1
```

**Notes**

All remaining source/pipeline/recovery binaries pass. Supervisor backlog case exposed that its freed compact A request capacity was too small to admit any new reserved B batch; the exact full-queue and receipt assertions remain unchanged.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor backlog::full_prepared_queue_retains_unknown_receipt_and_recovers_independent_b_after_restart --locked -- --exact > /tmp/jj-store-agent-capture-m172-headroom-backlog.log 2>&1
```

**Notes**

Increasing the retained native A body also requires its original admission capacity to cover local payload duplication; corrected only fixture startup capacity to 8 MiB. The live full queue is still reopened at the exact actual prepared A byte size.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor backlog::full_prepared_queue_retains_unknown_receipt_and_recovers_independent_b_after_restart --locked -- --exact > /tmp/jj-store-agent-capture-m172-headroom-backlog-sized.log 2>&1
```

**Notes**

Original full prepared backlog/revocation/restart/receipt case passes with an actual larger native payload. Freed physical capacity now admits the new reserved B batch without changing the exact full-queue bound or cursor/receipt assertions.
<!-- /run -->
<!-- /evidence-field -->

