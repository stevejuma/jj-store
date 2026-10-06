---
schema: 1
id: E-M17-055
---

# E-M17-055 details 2

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_management sources::source_pages_are_bounded_cached_and_hold_no_admission_proofs --locked -- --exact > /tmp/jj-store-agent-capture-m172-sources-before.log 2>&1
```

**Notes**

Genuine compiled before: 404 unknown_route rather than 200. Prior context retained.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_management sources::source_pages_are_bounded_cached_and_hold_no_admission_proofs --locked -- --exact > /tmp/jj-store-agent-capture-m172-sources-after.log 2>&1
```

**Notes**

Original source HTTP regression one to zero; unused import warning subsequently repaired.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_management sources::source_pages_are_bounded_cached_and_hold_no_admission_proofs --locked -- --exact > /tmp/jj-store-agent-capture-m172-sources-rerun.log 2>&1
```

**Notes**

Endpoint macro separates body and query; warning repaired.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-cli --test capture_cli sources::disabled_source_listing_requires_no_repository_or_capture_state --locked -- --exact > /tmp/jj-store-agent-capture-m172-cli-sources-before.log 2>&1
```

**Notes**

Genuine cold binary failure: unrecognized sources subcommand.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-cli --test capture_cli sources::disabled_source_listing_requires_no_repository_or_capture_state --locked -- --exact > /tmp/jj-store-agent-capture-m172-cli-sources-after.log 2>&1
```

**Notes**

Cold CLI original regression one to zero.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_management sources:: --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-sources-expanded.log 2>&1
```

**Notes**

Two expanded server integrations with original deadlines pass: cached enumeration, retirement without ordinal reuse, no admission/progress, bounds/stale query errors, current authority/config/identity/physical policy and disabled state preservation.
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
cargo fmt --all
```

**Notes**

Run during edits before focused checks.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_management sources::source_pages_are_bounded_cached_and_hold_no_admission_proofs --locked -- --exact > /tmp/jj-store-agent-capture-m172-sources-final.log 2>&1
```

**Notes**

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 1.21s
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::sources:: --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-sources-ownership.log 2>&1
```

**Notes**

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 44 filtered out; finished in 0.28s
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_runtime::sources::tests:: --locked > /tmp/jj-store-agent-capture-m172-sources-loss.log 2>&1
```

**Notes**

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 45 filtered out; finished in 0.00s
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_supervisor::sources::tests:: --locked > /tmp/jj-store-agent-capture-m172-sources-selector.log 2>&1
```

**Notes**

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 45 filtered out; finished in 0.00s
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-http --lib contract::capture_tests::capture_management_contract_is_installation_local_and_actor_free --locked -- --exact > /tmp/jj-store-agent-capture-m172-sources-http-contract.log 2>&1
```

**Notes**

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.07s
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-http --lib contract::capture_tests::capture_management_contract_is_installation_local_and_actor_free --locked -- --exact > /tmp/jj-store-agent-capture-m172-sources-http-contract-repair.log 2>&1
```

**Notes**

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.08s
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-cli --test capture_cli sources:: --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-cli-sources-expanded.log 2>&1
```

**Notes**

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 5.68s
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
pnpm exec prettier --write scripts/contracts.mjs spec/src/api.md spec/src/cli.md spec/src/collaboration.md spec/src/compatibility.md spec/src/mutations.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-sources-docs-format.log 2>&1
```

**Notes**

Initial focused metadata pass.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-sources-boundaries.log 2>&1
```

**Notes**

Initial focused metadata pass.
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
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-sources-architecture.log 2>&1
```

**Notes**

Initial focused metadata pass.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-sources-validate.log 2>&1
```

**Notes**

Initial focused metadata pass.
<!-- /run -->
<!-- /evidence-field -->

