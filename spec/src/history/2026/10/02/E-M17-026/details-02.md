---
schema: 1
id: E-M17-026
---

# E-M17-026 details 2

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_parking parked_pipeline_retains_verification_and_releases_leases --locked -- --exact > /tmp/jj-store-agent-capture-m172-parking-before.log 2>&1
```

**Notes**

Original exact large-prefix handoff regression fails because the park API is absent.
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

Affected Rust formatting repeated after implementation and fixture edits.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_parking parked_pipeline_retains_verification_and_releases_leases --locked -- --exact > /tmp/jj-store-agent-capture-m172-parking-after.log 2>&1
```

**Notes**

Incorrect service SourcePosition import failed compilation; moved monotonic cursor validation to its owning capture runtime.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_parking parked_pipeline_retains_verification_and_releases_leases --locked -- --exact > /tmp/jj-store-agent-capture-m172-parking-after-2.log 2>&1
```

**Notes**

Original handoff regression passes; remaining fixture helper warnings are exercised by expanded tests.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_parking --locked > /tmp/jj-store-agent-capture-m172-parking-expanded.log 2>&1
```

**Notes**

Three cases passed, one fixture body assertion inspected serialized byte arrays instead of sanitized payload bytes; corrected the assertion to decode actual payload bytes.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_parking --locked > /tmp/jj-store-agent-capture-m172-parking-expanded-2.log 2>&1
```

**Notes**

All four expanded handoff/admission/advanced-cursor/replacement cases pass with actual sanitized-body assertions.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_parking --locked > /tmp/jj-store-agent-capture-m172-parking-checkpoint.log 2>&1
```

**Notes**

Four cases pass after making parking infallible and retaining the reader actual acknowledged checkpoint, rather than borrowing a possibly advanced spool cursor.
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
cargo test -p jj-store-capture --test source_parking --locked > /tmp/jj-store-agent-capture-m172-parking-runtime.log 2>&1
```

**Notes**

Three low-level cases pass: exact raw Arc/batch retained across append and overlapping roots/hardlinks; foreign channel and identical physical/header replacement refused; failed attempts retain the frame.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_parking missing_or_regressive_durable_progress_never_resets_a_parked_reader --locked -- --exact > /tmp/jj-store-agent-capture-m172-parking-progress.log 2>&1
```

**Notes**

Actual published native spool journal loss, rollback and same-position changed batch identity are refused without reset; restored journal resumes once.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --locked > /tmp/jj-store-agent-capture-m172-parking-capture-suite.log 2>&1
```

**Notes**

80 runtime cases pass. Complete capture crate suite is required by shared leased-reader/progress API impact, with directly affected bound/source/registry/activity/context/resume/recovery/spool consumers; no root aggregate escalation.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_parking --test capture_pipeline_worker --test capture_pipeline --test capture_resume --test capture_recovery --test capture_source --test capture_activity --test capture_authority --test capture_admission --locked > /tmp/jj-store-agent-capture-m172-parking-server-direct.log 2>&1
```

**Notes**

All 30 directly affected native MemoryStore server cases pass, including five new parking cases and unchanged native ingress/publication/owner/resume/recovery policy.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-cli --test agent_capture_publication --test agent_capture_spool --test local_agent_recovery --locked > /tmp/jj-store-agent-capture-m172-parking-cli-direct.log 2>&1
```

**Notes**

All 31 primary direct native/local publication/shared-spool/recovery cases pass; process fixtures execute through their parent cases.
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
cargo check -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-parking-check.log 2>&1
```

**Notes**

Affected capture/server and direct CLI all-target consumers compile.
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
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-parking-clippy.log 2>&1
```

**Notes**

Strict affected and direct-consumer lint passes.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-parking-boundaries.log 2>&1
```

**Notes**

All 17 package boundaries pass; no added dependency or persistence/wire type.
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
cargo fmt -p jj-store-server -p jj-store-capture -- --check > /tmp/jj-store-agent-capture-m172-parking-rust-format.log 2>&1
```

**Notes**

Affected Rust formatting passes.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_parking parked_pipeline_retains_verification_and_releases_leases --locked -- --exact > /tmp/jj-store-agent-capture-m172-parking-exact-final.log 2>&1
```

**Notes**

Original exact large-prefix regression rerun passes after final checkpoint and corruption cases.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-parking-doc-write.log 2>&1
```

**Notes**

Canonical chapter, verification and ledger formatting passes.
<!-- /run -->
<!-- /evidence-field -->

