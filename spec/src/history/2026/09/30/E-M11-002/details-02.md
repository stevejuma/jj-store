---
schema: 1
id: E-M11-002
---

# E-M11-002 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
cargo check -p jj-store-service --locked
```

**Notes**

Incomplete scaffolding: missing Work modules; later compiled.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-service --test work_store --locked
```

**Notes**

Native creation already owns an origin ledger, invalidating the initial rejection.
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
cargo test -p jj-store-wal --test work_store --locked
```

**Notes**

Six Work tests pass after preserving and freezing the origin ledger.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 15
hasNote: true
noteEncoding: markdown

```

```shell
cargo check -p jj-store-cli --locked
```

**Notes**

Glob-imported service Result conflicted with CLI result; replaced with explicit imports.
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
cargo check -p jj-store-cli --locked
```

**Notes**

Server replication match needed permanent Unsupported handling.
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
cargo check -p jj-store-service --locked
```

**Notes**

Owned service compiles.
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
cargo check -p jj-store-rpc -p jj-store-http --locked
```

**Notes**

Affected transport packages compile.
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
cargo test -p jj-store-rpc --test work --locked
```

**Notes**

One real isolated local RPC transport acceptance passes; storage is fake.
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
cargo test -p jj-store-cli --test executable_native --locked executable_workstore_creates_tasks_comments_and_recovers_without_an_upstream -- --exact
```

**Notes**

Compiled CLI creates project/task/comment, retries, rejects reader writes, restarts and recovers receipts with no upstream; one pass.
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
cargo test -p jj-store-cli --test native_lifecycle --locked http_work_acceptance::http_workstore_pages_commands_permissions_and_contract_capture -- --exact
```

**Notes**

One executed HTTP paging/commands/permissions/status acceptance passes.
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
cargo test -p jj-store-wal --locked
```

**Notes**

New copy test incorrectly named a service-only descriptor method; removed the invalid call.
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
cargo test -p jj-store-wal --locked
```

**Notes**

58 authority tests pass. Escalation: format-3 origin freeze affects shared code, AgentStore, ReviewStore publications and native copy. Nonempty historical origin ledger survives Work upgrade, checkpoint, copy and cold recovery.
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
cargo test -p jj-store-service --test work_store --locked
```

**Notes**

Three acceptances pass: pinned paging, no GET publication, durable actor/receipt/event semantics, separate target-code authorization, cursor expiry and token expiry.
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
cargo test -p jj-store-replication --locked
```

**Notes**

16 direct consumer tests pass across origin transfer/bootstrap/checkpoint and agent/review convergence.
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
cargo clippy -p jj-store-service -p jj-store-http -p jj-store-rpc -p jj-store-cli -p jj-store-server --all-targets --locked -- -D warnings
```

**Notes**

Affected packages and direct consumers pass with warnings denied.
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
pnpm contracts:generate
```

**Notes**

OpenAPI and exact-integer browser contract generated.
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
pnpm contracts:check
```

**Notes**

78 actual HTTP/SSE responses conform; six contract unit tests and contract TypeScript pass. Escalation: shared route inventory and public generated API affect all captured direct consumers.
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
cargo fmt -p jj-store-service -p jj-store-wal -p jj-store-http -p jj-store-rpc -p jj-store-cli -p jj-store-server
```

**Notes**

Affected Rust packages formatted.
<!-- /run -->
<!-- /evidence-field -->

