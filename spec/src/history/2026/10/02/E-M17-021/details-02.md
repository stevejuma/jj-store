---
schema: 1
id: E-M17-021
---

# E-M17-021 details 2

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_recovery queued_other_repository_recovers_exact_context_after_restart --locked -- --exact > /tmp/jj-store-agent-capture-m172-recovery-before.log 2>&1
```

**Notes**

New exact native regression fails at the deliberate unresolved pending-directory recovery guard after a queued repository-B batch from a repository-A header.
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

Focused formatting after initial implementation; repeated after fixtures.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_recovery queued_other_repository_recovers_exact_context_after_restart --locked -- --exact > /tmp/jj-store-agent-capture-m172-recovery-after.log 2>&1
```

**Notes**

Initial exact repository-change recovery regression passes.
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
cargo fmt -p jj-store-capture -p jj-store-server
```

**Notes**

Intermediate formatter rejects a test-only inner Unix cfg attribute appended after items. Moved it to the file header without changing qualification.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_recovery --test capture_source --locked > /tmp/jj-store-agent-capture-m172-recovery-server.log 2>&1
```

**Notes**

Intermediate compilation rejects the misplaced Unix test cfg attribute. Corrected at the header.
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
cargo test -p jj-store-capture --test codex_recovery --locked > /tmp/jj-store-agent-capture-m172-recovery-runtime.log 2>&1
```

**Notes**

Long-prefix fixture mistakenly changed the first x in the Codex header. Restricted the rewrite to bytes after the qualified first line, retaining the header and prefix-change assertions.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_recovery --test capture_source --locked > /tmp/jj-store-agent-capture-m172-recovery-server-fixed.log 2>&1
```

**Notes**

Fixtures attempted receipt replay through a Read binding and expected an error rather than the explicit Withheld source-replacement result. Corrected to Write and exact Withheld, retaining unchanged-queue checks.
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
cargo fmt -p jj-store-cli -p jj-store-capture -p jj-store-server
```

**Notes**

Focused formatting repeated after exact-entry drain, tests, resumed-reader safeguard and lint correction.
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
cargo test -p jj-store-capture --test codex_recovery --locked > /tmp/jj-store-agent-capture-m172-recovery-runtime-fixed.log 2>&1
```

**Notes**

Four original proof cases pass, including bounded multi-step verification and fresh-process admitted-channel proof recovery; child fixture is invoked by its parent.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_recovery --test capture_source --locked > /tmp/jj-store-agent-capture-m172-recovery-server-final.log 2>&1
```

**Notes**

Three initial native recovery and four admitted pipeline cases pass.
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
cargo test -p jj-store-cli --test agent_capture_publication exact_queued_entry_drain_preserves_other_batches_and_prepared_retry --locked -- --exact > /tmp/jj-store-agent-capture-m172-recovery-exact.log 2>&1
```

**Notes**

Exact selection regression passes: later selected batch alone prepares, unknown outcome retains intent, changed expected capture is refused and retry uses identical prepared bytes.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_recovery multiple_queued_partitions_recover_in_source_order_before_new_reading --locked -- --exact > /tmp/jj-store-agent-capture-m172-recovery-multiple.log 2>&1
```

**Notes**

Initial multiple-queued-partition case passes; strengthened below to retain a live original driver and prove cursor resumption after independently queued progress.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_recovery --test capture_source --test capture_activity --test capture_authority --test capture_admission --locked > /tmp/jj-store-agent-capture-m172-recovery-server-direct.log 2>&1
```

**Notes**

All 14 direct native admission/activity/authority/pipeline/recovery cases pass. Strengthened multi-partition test drains A/B/A in sequence and reads new tail only after reopening at the latest durable cursor.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --lib --tests --locked > /tmp/jj-store-agent-capture-m172-recovery-runtime-direct.log 2>&1
```

**Notes**

70 primary runtime cases pass before the additive Desktop/incomplete-ending case; subprocess fixtures run from their parents. Complete direct context, reader, registry, cursor and sanitizer consumers qualify the shared prefix callback and lease-preserving resume.
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
cargo test -p jj-store-cli --test agent_capture_publication --test agent_capture_spool --test local_agent_recovery --locked > /tmp/jj-store-agent-capture-m172-recovery-cli-direct.log 2>&1
```

**Notes**

26 direct CLI publication/spool/local recovery cases pass; subprocess fixtures are explicitly invoked by parent cases.
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
cargo test -p jj-store-capture --test codex_recovery --locked > /tmp/jj-store-agent-capture-m172-recovery-runtime-profiles.log 2>&1
```

**Notes**

All five final proof cases pass, including independent Desktop 0.159.2 versus CLI 0.156.1 qualification and rejection of a recomputed identity ending before a complete newline. Together with the direct run this proves 71 distinct primary runtime cases.
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
cargo check -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-recovery-check.log 2>&1
```

**Notes**

Affected runtime and direct native/CLI consumers compile in all targets.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-recovery-clippy.log 2>&1
```

**Notes**

Strict lint rejects nested ready-state condition; collapsed it without changing candidate revalidation or ready semantics.
<!-- /run -->
<!-- /evidence-field -->

