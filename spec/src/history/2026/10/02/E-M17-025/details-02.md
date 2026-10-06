---
schema: 1
id: E-M17-025
---

# E-M17-025 details 2

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_pipeline_worker pipeline_work_runs_with_owned_resources_and_clean_shutdown --locked -- --exact > /tmp/jj-store-agent-capture-m172-worker-before.log 2>&1
```

**Notes**

Original exact native-owner regression fails because the capture_worker module/API is absent.
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
cargo fmt -p jj-store-server
```

**Notes**

Affected formatting repeated after owner, executor, blocked-I/O/control and native fixtures.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_pipeline_worker pipeline_work_runs_with_owned_resources_and_clean_shutdown --locked -- --exact > /tmp/jj-store-agent-capture-m172-worker-after.log 2>&1
```

**Notes**

Original exact native owner flow passes: sanitation/durability, one in-flight step, publication and source-lease release after clean shutdown.
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
cargo test -p jj-store-server --lib capture_worker::executor::tests --locked > /tmp/jj-store-agent-capture-m172-worker-executor.log 2>&1
```

**Notes**

Seven initial owned executor cases pass, including the real five-second shutdown wait on retained blocked I/O.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_pipeline_worker --locked > /tmp/jj-store-agent-capture-m172-worker-native.log 2>&1
```

**Notes**

Both actual native MemoryStore pipeline owner cases pass; pause/resume never overrides absent Write and no intent is acknowledged on failure.
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
cargo check -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-worker-check.log 2>&1
```

**Notes**

Affected server and direct CLI consumers compile.
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
cargo clippy -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-worker-clippy.log 2>&1
```

**Notes**

Affected/direct-consumer strict lint passes.
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
cargo test -p jj-store-server --lib capture_worker::executor::tests --locked > /tmp/jj-store-agent-capture-m172-worker-executor-final.log 2>&1
```

**Notes**

All eight final executor cases pass after adding dropped-controller retention. Single-thread Tokio tests use deterministic off-thread blocking gates to prove ordinary runtime responsiveness, overdue ownership and unchanged dispatch exclusivity. One-blocking-thread runtime proves expired queued work never enters its source step. Pause/resume, actual five-second shutdown timeout/retry, late known completion, task loss, invalid limits and drop settlement remain explicit.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_pipeline_worker --test capture_pipeline --test capture_resume --test capture_recovery --test capture_source --test capture_activity --test capture_authority --test capture_admission --locked > /tmp/jj-store-agent-capture-m172-worker-server-direct.log 2>&1
```

**Notes**

All 25 direct native owner/pipeline/source/resume/recovery/admission/activity/authority cases pass. Current Write, sanitized spool durability and real source/physical lease exclusion/release are exercised by the new facade consumers.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-cli --test agent_capture_publication --test agent_capture_spool --test local_agent_recovery --locked > /tmp/jj-store-agent-capture-m172-worker-cli-direct.log 2>&1
```

**Notes**

All 31 primary direct publication/shared spool/local recovery cases pass; two process fixtures execute through parents. Prepared retry, cancellation/unknown intent and native/local durability remain distinct from executor scheduling proofs.
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
cargo check -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-worker-check-final.log 2>&1
```

**Notes**

Final affected and direct-consumer all-target compile includes expanded drop/cursor assertions.
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
cargo clippy -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-worker-clippy-final.log 2>&1
```

**Notes**

Final affected/direct-consumer strict lint passes.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-worker-boundaries.log 2>&1
```

**Notes**

All 17 package boundaries pass; no new dependencies, schemas or upstream imports.
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
cargo fmt -p jj-store-server -- --check > /tmp/jj-store-agent-capture-m172-worker-rust-format.log 2>&1
```

**Notes**

Affected Rust formatting passes.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_pipeline_worker pipeline_work_runs_with_owned_resources_and_clean_shutdown --locked -- --exact > /tmp/jj-store-agent-capture-m172-worker-exact-final.log 2>&1
```

**Notes**

Original exact native-owner regression rerun passes after final lifecycle assertions.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-worker-doc-write.log 2>&1
```

**Notes**

Final canonical chapters, verification and evidence formatting passes.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-worker-progress.log 2>&1
```

**Notes**

129 tasks validate; 280 evidence entries preserve all 279 parent structures.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-worker-spec.log 2>&1
```

**Notes**

Five validators, 21 chapters and 160 requirements pass.
<!-- /run -->
<!-- /evidence-field -->

