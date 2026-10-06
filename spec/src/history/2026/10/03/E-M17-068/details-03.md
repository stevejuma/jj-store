---
schema: 1
id: E-M17-068
---

# E-M17-068 details 3

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_worker::executor:: --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-restart-recovery-executor-library.log 2>&1
```

**Notes**

5.8 seconds; 9 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib process::tests::capture_factory::controls:: --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-restart-recovery-factory-controls.log 2>&1
```

**Notes**

2.12 seconds; 2 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib process::tests::capture_factory::management:: --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-restart-recovery-factory-management.log 2>&1
```

**Notes**

0.68 seconds; 2 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-restart-recovery-supervisor-consumer.log 2>&1
```

**Notes**

224.97 seconds; 27 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_runtime --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-restart-recovery-runtime-consumer.log 2>&1
```

**Notes**

64.2 seconds; 10 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_source --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-restart-recovery-source-consumer.log 2>&1
```

**Notes**

31.84 seconds; 4 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_management --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-restart-recovery-management-http.log 2>&1
```

**Notes**

19.26 seconds; 13 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-cli --test capture_import_cli --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-restart-recovery-native-cli.log 2>&1
```

**Notes**

31.79 seconds; 1 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo check -p jj-store-server --all-targets --locked > /tmp/jj-store-agent-capture-m172-restart-recovery-check-final.log 2>&1
```

**Notes**

4.86 seconds; 0 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo clippy -p jj-store-server --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-restart-recovery-lint-final.log 2>&1
```

**Notes**

18.14 seconds; 0 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher pnpm exec prettier --write spec/src/api.md spec/src/cli.md spec/src/collaboration.md spec/src/compatibility.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-restart-recovery-governance-format-docs.log 2>&1
```

**Notes**

3.13 seconds; 0 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher pnpm spec:validate > /tmp/jj-store-agent-capture-m172-restart-recovery-governance-canonical.log 2>&1
```

**Notes**

2.21 seconds; 0 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher pnpm spec:first > /tmp/jj-store-agent-capture-m172-restart-recovery-governance-spec-first.log 2>&1
```

**Notes**

1.55 seconds; 0 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-restart-recovery-governance-boundaries.log 2>&1
```

**Notes**

1.26 seconds; 0 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher pnpm check:architecture > /tmp/jj-store-agent-capture-m172-restart-recovery-governance-architecture.log 2>&1
```

**Notes**

0.77 seconds; 0 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher pnpm check:progress > /tmp/jj-store-agent-capture-m172-restart-recovery-governance-progress.log 2>&1
```

**Notes**

2.4 seconds; 0 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-restart-recovery-governance-rust-format.log 2>&1
```

**Notes**

2.98 seconds; 0 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher pnpm exec prettier --check spec/src/api.md spec/src/cli.md spec/src/collaboration.md spec/src/compatibility.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-restart-recovery-governance-doc-check.log 2>&1
```

**Notes**

2.62 seconds; 0 top-level tests
<!-- /run -->
<!-- /evidence-field -->

