---
schema: 1
id: E-M17-053
---

# E-M17-053 details 2

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
cargo fmt --all
```

**Notes**

Repeated pinned formatting while adding the regression, pure observations and HTTP captures.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_management coverage::harness_coverage_is_bounded_cached_and_contains_no_roots --locked -- --exact > /tmp/jj-store-agent-capture-m172-coverage-before.log 2>&1
```

**Notes**

Genuine before regression: two prepared profiles have no runtime harness groups. Compiles; fails missing per-harness coverage.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_management coverage::harness_coverage_is_bounded_cached_and_contains_no_roots --locked -- --exact > /tmp/jj-store-agent-capture-m172-coverage-after.log 2>&1
```

**Notes**

Original exact regression passes after adding typed profile groups and cached counters. Later assertions add aggregate totals, actual HTTP captures and reversed configuration order; final capture runner reruns the same exact target.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_management coverage::harness_coverage_is_bounded_cached_and_contains_no_roots --locked -- --exact > /tmp/jj-store-agent-capture-m172-coverage-after2.log 2>&1
```

**Notes**

Explicit serial profile; original worker deadlines/observation bounds and within-scenario concurrency unchanged. test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.12s
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-coverage-library.log 2>&1
```

**Notes**

Explicit serial profile; original worker deadlines/observation bounds and within-scenario concurrency unchanged. test result: ok. 42 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 140.08s
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_runtime --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-coverage-runtime.log 2>&1
```

**Notes**

Explicit serial profile; original worker deadlines/observation bounds and within-scenario concurrency unchanged. test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 54.10s
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_management --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-coverage-management.log 2>&1
```

**Notes**

Explicit serial profile; original worker deadlines/observation bounds and within-scenario concurrency unchanged. test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.79s
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-coverage-supervisor.log 2>&1
```

**Notes**

Explicit serial profile; original worker deadlines/observation bounds and within-scenario concurrency unchanged. test result: ok. 27 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 209.76s
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-http --lib --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-coverage-http.log 2>&1
```

**Notes**

Explicit serial profile; original worker deadlines/observation bounds and within-scenario concurrency unchanged. test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.84s
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo check -p jj-store-server -p jj-store-service -p jj-store-http -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-coverage-check.log 2>&1
```

**Notes**

All direct server/service/HTTP/CLI targets compile before the notification representation repair; strict lint includes all final targets.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo clippy -p jj-store-server -p jj-store-service -p jj-store-http -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-coverage-lint.log 2>&1
```

**Notes**

One strict large_enum_variant diagnostic: the new body-free status increases the completion payload to 352 bytes. Box only that notification and remove Copy from its enum; cached Copy snapshots and scheduling/settlement remain unchanged. Direct library/runtime/management/supervisor suites rerun after this representation change.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo clippy -p jj-store-server -p jj-store-service -p jj-store-http -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-coverage-lint-after.log 2>&1
```

**Notes**

All direct targets pass strict lint after bounded completion payload indirection.
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
pnpm exec prettier --write scripts/contracts.mjs spec/src/api.md spec/src/cli.md spec/src/collaboration.md spec/src/compatibility.md spec/src/spec-governance.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-coverage-format.log 2>&1
```

**Notes**

Direct script/canonical/ledger formatting before adding the mapped mutation chapter.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-coverage-boundaries.log 2>&1
```

**Notes**

Owned architectural dependency boundaries pass.
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
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-coverage-architecture.log 2>&1
```

**Notes**

Architecture and production module budgets pass.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-coverage-validate.log 2>&1
```

**Notes**

Canonical governance validation passes.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first > /tmp/jj-store-agent-capture-m172-coverage-first.log 2>&1
```

**Notes**

One missing mapped mutation chapter for shared public service contracts. Add normative diagnostic grouping/non-authority semantics to the chapter; no production change.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-coverage-progress.log 2>&1
```

**Notes**

All 129 tasks and 308 evidence entries validate before final sealing.
<!-- /run -->
<!-- /evidence-field -->

