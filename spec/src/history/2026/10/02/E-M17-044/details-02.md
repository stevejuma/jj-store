---
schema: 1
id: E-M17-044
---

# E-M17-044 details 2

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_supervisor::tests --locked > /tmp/jj-store-agent-capture-m172-lane-loss-before.log 2>&1
```

**Notes**

Initial matrix wrongly expected global-lane resume to succeed after task loss. It fails with the existing capture worker stopped result before reaching the delivery diagnostic target. Expanded the occupied test pool from one to four threads to leave normal nested registry I/O capacity after queue release; all four are occupied before fault injection.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_supervisor::tests --locked > /tmp/jj-store-agent-capture-m172-lane-loss-before-corrected.log 2>&1
```

**Notes**

Four-thread queued fault is deterministic, but the initial control expectation still incorrectly resumes a lost global lane. Corrected to require resume/dispatch refusal and preservation through idle control turns. No production code had changed.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_supervisor::tests --locked > /tmp/jj-store-agent-capture-m172-lane-loss-before-contract.log 2>&1
```

**Notes**

Genuine before behavior: discovery-loss case completes preservation/current replacement, but settled delivery loss reports unavailable zero instead of one. Exact assertion fails; queue bytes/cursors remain retained. No production logic was edited before this proof.
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
cargo fmt --all
```

**Notes**

Pinned formatting applied after test and narrow diagnostic edits.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_supervisor::tests --locked > /tmp/jj-store-agent-capture-m172-lane-loss-after.log 2>&1
```

**Notes**

Exact repaired two-case loss matrix passes. Each real queued-task abort closes the owner and counts unavailable once; exact accepted bytes/cursor survive control refusal and explicit current replacement drains the batch.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_supervisor::tests --locked > /tmp/jj-store-agent-capture-m172-lane-loss-current-admission.log 2>&1
```

**Notes**

Strengthened exact matrix passes both lanes with repository registration revoked during explicit replacement. Queue bytes, cursor and not-caught-up state remain unchanged until registration is restored; only fresh current admission can drain. No timeout/deadline or supported shape changed.
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
cargo check -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-lane-loss-check.log 2>&1
```

**Notes**

Affected server and direct CLI all-target checks pass before unit-fixture consolidation.
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
cargo clippy -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-lane-loss-clippy.log 2>&1
```

**Notes**

Strict lint rejects including the same fixture module twice in the server unit-test crate. Loaded it once at the cfg(test,unix) crate root and changed both consumers to imports; no lint waiver or fixture behavior change.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_supervisor::tests --locked > /tmp/jj-store-agent-capture-m172-lane-loss-fixture-final.log 2>&1
```

**Notes**

Exact two-lane loss/current-admission matrix passes after single shared unit-fixture selection. Final owning-library rerun checks the direct factory consumer as well.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-lane-loss-rustfmt.log 2>&1
```

**Notes**

Pinned main and embedded Rust formatting pass; final fixture move formatting applied by cargo fmt --all.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-lane-loss-boundaries.log 2>&1
```

**Notes**

17 package boundary audits pass.
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
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-lane-loss-architecture.log 2>&1
```

**Notes**

719 web module architecture audit passes.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-lane-loss-doc-format.log 2>&1
```

**Notes**

All four canonical documents formatted; final evidence sealing rerun remains.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-lane-loss-spec-validate.log 2>&1
```

**Notes**

Five validators, 21 chapters and 160 requirements pass.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-lane-loss-spec-first.log 2>&1
```

**Notes**

Four initial protected paths and three canonical chapters pass; final shared-fixture paths are checked at sealing.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-lane-loss-progress.log 2>&1
```

**Notes**

129 tasks and 299 evidence entries pass bookkeeping; previous 298 entries unchanged. Repeated after sealing.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --test capture_runtime --test capture_supervisor --locked > /tmp/jj-store-agent-capture-m172-lane-loss-server-final.log 2>&1
```

**Notes**

57 direct entries pass: 23 owning-library entries including the loss matrix, seven runtime and 27 supervisor entries. Three no-op helper registrations are not independent behavior tests. Complete library is rerun after the later single-fixture selection change.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --locked > /tmp/jj-store-agent-capture-m172-lane-loss-lib-final.log 2>&1
```

**Notes**

All 23 owning-library entries pass after single-fixture consolidation, including the exact loss/current-admission matrix and the 16-case fresh-process foreground factory. The factory still compiles its own cold scanner cache; full library completes in 99.24 seconds.
<!-- /run -->
<!-- /evidence-field -->

