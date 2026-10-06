---
schema: 1
id: E-M17-028
---

# E-M17-028 details 2

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor enabled_supervisor_backfills_new_start_without_ingesting_inactive_history --locked -- --exact > /tmp/jj-store-agent-capture-m172-supervisor-before.log 2>&1
```

**Notes**

Original connected regression cannot compile: supervisor API is absent and test incorrectly calls an upstream trait on NativeSession. Replaced that test read with another owned NativeRequest; no upstream server import introduced.
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
cargo fmt -p jj-store-capture -p jj-store-server
```

**Notes**

Repeated after each support, supervisor, resource/fault and fixture correction.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor enabled_supervisor_backfills_new_start_without_ingesting_inactive_history --locked -- --exact > /tmp/jj-store-agent-capture-m172-supervisor-after.log 2>&1
```

**Notes**

Original native-service backfill/inactive-history/privacy/code-head regression passes; remaining unused fixture helpers are exercised by expanded cases.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor --locked > /tmp/jj-store-agent-capture-m172-supervisor-expanded.log 2>&1
```

**Notes**

Discovery completion was mistaken for an armed activity baseline in two fixtures; the long-prefix fixture stopped at Waiting before EOF. Added observed-source status, bounded predicate waits and exact compact-EOF caught-up semantics; retained all assertions.
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
cargo test -p jj-store-capture --test codex_activity --test source_parking --locked > /tmp/jj-store-agent-capture-m172-compaction.log 2>&1
```

**Notes**

All 15 direct activity and parked reader cases pass, including old partial-start exclusion, renewed activity and exact completed partial-record bytes.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor --locked > /tmp/jj-store-agent-capture-m172-supervisor-expanded-fixed.log 2>&1
```

**Notes**

Six initial supervisor cases pass with actual bounded state predicates.
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
cargo check -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-supervisor-check.log 2>&1
```

**Notes**

Affected capture/server and direct CLI consumers compile before additional fault/resource cases.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor blocked::blocked_directory_keeps_its_owner_while_another_source_delivers --locked -- --exact > /tmp/jj-store-agent-capture-m172-supervisor-blocked.log 2>&1
```

**Notes**

The blocked A owner permits B delivery, but pause/resume cannot settle due repeated shared journal lock collisions.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor blocked::blocked_directory_keeps_its_owner_while_another_source_delivers --locked -- --exact > /tmp/jj-store-agent-capture-m172-supervisor-blocked-diagnostic.log 2>&1
```

**Notes**

Bounded status confirms persistent unavailable turns and retained reservation; raw paths/bodies are not logged.
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
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-supervisor-lint.log 2>&1
```

**Notes**

Private scheduling branch triggers collapsible-if lint; corrected without changing its predicates.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor blocked::blocked_directory_keeps_its_owner_while_another_source_delivers --locked -- --exact > /tmp/jj-store-agent-capture-m172-supervisor-blocked-reason.log 2>&1
```

**Notes**

Temporary static diagnostic identifies parked reader progress lock contention; diagnostic removed.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor blocked::blocked_directory_keeps_its_owner_while_another_source_delivers --locked -- --exact --nocapture > /tmp/jj-store-agent-capture-m172-supervisor-blocked-detail.log 2>&1
```

**Notes**

Temporary filesystem-error diagnostic confirms authoritative journal lock collision; diagnostic removed.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor blocked::blocked_directory_keeps_its_owner_while_another_source_delivers --locked -- --exact > /tmp/jj-store-agent-capture-m172-supervisor-blocked-fifo.log 2>&1
```

**Notes**

Forked spool FIFO prevents repeated journal collisions and the original blocked-owner case passes; broader acquisition contention remains separately reproduced.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --locked > /tmp/jj-store-agent-capture-m172-supervisor-runtime.log 2>&1
```

**Notes**

Full affected capture runtime passes: 84 parent cases plus a launched privacy child; three explicit fixture targets stay ignored in parent enumeration.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-supervisor-lint-final.log 2>&1
```

**Notes**

Expanded fixture config specifies every field; removed redundant default struct update.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor --test capture_activation --test capture_parking --test capture_pipeline_worker --test capture_pipeline --test capture_resume --test capture_recovery --test capture_source --test capture_activity --test capture_authority --test capture_admission --locked > /tmp/jj-store-agent-capture-m172-supervisor-server-final.log 2>&1
```

**Notes**

All 34 prior direct native cases and seven supervisor cases pass; two concurrent source registry acquisitions can still repeatedly collide, preventing caught-up.
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
cargo test -p jj-store-server --lib capture_worker::executor::tests --locked > /tmp/jj-store-agent-capture-m172-supervisor-executor.log 2>&1
```

**Notes**

All eight owned executor tests pass, including real five-second shutdown timeout, retained resources, late known outcome, queued expiry and task loss.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-cli --test agent_capture_spool --test agent_capture_publication --test local_agent_recovery --test local_agent_provenance --test local_agent_provenance_cli --locked > /tmp/jj-store-agent-capture-m172-supervisor-cli.log 2>&1
```

**Notes**

All 43 direct native/local CLI capture publication, spool, provenance and recovery cases pass; two real-RustFS profiles stay explicitly ignored.
<!-- /run -->
<!-- /evidence-field -->

