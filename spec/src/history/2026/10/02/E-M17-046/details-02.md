---
schema: 1
id: E-M17-046
---

# E-M17-046 details 2

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

Pinned formatting applied to the initial independent rebinding fixture.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_supervisor::tests::rebinding --locked > /tmp/jj-store-agent-capture-m172-rebinding-before.log 2>&1
```

**Notes**

Three-case exact baseline qualification passes in 11.40 seconds. Existing production correctly rejects changed physical transcript qualification and rebound private source/spool state after exact proof recovery. No product before failure was demonstrated.
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

Pinned formatting after adding catalog disappearance and exact allocation restoration checks.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_supervisor::tests::rebinding --locked > /tmp/jj-store-agent-capture-m172-rebinding-catalog.log 2>&1
```

**Notes**

Strengthened four-case matrix passes in 15.60 seconds. Real catalog disappearance remains unavailable without resetting source allocation; restored original state permits fresh qualified recovery. Pending/source/cursor byte maps and native roots remain exact while withheld.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_supervisor::tests --locked > /tmp/jj-store-agent-capture-m172-rebinding-unit.log 2>&1
```

**Notes**

All four owning supervisor unit entries pass in 16.37 seconds, including one no-op running-loss subprocess helper. Existing queued/running lane fault matrices remain passing.
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
cargo fmt --all
```

**Notes**

Final independent fixture formatting after requiring exactly one recovered native event batch.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_supervisor::tests::rebinding --locked > /tmp/jj-store-agent-capture-m172-rebinding-final.log 2>&1
```

**Notes**

Final four-case matrix passes in 16.90 seconds, including exactly one native event batch on recovery. Exact bytes/cursors and sanitized content assertions preserved.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor --locked > /tmp/jj-store-agent-capture-m172-rebinding-supervisor.log 2>&1
```

**Notes**

All 27 direct connected supervisor entries pass in 47.30 seconds.
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
cargo check -p jj-store-server --all-targets --locked > /tmp/jj-store-agent-capture-m172-rebinding-check.log 2>&1
```

**Notes**

Owning server all-target compilation passes.
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
cargo clippy -p jj-store-server --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-rebinding-clippy.log 2>&1
```

**Notes**

Owning server strict all-target lint passes.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-rebinding-doc-format.log 2>&1
```

**Notes**

All four canonical documents formatted; repeated before final sealed checks.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-rebinding-rustfmt.log 2>&1
```

**Notes**

Pass; canonical, progress and document checks are repeated after final sealing.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-rebinding-boundaries.log 2>&1
```

**Notes**

Pass; canonical, progress and document checks are repeated after final sealing.
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
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-rebinding-architecture.log 2>&1
```

**Notes**

Pass; canonical, progress and document checks are repeated after final sealing.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-rebinding-spec-validate.log 2>&1
```

**Notes**

Pass; canonical, progress and document checks are repeated after final sealing.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-rebinding-spec-first.log 2>&1
```

**Notes**

Pass; canonical, progress and document checks are repeated after final sealing.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-rebinding-progress.log 2>&1
```

**Notes**

Pass; canonical, progress and document checks are repeated after final sealing.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-rebinding-doc-check.log 2>&1
```

**Notes**

Pass; canonical, progress and document checks are repeated after final sealing.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64; synthetic Codex CLI 0.156.1; native MemoryStore publication, synthetic directory verifier and real private filesystem mutations. Existing synthetic scanner cache. No real JJ binding, RustFS or live harness qualification.
<!-- /evidence-field -->

