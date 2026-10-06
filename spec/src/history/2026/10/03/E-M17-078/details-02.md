---
schema: 1
id: E-M17-078
---

# E-M17-078 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_management api_snapshot::api_snapshot_progress_is_native_receipt_backed_and_body_free --locked -- --exact
```

**Notes**

Qualified before: two E0609 missing snapshot_publication field diagnostics. Initial fixture hygiene errors were fixed before this qualified observation. /tmp/jj-store-snapshot-management-before-qualified.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_management api_snapshot::api_snapshot_progress_is_native_receipt_backed_and_body_free --locked -- --exact
```

**Notes**

Initial native HTTP after passes. Complete final management binary below includes additional stopped/disabled assertions. /tmp/jj-store-snapshot-management-after.log.
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
cargo test -p jj-store-capture --test snapshot_publication --locked
```

**Notes**

10 tests pass, including bounded batch inspection. /tmp/jj-store-snapshot-management-capture.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture -p jj-store-server --test snapshot_publication --test reader_publication --test capture_management --test opencode_supervisor --test opencode_lifecycle --test opencode_source --locked
```

**Notes**

61 selected tests pass across six binaries. /tmp/jj-store-snapshot-management-consumers.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/capture-management.test.ts web/tests/vite/production-capture-management.test.ts
```

**Notes**

11-test observation: transport assertion incorrectly expected small safe JSON numbers as bigint; production lossless integer behavior was unchanged. A later 12-test run exposed an assertion comparing ApiError code to its human-readable message; corrected to the error class. Both failures retained in the work record.
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
pnpm check:architecture
```

**Notes**

New inline snapshot validation exceeded the existing 300-line budget at 324 counted lines. Extracted shared primitives and the snapshot validator; no budget or baseline relaxation.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/capture-management.test.ts web/tests/vite/production-capture-management.test.ts
```

**Notes**

12 tests pass; malformed coverage and exact uint64 transport plus replacement-generation counts. /tmp/jj-store-snapshot-management-client-after.log.
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
pnpm web:check
```

**Notes**

Zero errors and warnings. /tmp/jj-store-snapshot-management-web-check.log.
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
pnpm check:architecture
```

**Notes**

734 modules pass. /tmp/jj-store-snapshot-management-architecture.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_management --locked -- --test-threads=1
```

**Notes**

14 tests pass including final offline/paused/stopped/disabled actual HTTP snapshot fixture. /tmp/jj-store-snapshot-management-final-native.log.
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
cargo clippy -p jj-store-capture -p jj-store-service -p jj-store-server -p jj-store-http --all-targets --locked -- -D warnings
```

**Notes**

Strict affected packages and all-target consumers pass. /tmp/jj-store-snapshot-management-clippy.log.
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
pnpm contracts:generate
```

**Notes**

Additive version-1 response record; regenerated OpenAPI and TypeScript. /tmp/jj-store-snapshot-management-generate.log.
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
cargo fmt -p jj-store-capture -p jj-store-service -p jj-store-server -p jj-store-http -- --check
```

**Notes**

Affected Rust formatting passes.
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
pnpm exec prettier --write spec/src/api.md spec/src/cli.md spec/src/collaboration.md spec/src/compatibility.md spec/src/mutations.md spec/src/spec-governance.md spec/src/verification.md web/tests/vite/capture-management.test.ts
```

**Notes**

Changed-document formatting; client validation leaf modules were formatted separately.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher pnpm contracts:check
```

**Notes**

Complete registered inventory: 199 actual HTTP/SSE responses conform; seven contract-tool tests and generated-client TypeScript pass. /tmp/jj-store-snapshot-management-contracts.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management:: --locked -- --test-threads=1
```

**Notes**

49 pass; the existing ten-second multi-step import test expires while the independent shared contract CLI rebuild is running. No assertion or deadline changes. /tmp/jj-store-snapshot-management-units.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::scheduled_import::multi_step_import_prefix_survives_worker_parking_without_starving_ingress --locked -- --exact
```

**Notes**

Unchanged exact test passes alone in 8.28s; the complete management module is being rerun with no concurrent build. This exact pass is duplicate verification, not a substitute for that module. /tmp/jj-store-snapshot-management-deadline-exact.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management:: --locked -- --test-threads=1
```

**Notes**

Complete unchanged module: 50 pass in 140.99s with no concurrent build. /tmp/jj-store-snapshot-management-units-isolated.log. Concurrent-build timing remains unqualified.
<!-- /run -->
<!-- /evidence-field -->

