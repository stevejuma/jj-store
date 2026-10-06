---
schema: 1
id: E-M17-036
---

# E-M17-036

<!-- evidence-field:id markdown -->
## Id

E-M17-036
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/codex_activity.rs
- crates/jj-store-capture/tests/codex_activity.rs
- crates/jj-store-server/src/capture_supervisor/source.rs
- crates/jj-store-server/tests/capture_supervisor.rs
- crates/jj-store-server/tests/capture_supervisor/rearming.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/capture_supervisor/rearming.rs#unadmitted_prefix_rewrite_rearms_without_admitting_rewritten_history
- crates/jj-store-server/tests/capture_supervisor/rearming.rs#unadmitted_truncation_rearms_without_admitting_rewritten_history
- crates/jj-store-capture/tests/codex_activity.rs#bounded_prefix_verification_detects_rewrites_before_reusing_activity
- crates/jj-store-capture/tests/codex_activity.rs#retryable_header_failure_preserves_the_original_activity_boundary
<!-- /evidence-field -->

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor rearming:: --locked > /tmp/jj-store-agent-capture-m172-rearming-before.log 2>&1
```

**Notes**

Two genuine before failures after 30 seconds: rewritten-prefix probe permanently unavailable with one raw frame, truncated probe never admits a later genuine start.
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

Affected Rust formatting.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor rearming:: --locked > /tmp/jj-store-agent-capture-m172-rearming-after.log 2>&1
```

**Notes**

Both exact rewrite/truncation regressions pass after conservative rearming; actual native sanitation/backfill, stable code heads and no historical admission.
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
cargo test -p jj-store-capture --test codex_activity retryable_header_failure_preserves_the_original_activity_boundary --locked -- --exact > /tmp/jj-store-agent-capture-m172-rearming-header.log 2>&1
```

**Notes**

Retryable header error retains the original cutoff; exact restoration recognizes a new start that arrived before recovery.
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
cargo test -p jj-store-capture --test codex_activity --locked > /tmp/jj-store-agent-capture-m172-rearming-activity-final.log 2>&1
```

**Notes**

All 12 core activity cases pass.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor --locked > /tmp/jj-store-agent-capture-m172-rearming-supervisor-final.log 2>&1
```

**Notes**

All 20 direct supervisor cases pass, including previous rotation/pressure/loss evidence.
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
cargo check -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-rearming-check-final.log 2>&1
```

**Notes**

Affected packages and direct CLI consumers compile.
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
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-rearming-clippy-final.log 2>&1
```

**Notes**

Affected packages and direct CLI consumers pass strict lint.
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
cargo fmt -p jj-store-capture -p jj-store-server -- --check > /tmp/jj-store-agent-capture-m172-rearming-rust-format-final.log 2>&1
```

**Notes**

Affected Rust formatting passes.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-rearming-doc-format-final.log 2>&1
```

**Notes**

Changed canonical documents and ledger formatted.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-rearming-doc-check-final.log 2>&1
```

**Notes**

Changed canonical documents and ledger pass formatting.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-rearming-validate-final.log 2>&1
```

**Notes**

Canonical specification validation passes.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-rearming-first-final.log 2>&1
```

**Notes**

Protected chapters accompany implementation.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-rearming-progress-final.log 2>&1
```

**Notes**

All 129 tasks validate; all prior 290 entries preserved and M17/M17.2 stay In progress.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64; synthetic Codex 0.156.1 CLI files, directory/socket admission and actual native MemoryStore publication. No real harness, RustFS or cold whole-server qualification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No persistence, dependency or public wire changes. Typed read-only terminal activity state adds no source identity, catalog mutation, lease or progress. Current-only AgentStore, source admission/cursors and exact padded pending requests/actors/receipts retain their encodings and authority.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change wvuznrxw
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

A same-physical/header inactive source no longer retains a permanently unavailable probe after verified prefix change or truncation. Only terminally invalidated unadmitted activity/preparation is dropped; the next reserved owned source step rechecks the exact qualification and arms at the current length. Starts already present then remain historical and arbitrary growth cannot enroll/ack/publish. A later genuine start independently admits sanitized backfill and one native session without changing code heads. Retryable metadata/header/I/O errors retain the original probe and cutoff; exact restored header can recover a new start observed since that original cutoff. Admitted readers and durable queues retain independent recovery; no pending raw admitted batch is discarded. Pause/resume and tracking/resident bounds remain exact. Both narrow regressions fail before implementation and pass after; initial canonical patch missed formatter-wrapped context and made no change, then was reapplied correctly. Complete activity and direct supervisor binaries are justified by shared proof-state and source lifecycle composition; owning packages and direct CLI consumers compile/lint. Other reader/publication/spool/model/UI/whole-Rust-workspace/root aggregate and real-storage lanes are out of scope under targeted policy. Production modules remain below 300 lines. All previous 290 evidence entries preserved. M17/M17.2 remain In progress. Remaining gates include admitted multi-physical generation switching, wider rotation/root rebinding, task-loss/deadline faults, compact spool capacity diagnostics, configured startup and ordered storage shutdown, Cursor/OpenCode readers, authenticated controls/coverage UI and real harness/cold acceptance. M17.3-M17.7 remain pending. Automatic startup stays guarded; only current AgentStore is supported. No installed binary/service, unrelated daemon restart, credential, checkpoint, live harness, deployment or publication changed.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


