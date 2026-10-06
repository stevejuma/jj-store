---
schema: 1
id: E-M17-069
---

# E-M17-069 details 2

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test configuration enabled_capture_startup_fails_before_state_creation_until_workers_are_connected --locked -- --exact --test-threads=1 > /tmp/jj-store-agent-capture-m172-startup-server-guard-before.log 2>&1
```

**Notes**

7.09 seconds; 1 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-cli --test server_executable cold_capture_config_check_and_startup_gate_preserve_private_state --locked -- --exact --test-threads=1 > /tmp/jj-store-agent-capture-m172-startup-cli-guard-before.log 2>&1
```

**Notes**

14.44 seconds; 1 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher JJ_STORE_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test -p jj-store-cli --test server_executable configured_capture::cold_configured_codex_capture_initializes_publishes_and_preserves_pause --locked -- --ignored --exact --test-threads=1 > /tmp/jj-store-agent-capture-m172-startup-acceptance-before.log 2>&1
```

**Notes**

12.09 seconds; 0 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher JJ_STORE_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test -p jj-store-cli --test server_executable configured_capture::cold_configured_codex_capture_initializes_publishes_and_preserves_pause --locked -- --ignored --exact --test-threads=1 > /tmp/jj-store-agent-capture-m172-startup-acceptance-before.log 2>&1
```

**Notes**

19.72 seconds; 0 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher JJ_STORE_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test -p jj-store-cli --test server_executable configured_capture::cold_configured_codex_capture_initializes_publishes_and_preserves_pause --locked -- --ignored --exact --test-threads=1 > /tmp/jj-store-agent-capture-m172-startup-acceptance-after.log 2>&1
```

**Notes**

26.7 seconds; 0 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher JJ_STORE_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test -p jj-store-cli --test server_executable configured_capture::cold_configured_codex_capture_initializes_publishes_and_preserves_pause --locked -- --ignored --exact --test-threads=1 > /tmp/jj-store-agent-capture-m172-startup-acceptance-retry.log 2>&1
```

**Notes**

24.48 seconds; 0 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher JJ_STORE_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test -p jj-store-cli --test server_executable configured_capture::cold_configured_codex_capture_initializes_publishes_and_preserves_pause --locked -- --ignored --exact --test-threads=1 > /tmp/jj-store-agent-capture-m172-startup-acceptance-retry-code.log 2>&1
```

**Notes**

11.45 seconds; 0 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher JJ_STORE_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test -p jj-store-cli --test server_executable configured_capture::cold_configured_codex_capture_initializes_publishes_and_preserves_pause --locked -- --ignored --exact --test-threads=1 > /tmp/jj-store-agent-capture-m172-startup-acceptance-repaired.log 2>&1
```

**Notes**

104.53 seconds; 0 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher pnpm spec:validate > /tmp/jj-store-agent-capture-m172-startup-governance-canonical.log 2>&1
```

**Notes**

2.68 seconds; 0 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher pnpm spec:first > /tmp/jj-store-agent-capture-m172-startup-governance-spec-first.log 2>&1
```

**Notes**

1.49 seconds; 0 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-startup-governance-boundaries.log 2>&1
```

**Notes**

1.29 seconds; 0 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher pnpm check:architecture > /tmp/jj-store-agent-capture-m172-startup-governance-architecture.log 2>&1
```

**Notes**

1.24 seconds; 0 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher pnpm check:progress > /tmp/jj-store-agent-capture-m172-startup-governance-progress.log 2>&1
```

**Notes**

2.37 seconds; 0 top-level tests
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher JJ_STORE_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test -p jj-store-cli --test server_executable configured_capture::cold_configured_codex_capture_initializes_publishes_and_preserves_pause --locked -- --ignored --exact --test-threads=1 > /tmp/jj-store-agent-capture-m172-startup-acceptance-route.log 2>&1
```

**Notes**

111.16 seconds; 0 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher JJ_STORE_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test -p jj-store-cli --test server_executable configured_capture::cold_configured_codex_capture_initializes_publishes_and_preserves_pause --locked -- --ignored --exact --test-threads=1 > /tmp/jj-store-agent-capture-m172-startup-acceptance-typed-control.log 2>&1
```

**Notes**

124.2 seconds; 1 top-level tests
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib process::tests::capture_factory::startup::server_capture_initializes_only_absent_state_and_preserves_existing_policy --locked -- --exact --test-threads=1 > /tmp/jj-store-agent-capture-m172-startup-exact-fresh-state.log 2>&1
```

**Notes**

15.85 seconds; 0 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib process::tests::capture_factory::startup::server_capture_initializes_only_absent_state_and_preserves_existing_policy --locked -- --exact --test-threads=1 > /tmp/jj-store-agent-capture-m172-startup-exact-fresh-state.log 2>&1
```

**Notes**

13.77 seconds; 1 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test configuration --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-startup-configuration.log 2>&1
```

**Notes**

13.28 seconds; 16 top-level tests
<!-- /run -->
<!-- /evidence-field -->

