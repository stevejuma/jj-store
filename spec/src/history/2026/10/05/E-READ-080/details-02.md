---
schema: 1
id: E-READ-080
---

# E-READ-080 details 2

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
cargo check -p jj-store-jj -p jj-store-service --locked
```

**Notes**

.local/bounded-reads/managed/persistent-integration-check.log: duplicate prepare_inner; manager-check.log: Stats type mismatch. Both corrected in later checks.
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
cargo test -p jj-store-jj query::cache --locked -- --nocapture
```

**Notes**

.local/bounded-reads/managed/cache-final.log: 18 passed, 58 filtered.
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
cargo test -p jj-store-cli --test local_browser managed --locked -- --nocapture
```

**Notes**

.local/bounded-reads/managed/local-managed-final.log: 4 passed, 1 failed, 1 ignored; HTTP fixture discriminator corrected. Earlier local-managed-http.log preserves the private Result compile error.
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
cargo test -p jj-store-cli --test local_browser managed --locked -- --nocapture
```

**Notes**

.local/bounded-reads/managed/local-managed-corrected.log: 6 passed, 1 ignored.
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
cargo test -p jj-store-cli --test native_lifecycle content_acceptance::http_acceptance::managed_core_http --locked -- --nocapture
```

**Notes**

.local/bounded-reads/managed/native-http-final.log: missing required graph fields; earlier native-http.log preserves the principal size compile failure.
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
cargo test -p jj-store-cli --test native_lifecycle content_acceptance::http_acceptance::managed_core_http --locked -- --nocapture
```

**Notes**

.local/bounded-reads/managed/native-http-slice-final.log: 1 passed after monitor retry/coalescing correction.
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
cargo test -p jj-store-cli --test local_browser --locked
```

**Notes**

.local/bounded-reads/managed/local-consumers-final.log: 44 passed, 2 failed, 3 ignored. Legacy ready-point compatibility and observer contention.
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
target/debug/deps/local_browser-0bf2af6282e51f36 prepared_points_http --nocapture
```

**Notes**

.local/bounded-reads/managed/point-http-before.log: warm point reread has same 15 metadata nodes as cold.
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
cargo test -p jj-store-cli --test local_browser prepared_points_http --locked -- --nocapture
```

**Notes**

.local/bounded-reads/managed/point-http-corrected.log: 1 passed, original assertions unchanged.
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
cargo test -p jj-store-cli --test local_browser managed_lifecycle --locked -- --nocapture
```

**Notes**

.local/bounded-reads/managed/manager-final.log: 4 passed, including revocation/generation and teardown.
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
target/debug/deps/local_browser-0bf2af6282e51f36 filesystem_runtime_registers_and_reads_without_native_storage --exact --nocapture
```

**Notes**

.local/bounded-reads/managed/filesystem-cold-before.log: 1 passed in isolation; establishes contention instead of consistent cold-flow failure.
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
cargo test -p jj-store-cli --test local_browser --locked
```

**Notes**

.local/bounded-reads/managed/local-consumers-corrected.log: 46 passed, 1 failed, 3 ignored. Busy moved to review snapshot, confirming background pressure.
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
cargo test -p jj-store-jj query::workers::tests::background_observation_preserves_foreground_admission --locked -- --exact
```

**Notes**

.local/bounded-reads/managed/observation-admission.log: 1 passed; other test binaries filtered to zero.
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
cargo test -p jj-store-cli --test native_lifecycle --locked
```

**Notes**

.local/bounded-reads/managed/native-consumers-final.log: 237 passed, 82.22s; complete directly affected native consumer binary.
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
cargo test -p jj-store-http application_events --locked
```

**Notes**

.local/bounded-reads/managed/events-final.log: 3 passed; independent finite SSE admission, restart/forged auth, shared monitors.
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
cargo check -p jj-store-http -p jj-store-cli --locked
```

**Notes**

.local/bounded-reads/managed/consumers-check-corrected.log: affected direct Rust consumers check.
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
pnpm contracts:check
```

**Notes**

.local/bounded-reads/managed/contracts-check.log: intermediate private Error.code access compile failure during the late observation patch, corrected through is_busy().
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

.local/bounded-reads/managed/web-check-final.log: snapshot union typing; web-check.log additionally records generated capability drift.
<!-- /run -->
<!-- /evidence-field -->

