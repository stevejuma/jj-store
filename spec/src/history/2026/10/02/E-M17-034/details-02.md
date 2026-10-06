---
schema: 1
id: E-M17-034
---

# E-M17-034 details 2

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
cargo test -p jj-agent --test activity only_current_tail_pages_decode_and_retain_strong_prefix_dependencies --locked -- --exact > /tmp/jj-store-agent-capture-current-agent-model-before.log 2>&1
```

**Notes**

Before removal: the exact new rejection regression fails once because canonical forward-page bytes still decode.
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
cargo test -p jj-store-service --lib rejects_legacy_agent_descriptor_without_in_memory_upgrade --locked > /tmp/jj-store-agent-capture-current-agent-descriptor-before.log 2>&1
```

**Notes**

Before removal: the exact new rejection regression fails once because the version-1 descriptor is upgraded in memory.
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
cargo fmt -p jj-agent -p jj-store-wal -p jj-store-service -p jj-store-cli
```

**Notes**

Affected Rust source/test formatting.
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
cargo test -p jj-agent --test activity only_current_tail_pages_decode_and_retain_strong_prefix_dependencies --locked -- --exact > /tmp/jj-store-agent-capture-current-agent-model-after.log 2>&1
```

**Notes**

Exact model regression passes: old encoding rejected, current tail identity/dependencies retained.
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
cargo test -p jj-store-service --lib agent_descriptor::tests --locked > /tmp/jj-store-agent-capture-current-agent-descriptor-after.log 2>&1
```

**Notes**

Both exact descriptor regressions pass: version 1 rejected; current version/identity/canonical-byte checks retained.
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
cargo test -p jj-store-wal --test agent_catalog tail_append --locked > /tmp/jj-store-agent-capture-current-agent-tail-after.log 2>&1
```

**Notes**

Two current-tail regressions pass: bounded multi-stream append and refusal to infer ordering from unrelated source receipts.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/publication.md spec/src/mutations.md spec/src/local-jj.md spec/src/cli.md spec/src/verification.md > /tmp/jj-store-agent-capture-current-agent-doc-format.log 2>&1
```

**Notes**

Owning canonical chapters formatted.
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
cargo test -p jj-agent -p jj-store-wal --locked > /tmp/jj-store-agent-capture-current-agent-model-wal.log 2>&1
```

**Notes**

143 cases pass, no ignored cases. Full affected crate suites justified by shared encoding and native/local publication/read/copy consumers.
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
cargo test -p jj-store-service --test agent_sessions --test agent_relations --test agent_checkpoints --test local_agent --test local_agent_binding --test local_agent_pages --locked > /tmp/jj-store-agent-capture-current-agent-service.log 2>&1
```

**Notes**

26 direct service cases pass, including current-tail paging, companion reopen/GC, pinned history and exact receipts.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-cli --test native_lifecycle content_acceptance::http_collaboration::local_trusted_http_lists_root_pinned_agent_sessions_without_bearer_credentials --locked -- --exact > /tmp/jj-store-agent-capture-current-agent-http.log 2>&1
```

**Notes**

Initial invocation used the wrong module qualifier and matched zero cases; no qualification claimed. Corrected exact invocation follows.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-cli --test native_lifecycle http_collaboration_acceptance::local_trusted_http_lists_root_pinned_agent_sessions_without_bearer_credentials --locked -- --exact > /tmp/jj-store-agent-capture-current-agent-http-corrected.log 2>&1
```

**Notes**

Correct exact HTTP fixture passes with only current tail pages and unchanged public response shapes.
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
cargo test -p jj-store-cli --test agent_capture_spool --test agent_capture_publication --test local_agent_recovery --test agent_hook --locked > /tmp/jj-store-agent-capture-current-agent-cli.log 2>&1
```

**Notes**

34 direct CLI spool, native publication, local recovery and hook cases pass; two ignored subprocess entry fixtures are invoked by their parent cases. Durable retry compatibility remains independently supported.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor --locked > /tmp/jj-store-agent-capture-current-agent-supervisor.log 2>&1
```

**Notes**

All 14 current supervisor cases pass with actual native MemoryStore publication, exact queues and source-owner loss/recovery.
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
cargo check -p jj-agent -p jj-store-wal -p jj-store-service -p jj-store-cli -p jj-store-server --all-targets --locked > /tmp/jj-store-agent-capture-current-agent-check.log 2>&1
```

**Notes**

Affected packages and direct consumers compile across all targets.
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
cargo clippy -p jj-agent -p jj-store-wal -p jj-store-service -p jj-store-cli -p jj-store-server --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-current-agent-clippy.log 2>&1
```

**Notes**

Same targets pass strict lint.
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
cargo fmt -p jj-agent -p jj-store-wal -p jj-store-service -p jj-store-cli -- --check > /tmp/jj-store-agent-capture-current-agent-rust-format.log 2>&1
```

**Notes**

Affected formatting passes. Behavioral modules are below 300 lines; inherited publication facade only removes the obsolete re-export without implementation growth.
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
cargo run --quiet --locked -p jj-store-http --example openapi > /tmp/jj-store-agent-capture-current-agent-openapi.json 2> /tmp/jj-store-agent-capture-current-agent-openapi.log
```

**Notes**

Generated public OpenAPI for direct contract comparison.
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
python3 compare generated OpenAPI to schemas/openapi.json > /tmp/jj-store-agent-capture-current-agent-contract-compare.log 2>&1
```

**Notes**

Generated OpenAPI is identical to the checked-in contract; no public response/schema generation changes.
<!-- /run -->
<!-- /evidence-field -->

