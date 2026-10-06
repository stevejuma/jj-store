---
schema: 1
id: E-M17-067
---

# E-M17-067 details 3

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib import_waits_for_existing_raw_active_reader_and_keeps_the_same_session_identity --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-public-import-handoff-after.log 2>&1
```

**Notes**

4.24 seconds; 1 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib multi_step_import_prefix_survives_worker_parking_without_starving_ingress --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-public-import-parking-after.log 2>&1
```

**Notes**

11.2 seconds; 0 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib multi_step_import_prefix_survives_worker_parking_without_starving_ingress --locked -- --test-threads=1 --nocapture > /tmp/jj-store-agent-capture-m172-public-import-parking-diagnostic.log 2>&1
```

**Notes**

23.64 seconds; 0 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib multi_step_import_prefix_survives_worker_parking_without_starving_ingress --locked -- --test-threads=1 --nocapture > /tmp/jj-store-agent-capture-m172-public-import-parking-diagnostic.log 2>&1
```

**Notes**

23.8 seconds; 0 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --test codex_import same_snapshot_reader_verification_extends_prefix_without_rescanning_earlier_bytes --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-public-import-extension-before.log 2>&1
```

**Notes**

5.31 seconds; 0 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --test codex_import reader:: --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-public-import-extension-after.log 2>&1
```

**Notes**

7.12 seconds; 3 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib multi_step_import_prefix_survives_worker_parking_without_starving_ingress --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-public-import-parking-fixed.log 2>&1
```

**Notes**

23.36 seconds; 1 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --test reader_publication --test reader_progress --test codex_import --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-public-import-capture-final.log 2>&1
```

**Notes**

16.71 seconds; 50 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_import_state:: --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-public-import-journal-final.log 2>&1
```

**Notes**

1.33 seconds; 8 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::native_import:: --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-public-import-native-final.log 2>&1
```

**Notes**

53.63 seconds; 15 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::scheduled_import:: --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-public-import-scheduled-final.log 2>&1
```

**Notes**

47.15 seconds; 9 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::import_policy:: --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-public-import-policy-final.log 2>&1
```

**Notes**

1.7 seconds; 7 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_management --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-public-import-http-final.log 2>&1
```

**Notes**

33.75 seconds; 13 top-level tests
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
cargo fmt --all
```

**Notes**

2.09 seconds
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-cli --test capture_import_cli --test capture_cli --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-public-import-cli-final.log 2>&1
```

**Notes**

47.43 seconds; 13 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_activation --test capture_runtime --test capture_source --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-public-import-cold-consumers.log 2>&1
```

**Notes**

94.12 seconds; 18 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo check -p jj-store-capture -p jj-store-server -p jj-store-service -p jj-store-http -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-public-import-check-final.log 2>&1
```

**Notes**

26.78 seconds; 0 top-level tests
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-service -p jj-store-http -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-public-import-lint-final.log 2>&1
```

**Notes**

27.8 seconds; 0 top-level tests
<!-- /run -->
<!-- /evidence-field -->

