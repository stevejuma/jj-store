---
schema: 1
id: E-M17-033
---

# E-M17-033

<!-- evidence-field:id markdown -->
## Id

E-M17-033
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

- crates/jj-store-server/tests/capture_supervisor/task_loss.rs
- crates/jj-store-server/tests/capture_supervisor.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/capture_supervisor/task_loss.rs#source_task_loss_retains_exact_queue_and_requires_renewed_current_admission
<!-- /evidence-field -->

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor task_loss::source_task_loss_retains_exact_queue_and_requires_renewed_current_admission --locked -- --exact > /tmp/jj-store-agent-capture-m172-task-loss-exact.log 2>&1
```

**Notes**

Initial fixture wrongly expected supervisor resume to reject all lanes after one source closes; one assertion failed. Healthy lanes may resume while the lost source stays closed.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor task_loss::source_task_loss_retains_exact_queue_and_requires_renewed_current_admission --locked -- --exact > /tmp/jj-store-agent-capture-m172-task-loss-corrected.log 2>&1
```

**Notes**

Corrected exact case passes; eight additional turns prove no source read, cursor advancement or queued-byte change after resume.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor --locked > /tmp/jj-store-agent-capture-m172-task-loss-supervisor.log 2>&1
```

**Notes**

14 direct supervisor cases pass. Full binary justified by shared registered harness and source lifecycle.
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
cargo check -p jj-store-server --test capture_supervisor --locked > /tmp/jj-store-agent-capture-m172-task-loss-check.log 2>&1
```

**Notes**

Focused server target checks.
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
cargo clippy -p jj-store-server --test capture_supervisor --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-task-loss-clippy.log 2>&1
```

**Notes**

Focused target passes strict lint.
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
cargo fmt -p jj-store-server -- --check > /tmp/jj-store-agent-capture-m172-task-loss-rust-format.log 2>&1
```

**Notes**

Affected Rust formatting passes.
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
python3 task-loss canonical prose insertion
```

**Notes**

CLI sentence corrected, then verification insertion assertion failed on wrapped context. Verification paragraph subsequently inserted using actual context; no product failure.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md > /tmp/jj-store-agent-capture-m172-task-loss-doc-format.log 2>&1
```

**Notes**

Canonical formatter passed; repeated after verification and ledger additions.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-task-loss-final-format.log 2>&1
```

**Notes**

Final canonical/ledger formatting.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-task-loss-validate.log 2>&1
```

**Notes**

5 validators, 21 chapters, 160 requirements pass.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-task-loss-first.log 2>&1
```

**Notes**

2 protected files and 3 chapters pass same-change gate.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-task-loss-progress.log 2>&1
```

**Notes**

129 tasks and 288 preserved evidence entries pass bookkeeping.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-task-loss-doc-check.log 2>&1
```

**Notes**

All changed docs pass formatting.
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

Rust 1.98.1; JJ 0.45.1; macOS ARM64. Synthetic Codex CLI 0.156.1 transcript and directory/socket admission over actual native MemoryStore publication; no RustFS or live harness.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No production, dependency, AgentStore, cursor, spool, request, receipt or wire format changed. Exact physical queued bytes and sanitized source cursor preserved.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change lwrwwtzt
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Qualifies owned source verifier task loss with unpublished exact A evidence, current A revocation and unread current B tail. Lost source reports no running/resident frames or caught-up state across repeated turns and pause/resume; healthy lanes may resume. Explicit replacement renews current retained-session/channel/directory/catalog/Write admission before recovery and publishes A/B separately with sanitization after restored catalogue. Initial mistaken fixture assertion and canonical insertion failure are retained. All previous 287 evidence entries preserved. M17/M17.2 stay In progress. Discovery/delivery task loss, rotation/retirement/requalification, deadlines, startup/ordered storage shutdown, other readers, authenticated controls/UI and live/cold acceptance remain pending. User has now dropped legacy AgentStore compatibility and absent-domain initialization from supported scope; removal is the next slice. Compact durable spool recovery remains independent and supported. Root aggregates, unrelated UI/crates and real storage are out of scope. Automatic enabled startup stays guarded.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


