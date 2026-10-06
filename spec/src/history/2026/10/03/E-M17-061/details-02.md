---
schema: 1
id: E-M17-061
---

# E-M17-061 details 2

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

myrxntuv empty atop verified E-M17-060 commit a04d633b.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
jj describe -m 'Retain exact import settlement gates across native admission awaits'
```
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::import_settlement --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-import-settlement-before.log 2>&1
```

**Notes**

Absent acceptance before implementation: zero matches, 57 filtered. Not a passing functional proof.
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

Repeated after final owned-descriptor release order.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo check -p jj-store-server --lib --locked > /tmp/jj-store-agent-capture-m172-import-settlement-initial-check.log 2>&1
```
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::import_settlement --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-import-settlement-exact.log 2>&1
```

**Notes**

Fixture compilation omitted the A repository constant import; also one unused type import warning. Imported the fixture constant and removed unused import without changing assertions or production behavior.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::import_settlement --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-import-settlement-exact-repaired.log 2>&1
```

**Notes**

Five exact regressions pass, including actual native Write authorization across an await with both private gates held. No source enrollment or body publication claimed.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::import_settlement --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-import-settlement-exact-final.log 2>&1
```

**Notes**

Five exact tests pass after final descriptor-before-manager release ordering.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-import-settlement-format.log 2>&1
```

**Notes**

Repeated after final ledger sealing.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-import-settlement-spec-validate.log 2>&1
```

**Notes**


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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-import-settlement-spec-first.log 2>&1
```

**Notes**


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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-import-settlement-boundaries.log 2>&1
```

**Notes**


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
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-import-settlement-architecture.log 2>&1
```

**Notes**


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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-import-settlement-progress.log 2>&1
```

**Notes**


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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-import-settlement-rust-format.log 2>&1
```

**Notes**


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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-import-settlement-doc-check.log 2>&1
```

**Notes**


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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --test capture_management --test capture_runtime --test capture_control_state --test capture_import_state --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-import-settlement-direct.log 2>&1
```

**Notes**

Complete direct suites pass 62 library, ten management, ten runtime, nine control journal and eight import journal tests: 99 total. Original deadlines and within-scenario concurrency retained. Cold factory/runtime/storage proofs pass; no live harness or real RustFS qualification claimed.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo check -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-import-settlement-check.log 2>&1
```

**Notes**


<!-- /run -->
<!-- /evidence-field -->

