---
schema: 1
id: E-M17-075
---

# E-M17-075 details 2

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --test opencode_checkpoint --locked
```

**Notes**

Original missing checkpoint module/status method/metadata diagnostics; /tmp/jj-store-opencode-lifecycle-checkpoint-before.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --test opencode_checkpoint --locked
```

**Notes**

Both checkpoint regressions pass; /tmp/jj-store-opencode-lifecycle-checkpoint-after.log.
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
cargo test -p jj-store-server --test opencode_lifecycle --locked
```

**Notes**

Original absent lifecycle_step; /tmp/jj-store-opencode-lifecycle-before.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test opencode_lifecycle --locked
```

**Notes**

Initial recovery filter borrowed self across proof mutation; capture the source ID before filtering. /tmp/jj-store-opencode-lifecycle-after.log was subsequently replaced by its passing original two-test result.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test opencode_lifecycle --locked
```

**Notes**

Original two restart regressions pass; 25.38s. /tmp/jj-store-opencode-lifecycle-after.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test opencode_lifecycle --test opencode_source --test opencode_supervisor --test opencode_read_api --locked
```

**Notes**

20 selected tests pass (six initial lifecycle, five API, six source, three supervisor); /tmp/jj-store-opencode-lifecycle-native.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_supervisor::opencode::tests --test opencode_lifecycle --locked
```

**Notes**

One blocked-ingress failure; this name filter did not execute lifecycle integration tests. Kept exact stronger tail-content assertion. /tmp/jj-store-opencode-lifecycle-final.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test opencode_lifecycle --locked
```

**Notes**

Returning-witness fixture exceeded 4 MiB pending reservation (required 5433974 bytes); isolated multi-generation fixture now reserves 16 MiB. Pressure contract remains tested separately. /tmp/jj-store-opencode-lifecycle-complete.log.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --locked capture_supervisor::opencode::tests
```

**Notes**

Blocked-ingress test still failed before sanitizer reuse. Separate status/message sanitizer construction increased owned-step latency; share current-workspace sanitizer and release at idle. /tmp/jj-store-opencode-lifecycle-owned.log.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy -p jj-store-capture -p jj-store-server --all-targets --locked -- -D warnings
```

**Notes**

Optional typed state grew shared poll enum beyond strict size budget; boxed state (first diagnostic 248 bytes, follow-up 208 bytes) and ready batches preserve durable encoding without lint suppression. /tmp/jj-store-opencode-lifecycle-clippy.log; /tmp/jj-store-opencode-lifecycle-clippy-final.log.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
cargo check -p jj-store-capture -p jj-store-server --all-targets --locked
```

**Notes**

Explicit-value test helpers needed unboxing after bounded ready representation; repaired the four owning fixtures. /tmp/jj-store-opencode-lifecycle-check.log.
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
cargo clippy -p jj-store-capture -p jj-store-server --all-targets --locked -- -D warnings
```

**Notes**

Both affected packages/all targets pass strict lint with no production suppression; /tmp/jj-store-opencode-lifecycle-clippy-final-2.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test opencode_lifecycle --test opencode_source --test opencode_supervisor --locked
```

**Notes**

Final behavioral set: seven lifecycle, six source, three supervisor tests pass. Sanitizer reuse reduces lifecycle runtime to 13.01s. /tmp/jj-store-opencode-lifecycle-complete-final.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --locked capture_supervisor::opencode::tests -- --test-threads=1
```

**Notes**

Both exact-proof/blocked-publication owned-lane regressions pass after final bounded representation and sanitizer reuse. Extra status+two messages fixture uses 8 MiB while preserving the three-intent proof. Existing macOS linker unwind-size warning remains distinct. /tmp/jj-store-opencode-lifecycle-owned-final.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --lib --test opencode_checkpoint --test opencode_ingress --test opencode_snapshot --test reader_progress --test reader_publication --test codex_transcript --test codex_resume --test codex_recovery --test codex_import --test transcript_sanitization --locked
```

**Notes**

Selected shared reader checkpoint/sanitization/progress/publication and Codex consumers; final adds all direct boxed-poll consumers. Explicit child/helper process tests and ignored child entrypoints remain distinct. /tmp/jj-store-opencode-lifecycle-readers.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --lib --test opencode_checkpoint --test opencode_ingress --test opencode_snapshot --test reader_progress --test reader_publication --test codex_transcript --test codex_resume --test codex_recovery --test codex_import --test codex_context --test codex_activity --test codex_bound_source --test source_parking --test source_registry --test source_registry_import --test transcript_sanitization --locked
```

**Notes**

Selected shared reader checkpoint/sanitization/progress/publication and Codex consumers; final adds all direct boxed-poll consumers. Explicit child/helper process tests and ignored child entrypoints remain distinct. /tmp/jj-store-opencode-lifecycle-readers-final.log. 147 top-level tests and one explicit sanitizer child pass.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_source --test capture_admission --test capture_authority --test capture_recovery --locked
```

**Notes**

13 direct native Codex boundary tests pass; /tmp/jj-store-opencode-lifecycle-codex-consumers.log.
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
cargo fmt --all
```

**Notes**

After Rust edits, including final extraction/indirection and fixtures.
<!-- /run -->
<!-- /evidence-field -->

