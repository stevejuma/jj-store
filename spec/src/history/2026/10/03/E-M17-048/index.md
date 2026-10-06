---
schema: 1
id: E-M17-048
---

# E-M17-048

<!-- evidence-field:id markdown -->
## Id

E-M17-048
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-03
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
- crates/jj-store-server/tests/capture_runtime/controls.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/capture_runtime/controls.rs#runtime_handoff_preserves_settled_pause_until_explicit_resume
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

Pinned formatting for exact paused-handoff regression.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_runtime controls::runtime_handoff_preserves_settled_pause_until_explicit_resume --locked > /tmp/jj-store-agent-capture-m172-paused-handoff-before.log 2>&1
```

**Notes**

Genuine behavior regression: constructor starts a paused supervisor with Running command/applied state, attempts refused dispatch and terminates. Exact assertion observes Stopped instead of Paused after 80 ms; one functional failure, not fixture compilation.
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

Pinned formatting after initializing the existing command/driver from settled supervisor pause.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_runtime controls::runtime_handoff_preserves_settled_pause_until_explicit_resume --locked > /tmp/jj-store-agent-capture-m172-paused-handoff-after.log 2>&1
```

**Notes**

Exact original regression passes in 6.75 seconds. Paused runtime remains usable without discovery/enrollment/publication. Resume arms a historical boundary; only a new later start admits native MemoryStore sanitized backfill.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --test capture_runtime --locked > /tmp/jj-store-agent-capture-m172-paused-handoff-consumers.log 2>&1
```

**Notes**

All 37 direct entries pass: 28 library (94.60 seconds) and nine runtime (25.57 seconds). Four no-op helpers are not independent behavior tests. All 16 fresh-process cold factory cases compile their private scanner cache; foreground lock and complete direct runtime consumers pass.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_runtime controls::runtime_handoff_preserves_settled_pause_until_explicit_resume --locked > /tmp/jj-store-agent-capture-m172-paused-handoff-final.log 2>&1
```

**Notes**

Final exact case passes in 3.83 seconds with explicit zero tracked/observed source assertions during initial pause. This strengthens only the leaf regression; direct consumers already pass the unchanged production implementation.
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
cargo check -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-paused-handoff-check.log 2>&1
```

**Notes**

Affected server/direct CLI all-target compilation passes.
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
cargo clippy -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-paused-handoff-clippy.log 2>&1
```

**Notes**

Final strict server/direct CLI all-target lint passes after zero-source assertion strengthening.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-paused-handoff-doc-format.log 2>&1
```

**Notes**

All four canonical documents formatted; repeated before final sealing checks.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-paused-handoff-rustfmt.log 2>&1
```

**Notes**

Pass; Rust/document/canonical/progress checks repeat after final sealing.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-paused-handoff-boundaries.log 2>&1
```

**Notes**

Pass; Rust/document/canonical/progress checks repeat after final sealing.
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
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-paused-handoff-architecture.log 2>&1
```

**Notes**

Pass; Rust/document/canonical/progress checks repeat after final sealing.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-paused-handoff-spec-validate.log 2>&1
```

**Notes**

Pass; Rust/document/canonical/progress checks repeat after final sealing.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-paused-handoff-spec-first.log 2>&1
```

**Notes**

Pass; Rust/document/canonical/progress checks repeat after final sealing.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-paused-handoff-progress.log 2>&1
```

**Notes**

Pass; Rust/document/canonical/progress checks repeat after final sealing.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-paused-handoff-doc-check.log 2>&1
```

**Notes**

Pass; Rust/document/canonical/progress checks repeat after final sealing.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64; synthetic Codex CLI 0.156.1 and native MemoryStore. Real private filesystem state. No real RustFS/live harness acceptance.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-048 details 2](details-02.md)
