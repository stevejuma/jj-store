---
schema: 1
id: E-M17-047
---

# E-M17-047

<!-- evidence-field:id markdown -->
## Id

E-M17-047
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.2
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-server/src/capture_runtime.rs
- crates/jj-store-server/src/capture_runtime/control.rs
- crates/jj-store-server/tests/capture_runtime.rs
- crates/jj-store-server/tests/capture_runtime/controls.rs
- crates/jj-store-server/tests/capture_runtime/draining.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/capture_runtime/controls.rs#shared_control_handles_follow_pause_resume_and_terminal_owner
- crates/jj-store-server/tests/capture_runtime/draining.rs#dropping_runtime_requests_stop_while_blocking_owner_retains_raw_channel
- crates/jj-store-server/src/capture_runtime/control.rs#closed_nonterminal_observation_refuses_stale_caught_up_and_controls
- crates/jj-store-server/src/capture_runtime/control.rs#completed_terminal_observation_remains_readable_with_closed_controls
<!-- /evidence-field -->

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
cargo fmt --all
```

**Notes**

Initial new control-consumer fixtures formatted.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_runtime controls::shared_control_handles_follow_pause_resume_and_terminal_owner --locked > /tmp/jj-store-agent-capture-m172-controls-before.log 2>&1
```

**Notes**

Before feature compilation has two missing controls() diagnostics in the new consumer and strengthened runtime-drop case. No product behavior failure was established; the shareable capability did not yet exist.
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

Pinned formatting after the narrow runtime/control ownership implementation.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_runtime controls::shared_control_handles_follow_pause_resume_and_terminal_owner --locked > /tmp/jj-store-agent-capture-m172-controls-after.log 2>&1
```

**Notes**

Exact original control-consumer target passes in 3.78 seconds. Shared pause/resume preserves historical eligibility, actual sanitized native MemoryStore publication and terminal refusal after sole-owner shutdown/drop.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --test capture_runtime --test capture_supervisor --locked > /tmp/jj-store-agent-capture-m172-controls-consumers.log 2>&1
```

**Notes**

63 direct entries pass: 28 owning-library, eight runtime, 27 supervisor. Four no-op helper registrations are not separate behavior tests. All 16 fresh-process factory cases run with independent cold scanner compilation (library 87.19 seconds). Runtime 24.16 seconds, supervisor 47.99 seconds.
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
cargo check -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-controls-check.log 2>&1
```

**Notes**

Affected server and direct CLI all-target compilation passes.
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
cargo clippy -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-controls-clippy.log 2>&1
```

**Notes**

Strict server/direct CLI all-target lint passes.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-controls-doc-format.log 2>&1
```

**Notes**

All four canonical documents formatted; repeated before final sealed checks.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-controls-rustfmt.log 2>&1
```

**Notes**

Pass; canonical/progress/document checks repeat after final sealing.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-controls-boundaries.log 2>&1
```

**Notes**

Pass; canonical/progress/document checks repeat after final sealing.
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
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-controls-architecture.log 2>&1
```

**Notes**

Pass; canonical/progress/document checks repeat after final sealing.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-controls-spec-validate.log 2>&1
```

**Notes**

Pass; canonical/progress/document checks repeat after final sealing.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-controls-spec-first.log 2>&1
```

**Notes**

Pass; canonical/progress/document checks repeat after final sealing.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-controls-progress.log 2>&1
```

**Notes**

Pass; canonical/progress/document checks repeat after final sealing.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-controls-doc-check.log 2>&1
```

**Notes**

Pass; canonical/progress/document checks repeat after final sealing.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64; synthetic Codex CLI 0.156.1; native MemoryStore, real private filesystem locks and explicit runtime capture. Closed-producer unit cases model watch-channel loss. No real RustFS or live harness result.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

New Rust-only CaptureRuntimeControl capability. Public/durable transcript, AgentStore, cursor, request and receipt formats unchanged. No HTTP/RPC/CLI/config shape or enablement override.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change nqwptmuy
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-047 details 2](details-02.md)
