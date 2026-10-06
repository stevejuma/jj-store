---
schema: 1
id: E-M17-027
---

# E-M17-027 details 2

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_activation fresh_start_admits_current_directory_after_event_directory_revocation --locked -- --exact > /tmp/jj-store-agent-capture-m172-activation-before.log 2>&1
```

**Notes**

Original exact fresh current-B regression fails because prepare_active is absent.
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
cargo fmt -p jj-store-server -p jj-store-capture
```

**Notes**

Affected Rust formatting repeated after fresh eligibility, staged composition and fixture edits.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_activation fresh_start_admits_current_directory_after_event_directory_revocation --locked -- --exact > /tmp/jj-store-agent-capture-m172-activation-after.log 2>&1
```

**Notes**

Original current-B admission and independent old-A safe omission/B body publication passes; unused fixture helpers are exercised by later cases.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_activation --locked > /tmp/jj-store-agent-capture-m172-activation-expanded.log 2>&1
```

**Notes**

Three initial fresh proof/current-context/no-Write/renewal cases pass; later unbound/spool case exercises remaining start helper.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_activation --locked > /tmp/jj-store-agent-capture-m172-activation-expanded-2.log 2>&1
```

**Notes**

Four cases pass: fresh current-B, bounded partial-context/no-Write, stale snapshot/original-probe renewal, unbound and changed spool before allocation.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_activation fresh_start_admits_current_directory_after_event_directory_revocation --locked -- --exact > /tmp/jj-store-agent-capture-m172-activation-exact-final.log 2>&1
```

**Notes**

Exact original case passes after adding final admitted-channel validation around reader construction.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_activation --test capture_parking --test capture_pipeline_worker --test capture_pipeline --test capture_resume --test capture_recovery --test capture_source --test capture_activity --test capture_authority --test capture_admission --locked > /tmp/jj-store-agent-capture-m172-activation-server-direct.log 2>&1
```

**Notes**

Three activation cases passed; original test assumed unordered pending spool list was publication order. Exact drain correctly rejected a later same-partition batch. Fixture now selects source sequence order.
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
cargo test -p jj-store-capture --test codex_activity --test codex_resume --test source_registry --test source_registry_policy --locked > /tmp/jj-store-agent-capture-m172-activation-runtime-direct.log 2>&1
```

**Notes**

All 27 direct activity/current-context/retained eligibility/registry cases pass, including process fixtures.
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
cargo check -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-activation-check.log 2>&1
```

**Notes**

Affected capture/server and direct CLI consumers compile with additive activity-proof/staged API.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-activation-boundaries.log 2>&1
```

**Notes**

All 17 package boundaries pass; no additional dependency or persisted/wire shape.
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
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-activation-clippy.log 2>&1
```

**Notes**

Private eligibility enum large-variant lint found; boxed the opaque fresh proof without Debug or serialization.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_activation fresh_start_admits_current_directory_after_event_directory_revocation --locked -- --exact > /tmp/jj-store-agent-capture-m172-activation-exact-after-order.log 2>&1
```

**Notes**

Original exact case passes after explicit sequence selection and private proof sizing fix; no publisher-order guard weakened.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_activation --test capture_parking --test capture_pipeline_worker --test capture_pipeline --test capture_resume --test capture_recovery --test capture_source --test capture_activity --test capture_authority --test capture_admission --locked > /tmp/jj-store-agent-capture-m172-activation-server-final.log 2>&1
```

**Notes**

All 34 complete direct native service/WAL/AgentStore over MemoryStore capture cases pass; shared staged eligibility extraction requires all admitted/fresh resume, pipeline/parking/worker/recovery/admission consumers.
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
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-activation-clippy-final.log 2>&1
```

**Notes**

Final affected/direct-consumer strict lint passes; no fixture warnings remain.
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
cargo check -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-activation-check-final.log 2>&1
```

**Notes**

Final affected/direct-consumer all-target compile passes after boxed proof and ordered fixture.
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
cargo fmt -p jj-store-server -p jj-store-capture -- --check > /tmp/jj-store-agent-capture-m172-activation-rust-format.log 2>&1
```

**Notes**

Final affected Rust formatting passes.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-activation-doc-write.log 2>&1
```

**Notes**

Canonical chapters, verification and ledger formatting passes.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-activation-progress-check.log 2>&1
```

**Notes**

129 tasks validate; 282 entries preserve all 281 parent structures.
<!-- /run -->
<!-- /evidence-field -->

