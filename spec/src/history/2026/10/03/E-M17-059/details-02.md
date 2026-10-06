---
schema: 1
id: E-M17-059
---

# E-M17-059 details 2

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
jj --no-pager st
```

**Notes**

Fresh oxowsltv empty after verified preparation commit 0b076b4a; no overlap.
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
jj describe -m "Guard import admission with current installation policy and retained gates"
```

**Notes**


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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::import_policy --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-import-policy-before.log 2>&1
```

**Notes**

Before capability: filter matches zero tests, 49 filtered. This is absent acceptance evidence, not a passing import proof or a genuine runtime regression.
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
cargo fmt --all
```

**Notes**

Repeated after complete guard/test modules and fixture API repair.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::import_policy --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-import-policy-after.log 2>&1
```

**Notes**

Two fixture compilation errors from calling control() instead of the actual controls() accessor; corrected without changing production contracts.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::import_policy --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-import-policy-repaired.log 2>&1
```

**Notes**

Seven policy tests pass. Existing macOS unwind-table linker warning unsuppressed.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::import_policy --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-import-policy-final-exact.log 2>&1
```

**Notes**

Seven pass after post-lookup authority recheck; same exact target before final binding assertions.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::import_policy --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-import-policy-bound-exact.log 2>&1
```

**Notes**

Seven exact policy regressions pass after adding configured-path/foreign installation/foreign generation assertions.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_runtime::control::tests::requested_pause_withholds_import_even_before_cached_observation_changes --locked -- --exact > /tmp/jj-store-agent-capture-m172-import-policy-pending-pause.log 2>&1
```
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-import-policy-server-lib.log 2>&1
```
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_management --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-import-policy-management.log 2>&1
```
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_runtime --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-import-policy-runtime.log 2>&1
```
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_control_state --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-import-policy-controls.log 2>&1
```
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_import_state --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-import-policy-imports.log 2>&1
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo check -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-import-policy-check.log 2>&1
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo clippy -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-import-policy-clippy.log 2>&1
```
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-import-policy-check-boundaries.log 2>&1
```

**Notes**

Repeated after metadata sealing.
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
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-import-policy-check-architecture.log 2>&1
```

**Notes**

Repeated after metadata sealing.
<!-- /run -->

<!-- run -->
### Run 19

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-import-policy-spec-validate.log 2>&1
```

**Notes**

Repeated after metadata sealing.
<!-- /run -->

<!-- run -->
### Run 20

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first > /tmp/jj-store-agent-capture-m172-import-policy-spec-first.log 2>&1
```

**Notes**

Repeated after metadata sealing.
<!-- /run -->
<!-- /evidence-field -->

