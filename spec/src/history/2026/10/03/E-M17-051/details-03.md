---
schema: 1
id: E-M17-051
---

# E-M17-051 details 3

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
pnpm exec prettier --write scripts/contracts.mjs spec/src/api.md spec/src/cli.md spec/src/collaboration.md spec/src/compatibility.md spec/src/mutations.md spec/src/spec-governance.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-management-doc-format.log 2>&1
```

**Notes**

Changed canonical documents and shared script formatted; final sealing repeats.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-management-boundaries.log 2>&1
```

**Notes**

Owning architecture boundary passes. Repeated after final result-field sealing with unchanged acceptance assertions.
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
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-management-architecture.log 2>&1
```

**Notes**

Production architecture budget and imports pass. Repeated after final result-field sealing with unchanged acceptance assertions.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-management-spec-validate.log 2>&1
```

**Notes**

Canonical requirements and ownership pass; final sealing repeats.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-management-spec-first.log 2>&1
```

**Notes**

25 protected files and seven canonical chapters pass; final sealing repeats.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-management-progress.log 2>&1
```

**Notes**

129 tasks and 306 entries pass bookkeeping, not product acceptance; prior 305 entries unchanged.
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
cargo test -p jj-store-server --lib capture_management::tests::policy_directory_rebinding_after_selection_withholds_application_until_exact_replay --locked > /tmp/jj-store-agent-capture-m172-management-rebinding-before.log 2>&1
```

**Notes**

Genuine functional failure: identical replacement private directory after durable selection still reports successful application instead of Unknown. Retain original receipt and binding requirement.
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
cargo test -p jj-store-server --lib capture_management::tests::policy_directory_rebinding_after_selection_withholds_application_until_exact_replay --locked > /tmp/jj-store-agent-capture-m172-management-rebinding-after.log 2>&1
```

**Notes**

Exact original regression passes after bounded private binding/latest-policy recheck under retained filesystem gate immediately before runtime application. Original directory restoration replays unchanged receipt bytes.
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
cargo test -p jj-store-http --lib --locked > /tmp/jj-store-agent-capture-m172-management-http-library2.log 2>&1
```

**Notes**

Complete HTTP library: 14 entries pass with strict 87-path inventory, 67 native and 20 local routes. Endpoint and security/shape assertions retained.
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
cargo check -p jj-store-server -p jj-store-service -p jj-store-http -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-management-check.log 2>&1
```

**Notes**

All direct targets compile before final handoff fix. Strict lint and final consumer rerun cover latest production.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib process::tests::capture_factory::cold_capture_factory_prepares_private_resources_and_retains_ordered_lifetime --locked > /tmp/jj-store-agent-capture-m172-management-cold-retry.log 2>&1
```

**Notes**

Original cold active factory still times out at unchanged 180-second caught-up observation while other builds run; subprocess completes in 276.07 seconds. Cold gate remains unresolved; no changed assertions or substituted passing control target.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_runtime --test capture_control_state --test capture_management --locked > /tmp/jj-store-agent-capture-m172-management-integration-consumers.log 2>&1
```

**Notes**

All nine journal and five management entries pass. Two runtime entries pass; seven fail at unchanged 30-second observations while sanitizer work stays Overdue, including a fresh-process storage helper. Serial complete rerun is required after isolating the scanner contention. No service/cursor/receipt assertion is weakened.
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
cargo clippy -p jj-store-server -p jj-store-service -p jj-store-http -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-management-clippy.log 2>&1
```

**Notes**

All direct targets compile and pass strict lint with final physical handoff fix.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-management-rustfmt.log 2>&1
```

**Notes**

Pinned Rust and embedded-runtime formatting pass.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_runtime owned_runtime_drives_discovery_active_capture_pause_and_terminal_shutdown --locked -- --exact > /tmp/jj-store-agent-capture-m172-management-runtime-retry.log 2>&1
```

**Notes**

Original active runtime scenario passes alone in 17.32 seconds with unchanged 30-second observation bounds, worker deadline and scanner. Complete serial suite remains required; no code/assertions changed for timing failures.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_runtime --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-management-runtime-serial.log 2>&1
```

**Notes**

All nine direct runtime entries pass serially in 123.74 seconds, including fresh-process storage/drain/drop/cancellation. Original seven timeout failures to zero with unchanged assertions, worker deadlines and observation bounds. Within-scenario concurrency unchanged; default-concurrency timing failures remain recorded.
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
pnpm contracts:check > /tmp/jj-store-agent-capture-m172-management-contracts.log 2>&1
```

**Notes**

All 147 actual registered HTTP/SSE responses conform, including eight new management samples and all prior captures. Seven shared contract-tool tests and TypeScript contract fixture compile pass. Final manager binary used by capture runs includes physical handoff fix.
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
pnpm exec prettier --write scripts/contracts.mjs spec/src/api.md spec/src/cli.md spec/src/collaboration.md spec/src/compatibility.md spec/src/mutations.md spec/src/spec-governance.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-management-doc-format2.log 2>&1
```

**Notes**

Direct documents/shared script formatted after physical handoff and timing evidence; final sealing repeats.
<!-- /run -->
<!-- /evidence-field -->

