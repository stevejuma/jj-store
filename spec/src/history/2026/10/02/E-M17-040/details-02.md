---
schema: 1
id: E-M17-040
---

# E-M17-040 details 2

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
cargo test -p jj-store-server --test capture_runtime --locked > /tmp/jj-store-agent-capture-m172-runtime-before.log 2>&1
```

**Notes**

Before target cannot compile because the explicit capture_runtime API does not exist (two unresolved imports). This is API absence, not a behavioral test failure.
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

Affected production and tests formatted after each implementation/review change.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_runtime --locked > /tmp/jj-store-agent-capture-m172-runtime-after.log 2>&1
```

**Notes**

Two initial native runtime cases pass. Shared fixture has unused helpers in this binary; dead-code allowance is limited to that imported test module.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_runtime --locked > /tmp/jj-store-agent-capture-m172-runtime-matrix.log 2>&1
```

**Notes**

Four explicit-runtime cases pass: autonomous capture/pause/resume, unacknowledged raw shutdown, retained timeout/retry and dropped handle with blocked owner.
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
cargo check -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-runtime-check.log 2>&1
```

**Notes**

Owning server and direct CLI targets compile.
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
cargo clippy -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-runtime-lint.log 2>&1
```

**Notes**

Strict lint finds one collapsible_if in the new dispatch guard. Collapsed it; no lint or production rule suppressed.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_runtime --locked > /tmp/jj-store-agent-capture-m172-runtime-final.log 2>&1
```

**Notes**

Four exact native lifecycle cases pass after dispatch/stop ordering review.
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
cargo clippy -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-runtime-lint-final.log 2>&1
```

**Notes**

Owning and CLI targets pass strict lint after correction.
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
cargo test -p jj-store-server --lib capture_worker::executor --locked > /tmp/jj-store-agent-capture-m172-runtime-executor.log 2>&1
```

**Notes**

All eight underlying ownership/deadline/cancellation/task-loss executor cases pass; three unrelated library tests filtered.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_runtime --test capture_supervisor --locked > /tmp/jj-store-agent-capture-m172-runtime-supervisor-final.log 2>&1
```

**Notes**

All four final runtime and 27 complete supervisor cases pass at default concurrency; the existing backlog case exceeded 60 seconds but passed within its own preserved gate. With eight executor cases, 39 relevant native lifecycle cases pass.
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
cargo clippy -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-runtime-lint-sealed.log 2>&1
```

**Notes**

Final server and direct CLI target compilation and strict lint pass after terminal control/dispatch review.
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
cargo fmt --all -- --check > /tmp/jj-store-agent-capture-m172-runtime-rust-format.log 2>&1
```

**Notes**

Final Rust formatting passes.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-runtime-boundaries.log 2>&1
```

**Notes**

All 17 package boundaries pass.
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
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-runtime-architecture.log 2>&1
```

**Notes**

Architecture passes; changed behavioral modules have 162 and 293 lines, both under 300.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-runtime-doc-format.log 2>&1
```

**Notes**

Changed canonical chapters, verification and ledger formatted; repeated after final evidence sealing.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-runtime-spec-validate.log 2>&1
```

**Notes**

Specification passes five validators over 21 chapters and 160 requirements; repeated after sealing.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-runtime-spec-first.log 2>&1
```

**Notes**

Spec-first passes for five protected files and three canonical chapters; repeated after sealing.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-runtime-progress.log 2>&1
```

**Notes**

All 129 tasks validate, prior 294 entries preserved, total 295. M17/M17.2 remain In progress; repeated after sealing.
<!-- /run -->
<!-- /evidence-field -->

