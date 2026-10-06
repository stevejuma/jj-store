---
schema: 1
id: E-M17-068
---

# E-M17-068 details 2

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib accepted_import_resumes_tail_after_restart_without_a_second_command --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-restart-recovery-before.log 2>&1
```

**Notes**

28.03 seconds; 0 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib accepted_import_resumes_tail_after_restart_without_a_second_command --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-restart-recovery-after.log 2>&1
```

**Notes**

24.38 seconds; 1 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo check -p jj-store-server --all-targets --locked > /tmp/jj-store-agent-capture-m172-restart-recovery-check.log 2>&1
```

**Notes**

7.49 seconds; 0 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::scheduled_import::restart:: --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-restart-recovery-exact.log 2>&1
```

**Notes**

49.63 seconds; 6 top-level tests
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib process::tests::capture_factory::import_recovery:: --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-restart-recovery-foreground.log 2>&1
```

**Notes**

13.41 seconds; 1 top-level tests
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib process::tests::capture_factory::import_recovery:: --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-restart-recovery-foreground.log 2>&1
```

**Notes**

26.15 seconds; 1 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib process::tests::capture_factory::import_recovery:: --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-restart-recovery-foreground.log 2>&1
```

**Notes**

27.8 seconds; 1 top-level tests
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib process::tests::capture_factory::import_recovery:: --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-restart-recovery-foreground.log 2>&1
```

**Notes**

108.1 seconds; 1 top-level tests
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib process::tests::capture_factory::import_recovery:: --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-restart-recovery-foreground.log 2>&1
```

**Notes**

26.92 seconds; 1 top-level tests
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib process::tests::capture_factory::import_recovery:: --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-restart-recovery-foreground.log 2>&1
```

**Notes**

26.5 seconds; 1 top-level tests
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib process::tests::capture_factory::import_recovery:: --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-restart-recovery-foreground.log 2>&1
```

**Notes**

25.95 seconds; 1 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib process::tests::capture_factory::import_recovery:: --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-restart-recovery-foreground.log 2>&1
```

**Notes**

21.37 seconds; 2 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib process::tests::capture_factory::import_recovery:: --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-restart-recovery-foreground-final.log 2>&1
```

**Notes**

20.91 seconds; 2 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::scheduled_import::restart:: --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-restart-recovery-restart-final.log 2>&1
```

**Notes**

37.07 seconds; 6 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib paused_recovery_settles_while_management_holds_its_policy_gate --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-restart-recovery-gate-cancellation.log 2>&1
```

**Notes**

21.31 seconds; 1 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management:: --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-restart-recovery-management-library.log 2>&1
```

**Notes**

140.05 seconds; 50 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_supervisor:: --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-restart-recovery-supervisor-library.log 2>&1
```

**Notes**

49.94 seconds; 9 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_runtime:: --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-restart-recovery-runtime-library.log 2>&1
```

**Notes**

1.1 seconds; 4 top-level tests
<!-- /run -->
<!-- /evidence-field -->

