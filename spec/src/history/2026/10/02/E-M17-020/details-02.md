---
schema: 1
id: E-M17-020
---

# E-M17-020 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-server --test capture_source admitted_codex_source_sanitizes_partitions_safe_gaps_and_resumes_offline_tail --locked -- --exact > /tmp/jj-store-agent-capture-m172-source-before.log 2>&1
```

**Notes**

Exact new regression establishes two unresolved imports of the missing source-driver module.
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

Repeated focused formatting after source/profile accessors, singleton-intent guard and fixtures; later test-only formatting uses cargo fmt -p jj-store-server.
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
cargo test -p jj-store-server --test capture_source admitted_codex_source_sanitizes_partitions_safe_gaps_and_resumes_offline_tail --locked -- --exact > /tmp/jj-store-agent-capture-m172-source-after.log 2>&1
```

**Notes**

Cold rule compilation finished in this attempt; fixture incorrectly searched the encoded payload for readable aliases. Corrected to inspect decoded sanitized bytes with all privacy/redaction tags retained.
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
cargo test -p jj-store-server --test capture_source --locked > /tmp/jj-store-agent-capture-m172-source-driver.log 2>&1
```

**Notes**

Obsolete concurrent cold-rule attempt was intentionally terminated after the encoded-payload fixture defect; no passing acceptance claimed for this run.
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
kill -TERM 89437 90407
```

**Notes**

Cleanup of these owned test processes: the first had already finished its assertion failure; the second received SIGTERM. No unrelated process was targeted.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_source --locked > /tmp/jj-store-agent-capture-m172-source-final-driver.log 2>&1
```

**Notes**

Two cases passed; the same encoded-byte assertion failed before the fixture correction.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_source admitted_codex_source_sanitizes_partitions_safe_gaps_and_resumes_offline_tail --locked -- --exact > /tmp/jj-store-agent-capture-m172-source-exact-final.log 2>&1
```

**Notes**

Decoded privacy and partition checks passed; the fixture then incorrectly counted both a message and a lifecycle event as transcript message chunks. Existing normalizer/WAL semantics confirm one new message chunk; strengthened separate two-row cursor and Idle-state assertions.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_source admitted_codex_source_sanitizes_partitions_safe_gaps_and_resumes_offline_tail --locked -- --exact > /tmp/jj-store-agent-capture-m172-source-exact-fixed.log 2>&1
```

**Notes**

Exact native-body/partition/gap/offline-tail regression passes with the pinned compiled scanner cache.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_source --locked > /tmp/jj-store-agent-capture-m172-source-final.log 2>&1
```

**Notes**

All three admitted-source pipeline cases pass: native sanitized backfill/partitions/safe gaps/offline tail and code heads; spool refusal before ack and unadmitted history; native write before source enrollment and pending restart withheld for directory-proof recovery.
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
cargo test -p jj-store-capture --test codex_activity --test source_registry --test source_registry_policy --test reader_progress --test codex_context --locked > /tmp/jj-store-agent-capture-m172-source-runtime.log 2>&1
```

**Notes**

All 39 direct activity, context, source/physical lease, registry and shared-cursor cases pass.
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
cargo test -p jj-store-server --test capture_authority --test capture_activity --test capture_admission --locked > /tmp/jj-store-agent-capture-m172-source-server.log 2>&1
```

**Notes**

All six direct actor, guarded/native-write, active and directory admission cases pass.
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
cargo check -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-source-check.log 2>&1
```

**Notes**

Initial affected/direct-consumer all-target check passes.
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
cargo check -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-source-check-final.log 2>&1
```

**Notes**

Final affected/direct-consumer all-target check passes.
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
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-source-clippy.log 2>&1
```

**Notes**

Initial strict all-target lint passes.
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
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-source-clippy-final.log 2>&1
```

**Notes**

Final strict all-target lint passes without suppression.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-source-format.log 2>&1
```

**Notes**

Rust and embedded-runtime formatting pass.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-source-format-final.log 2>&1
```

**Notes**

Final Rust and embedded-runtime formatting pass.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-source-boundaries.log 2>&1
```

**Notes**

17-package dependency boundaries pass; no dependency added.
<!-- /run -->
<!-- /evidence-field -->

