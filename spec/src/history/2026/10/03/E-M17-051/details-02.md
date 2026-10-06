---
schema: 1
id: E-M17-051
---

# E-M17-051 details 2

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

Pinned Rust formatting repeated during scaffold, compiler repairs and admission coverage.
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
cargo test -p jj-store-http --lib contract::capture_tests::capture_management_contract_is_installation_local_and_actor_free --locked > /tmp/jj-store-agent-capture-m172-management-contract-before.log 2>&1
```

**Notes**

Original route contract fails because installation-local endpoint is absent.
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
cargo test -p jj-store-http --lib contract::capture_tests::capture_management_contract_is_installation_local_and_actor_free --locked > /tmp/jj-store-agent-capture-m172-management-contract-after.log 2>&1
```

**Notes**

Test incorrectly expected a named input schema; generator inlines commands. Preserve actor-free strict command assertions.
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
cargo test -p jj-store-http --lib contract::capture_tests::capture_management_contract_is_installation_local_and_actor_free --locked > /tmp/jj-store-agent-capture-m172-management-contract-after2.log 2>&1
```

**Notes**

Test incorrectly expected an input reference; inspect actual generated inline body without weakening field assertions.
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
cargo test -p jj-store-http --lib contract::capture_tests::capture_management_contract_is_installation_local_and_actor_free --locked > /tmp/jj-store-agent-capture-m172-management-contract-after3.log 2>&1
```

**Notes**

Original route/actor-free/generation/required-field contract passes.
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
cargo check -p jj-store-server --all-targets --locked > /tmp/jj-store-agent-capture-m172-management-scaffold-check.log 2>&1
```

**Notes**

Scaffold compiles before later test additions; not final acceptance.
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
pnpm contracts:generate > /tmp/jj-store-agent-capture-m172-management-generate.log 2>&1
```

**Notes**

Additive OpenAPI and direct generated TypeScript.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-server --test capture_management --locked > /tmp/jj-store-agent-capture-m172-management-integration.log 2>&1
```

**Notes**

Three scaffold E0599 errors: reqwest JSON helpers unavailable and CaptureAuthority is not Clone. Use existing byte/Serde API and public authority preparation.
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
cargo test -p jj-store-server --lib capture_management::tests --locked > /tmp/jj-store-agent-capture-m172-management-ownership.log 2>&1
```

**Notes**

Two reqwest response JSON helper errors in cold foreground test; no behavioral assertions ran.
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
cargo test -p jj-store-server --lib process::tests::capture_factory::management::cold_foreground_advertises_authenticated_disabled_capture_without_creating_policy --locked > /tmp/jj-store-agent-capture-m172-management-foreground.log 2>&1
```

**Notes**

Same two cold foreground compile errors; confirmed original running handle completed, without restarting on observation timeout.
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
cargo test -p jj-store-server --test capture_management --locked > /tmp/jj-store-agent-capture-m172-management-integration2.log 2>&1
```

**Notes**

One E0432 after authority repair used wrong module import; corrected to capture_authority.
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
cargo test -p jj-store-server --test capture_management --locked > /tmp/jj-store-agent-capture-m172-management-integration3.log 2>&1
```

**Notes**

Four integration entries pass. Unused journal fixture field warning removed before strict lint.
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
cargo test -p jj-store-server --lib capture_management::tests --locked > /tmp/jj-store-agent-capture-m172-management-ownership2.log 2>&1
```

**Notes**

Three cancellation/current-revocation/task-panic ownership entries pass.
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
cargo test -p jj-store-server --lib process::tests::capture_factory::management::cold_foreground_advertises_authenticated_disabled_capture_without_creating_policy --locked > /tmp/jj-store-agent-capture-m172-management-foreground2.log 2>&1
```

**Notes**

Fresh-process actual foreground disabled status/bootstrap/lock-release/no-policy-creation passes.
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
cargo test -p jj-store-server --test capture_management --locked > /tmp/jj-store-agent-capture-m172-management-integration4.log 2>&1
```

**Notes**

Five integration entries pass after adding Read/Write/Authenticated denials, hostile transport, 4096-byte limit, missing fields and stale generation; captures include actual runtime counters.
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
cargo test -p jj-store-http --lib --locked > /tmp/jj-store-agent-capture-m172-management-http-library.log 2>&1
```

**Notes**

13 pass, one inventory assertion fails (87 versus stale 84). Pinned parent OpenAPI has 85 paths: 67 native, 18 local; this slice adds exactly two installation-local paths. Update precise inventory to 87/67/20 and add explicit endpoint assertions.
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
pnpm web:check > /tmp/jj-store-agent-capture-m172-management-web.log 2>&1
```

**Notes**

Generated API direct consumer: zero errors and zero warnings.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --test capture_runtime --test capture_control_state --locked > /tmp/jj-store-agent-capture-m172-management-consumers.log 2>&1
```

**Notes**

35 library entries pass; original cold active capture subprocess times out at its unchanged 180-second caught-up observation while concurrent compilation runs. Direct runtime/journal binaries do not run because library failure stops this invocation. Exact cold rerun and full direct consumer rerun required after compilation settles; do not substitute successful management tests.
<!-- /run -->
<!-- /evidence-field -->

