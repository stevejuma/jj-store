---
schema: 1
id: E-M17-064
---

# E-M17-064 details 3

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-publication-capture-final.log 2>&1
```

**Notes**

Serial direct consumer scope; exact test details in /tmp/jj-store-agent-capture-m172-publication-capture-final.log
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::native_import --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-publication-native-import.log 2>&1
```

**Notes**

Serial direct consumer scope; exact test details in /tmp/jj-store-agent-capture-m172-publication-native-import.log
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::scheduled_import --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-publication-scheduled-import.log 2>&1
```

**Notes**

Serial direct consumer scope; exact test details in /tmp/jj-store-agent-capture-m172-publication-scheduled-import.log
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_source --test capture_resume --test capture_parking --test capture_reanchor --test capture_recovery --test capture_pipeline --test capture_pipeline_worker --test capture_supervisor --test capture_runtime --test capture_activation --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-publication-server-direct.log 2>&1
```

**Notes**

Serial direct consumer scope; exact test details in /tmp/jj-store-agent-capture-m172-publication-server-direct.log
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --test reader_publication --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-publication-exact-complete-final.log 2>&1
```

**Notes**

Serial direct consumer scope; exact test details in /tmp/jj-store-agent-capture-m172-publication-exact-complete-final.log
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --locked --lib reader_publication -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-publication-bounds-final-final.log 2>&1
```

**Notes**

Serial direct consumer scope; exact test details in /tmp/jj-store-agent-capture-m172-publication-bounds-final-final.log
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-publication-capture-final-final.log 2>&1
```

**Notes**

Serial direct consumer scope; exact test details in /tmp/jj-store-agent-capture-m172-publication-capture-final-final.log
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::native_import --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-publication-native-import-final.log 2>&1
```

**Notes**

Serial direct consumer scope; exact test details in /tmp/jj-store-agent-capture-m172-publication-native-import-final.log
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::scheduled_import --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-publication-scheduled-import-final.log 2>&1
```

**Notes**

Serial direct consumer scope; exact test details in /tmp/jj-store-agent-capture-m172-publication-scheduled-import-final.log
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_source --test capture_resume --test capture_parking --test capture_reanchor --test capture_recovery --test capture_pipeline --test capture_pipeline_worker --test capture_supervisor --test capture_runtime --test capture_activation --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-publication-server-direct-final.log 2>&1
```

**Notes**

Serial direct consumer scope; exact test details in /tmp/jj-store-agent-capture-m172-publication-server-direct-final.log
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-cli --test agent_capture_spool --test agent_capture_publication --test local_agent_recovery --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-publication-cli-direct-final.log 2>&1
```

**Notes**

Serial direct consumer scope; exact test details in /tmp/jj-store-agent-capture-m172-publication-cli-direct-final.log
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo check -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-publication-check-final.log 2>&1
```

**Notes**

Serial direct consumer scope; exact test details in /tmp/jj-store-agent-capture-m172-publication-check-final.log
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-publication-lint-final.log 2>&1
```

**Notes**

Serial direct consumer scope; exact test details in /tmp/jj-store-agent-capture-m172-publication-lint-final.log
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-publication-canonical.log 2>&1
```
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm spec:first > /tmp/jj-store-agent-capture-m172-publication-spec-first.log 2>&1
```
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-publication-boundaries.log 2>&1
```
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-publication-architecture.log 2>&1
```
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:progress > /tmp/jj-store-agent-capture-m172-publication-progress.log 2>&1
```
<!-- /run -->

<!-- run -->
### Run 19

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-publication-rust-format.log 2>&1
```
<!-- /run -->
<!-- /evidence-field -->

