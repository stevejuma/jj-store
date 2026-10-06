---
schema: 1
id: E-M17-041
---

# E-M17-041 details 2

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_runtime --locked > /tmp/jj-store-agent-capture-m172-storage-before.log 2>&1
```

**Notes**

Before target fails compilation: capture_lifetime/start_with_storage APIs absent (four diagnostics). No behavioral failure claimed.
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
cargo fmt --all
```

**Notes**

Initial formatting attempted before the new unit-test module existed. Added the module and reran formatting successfully; no formatter rule changed.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_runtime --locked > /tmp/jj-store-agent-capture-m172-storage-after.log 2>&1
```

**Notes**

Seven runtime entries pass (six main-process behavior tests plus no-op helper registration); storage entry dispatches five fresh-process cases: normal, retained timeout, drop, cancelled storage wait and late-attachment refusal. Native capture uses MemoryStore; external S3 lifetime client performs no server requests.
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
cargo test -p jj-store-server --lib storage::lifetime --locked > /tmp/jj-store-agent-capture-m172-storage-owners.log 2>&1
```

**Notes**

Four initial storage ownership targets pass against disposable mock embedded processes and actual private filesystem locks. One unused Path import warning was subsequently removed. Not real RustFS evidence.
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
cargo clippy -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-storage-lint.log 2>&1
```

**Notes**

Strict lint rejects the unused Path import. Removed it without suppressing lint.
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
cargo clippy -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-storage-lint-final.log 2>&1
```

**Notes**

Intermediate all-target lint passes. Final review then adds explicit state-before-storage destruction ordering and its exact completion/loss assertions, requiring rerun.
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
cargo test -p jj-store-server --lib storage::lifetime --locked > /tmp/jj-store-agent-capture-m172-storage-owners-final.log 2>&1
```

**Notes**

Added destructor-order fixture initially misses AtomicBool/Ordering imports (seven compiler diagnostics). Imported them; no product or assertion changed.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_runtime --test capture_supervisor --test capture_pipeline_worker --test configuration --test embedded_storage --locked > /tmp/jj-store-agent-capture-m172-storage-server-final.log 2>&1
```

**Notes**

Two pipeline-worker and seven runtime entries pass; supervisor has 26 pass and one independent-B delivery timeout at blocked.rs:170 under default suite concurrency. Configuration/embedded binaries were not reached. This intermediate build predates explicit RetainedWork destructor ordering; final exact and complete direct reruns required. No failure waived.
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
cargo test -p jj-store-server --lib storage::lifetime --locked > /tmp/jj-store-agent-capture-m172-storage-owners-sealed.log 2>&1
```

**Notes**

All five final ownership targets pass, including source-state destruction before storage release and unconsumed completion retention. Mock child/actual filesystem locks, two drop/loss modes and 128 final-release wakeup races are qualified. Eleven unrelated units filtered.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor --locked blocked::blocked_directory_keeps_its_owner_while_another_source_delivers -- --exact > /tmp/jj-store-agent-capture-m172-storage-blocked-exact.log 2>&1
```

**Notes**

The exact prior suite failure passes unchanged assertions and deadlines on the final RetainedWork implementation.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_runtime --test capture_supervisor --test capture_pipeline_worker --test configuration --test embedded_storage --locked > /tmp/jj-store-agent-capture-m172-storage-server-sealed.log 2>&1
```

**Notes**

Final default-concurrency direct suite passes two pipeline-worker, seven runtime entries (six main-process behaviors plus helper registration; five fresh-process storage cases), 27 supervisor and 16 configuration cases. Two real embedded-RustFS tests are ignored, never counted as passes. The earlier independent-B timeout is retained in evidence and resolved by exact and full direct reruns without changing its test or deadline.
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
cargo test -p jj-store-server --lib capture_worker::executor --locked > /tmp/jj-store-agent-capture-m172-storage-executor.log 2>&1
```

**Notes**

Eight complete executor fault/ownership/deadline cases pass. Eight other units filtered; not a full server library suite.
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
cargo fmt --all
```

**Notes**

Pinned Rust formatting succeeds after completing test registration and each subsequent review correction.
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
cargo fmt --all -- --check > /tmp/jj-store-agent-capture-m172-storage-rust-format.log 2>&1
```

**Notes**

Final Rust source and test formatting passes.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-storage-doc-format.log 2>&1
```

**Notes**

Changed canonical chapters, verification and ledger formatted; final evidence requires repeat sealing.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-storage-boundaries.log 2>&1
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
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-storage-architecture.log 2>&1
```

**Notes**

Architecture passes. Manually inspected changed behavioral Rust modules are 41 through 274 physical lines, each under 300; tests are also below 300.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-storage-spec-validate.log 2>&1
```

**Notes**

Five validators, 21 chapters and 160 requirements pass; final sealing rerun remains.
<!-- /run -->
<!-- /evidence-field -->

