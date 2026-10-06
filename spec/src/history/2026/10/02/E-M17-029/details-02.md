---
schema: 1
id: E-M17-029
---

# E-M17-029 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_reanchor current_directory_handoff_preserves_the_exact_backpressured_batch_after_old_anchor_revocation --locked -- --exact > /tmp/jj-store-agent-capture-m172-reanchor-before.log 2>&1
```

**Notes**

Before: missing CaptureParkedResumeStep/resume_current plus incorrect fixture IntegrationId reference; fixture now retains the actual integration from its acquired partition.
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
cargo fmt --all
```

**Notes**

Initial Rust formatting; only task files changed.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_reanchor current_directory_handoff_preserves_the_exact_backpressured_batch_after_old_anchor_revocation --locked -- --exact > /tmp/jj-store-agent-capture-m172-reanchor-after.log 2>&1
```

**Notes**

New implementation compiled; body assertion incorrectly inspected serialized payload bytes. Corrected test to inspect exact decoded observation payloads, preserving its body/secret assertions.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_reanchor current_directory_handoff_preserves_the_exact_backpressured_batch_after_old_anchor_revocation --locked -- --exact > /tmp/jj-store-agent-capture-m172-reanchor-payload.log 2>&1
```

**Notes**

Original regression passes: exact old boundary retained, A converted to omissions under B, separately sanitized B bodies and ordered native publication.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_reanchor --locked > /tmp/jj-store-agent-capture-m172-reanchor-edges.log 2>&1
```

**Notes**

Four initial handoff cases pass, including bounded preparation, partial context/Write, physical replacement and foreign spool.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor live_parked_session_hands_off_to_b_while_exact_a_intent_remains_withheld --locked -- --exact > /tmp/jj-store-agent-capture-m172-reanchor-supervisor.log 2>&1
```

**Notes**

New connected fixture incorrectly cloned NativeRequest. It now freshly binds a read session per exact native page/detail pair.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_reanchor race::append_during_each_admission_boundary_releases_acquisition_and_retains_the_frame --locked -- --exact > /tmp/jj-store-agent-capture-m172-reanchor-race.log 2>&1
```

**Notes**

New race fixture moved the cursor position before checking payloads. Corrected to borrow it.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_reanchor --test capture_supervisor --locked > /tmp/jj-store-agent-capture-m172-reanchor-connected.log 2>&1
```

**Notes**

Fixture cleanup temporarily removed an older test binding; restored the existing binding and removed only the unused new fixture variable.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_reanchor --test capture_supervisor --locked > /tmp/jj-store-agent-capture-m172-reanchor-connected-fixed.log 2>&1
```

**Notes**

Five handoff and nine connected supervisor cases pass, including admission snapshot changes before/after native Write and after temporary lease acquisition.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_reanchor progress:: --locked > /tmp/jj-store-agent-capture-m172-reanchor-progress-edges.log 2>&1
```

**Notes**

Two further cases pass: absent/regressive/same-position-changed progress and rebound physical spool retain frame; advanced progress reopens without reusing old raw batch.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --locked > /tmp/jj-store-agent-capture-m172-reanchor-runtime.log 2>&1
```

**Notes**

Full capture runtime passes: 86 parent cases plus launched privacy child. Shared complete-context reconstruction and EOF preparation compaction justify this runtime lane; no shared durable format changed.
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
cargo check -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-reanchor-check.log 2>&1
```

**Notes**

Affected capture/server packages and direct CLI consumers compile.
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
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-reanchor-clippy.log 2>&1
```

**Notes**

Inline optional current probe enlarged the Stage enum. Lazily boxed the single retained preparation; no lint allowance or weaker bound.
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
cargo fmt -p jj-store-capture -p jj-store-server
```

**Notes**

Affected Rust formatting after tests and lazy preparation allocation.
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
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-reanchor-clippy-box.log 2>&1
```

**Notes**

Strict affected/direct-consumer lint passes after lazy boxing.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-reanchor-boundaries.log 2>&1
```

**Notes**

All 17 package boundaries pass.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md > /tmp/jj-store-agent-capture-m172-reanchor-doc-format.log 2>&1
```

**Notes**

Owning canonical and verification documentation formatted.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_reanchor --test capture_supervisor --test capture_activation --test capture_parking --test capture_pipeline_worker --test capture_pipeline --test capture_resume --test capture_recovery --test capture_source --test capture_activity --test capture_authority --test capture_admission --locked > /tmp/jj-store-agent-capture-m172-reanchor-server-qualified.log 2>&1
```

**Notes**

All 50 directly affected native capture-server cases pass. This shared retained-reading/admission/supervisor surface justifies its complete direct binaries; fresh MemoryStore fixtures are not full-server cold qualification.
<!-- /run -->
<!-- /evidence-field -->

