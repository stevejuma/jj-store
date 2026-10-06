---
schema: 1
id: E-M10-004
---

# E-M10-004 details 2

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
cargo test -p jj-store-replication --test collaboration --locked
```

**Notes**

All three collaboration import cases pass. AgentStore and ReviewStore facts share one atomic receiver publication, disjoint facts merge, alias collisions remain explicit, facts must match their exact authored-root selections, and the import-only guard proves the receiver never enters authored domain, collaboration-command or native-transition authorization.
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
cargo test -p jj-store-replication --test review_convergence --locked
```

**Notes**

All three review convergence cases pass under reversed and duplicate delivery. Concurrent revisions remain explicit, workflow races reduce conservatively, exact conflict resolution names every superseded revision, stale coverage fails and the resolution converges after replication. Cold native copy and recovery retain the merged thread and conflict closure.
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
cargo test -p jj-store-replication --test agent_convergence --locked
```

**Notes**

Disjoint session and tool facts union deterministically, incompatible terminal outcomes remain an explicit conflict and cold native copy plus recovery preserves that conflict and its alternatives.
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
cargo test -p jj-store-replication --test one_way --locked
```

**Notes**

Both established one-way receiver cases continue to pass with typed collaboration facts enabled.
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
cargo test -p jj-store-wal --locked
```

**Notes**

All 52 WAL integration cases plus unit and documentation targets pass, including publication, native copy, AgentStore and ReviewStore regression lanes.
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
cargo test -p jj-agent --locked
```

**Notes**

All 20 AgentStore model, normalization, catalog, provenance and codec cases pass with explicit catalog-conflict objects.
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
cargo test -p jj-review --locked
```

**Notes**

All 15 ReviewStore anchor, object, delta and resolver cases pass; existing canonical comment bytes remain stable because empty supersession sets are omitted.
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
cargo check -p jj-store-wal -p jj-store-replication -p jj-store-service --all-targets --locked
```

**Notes**

The accepted-fact reducers, command continuation over merged threads and service presentation compile across every affected target.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo check -p jj-review -p jj-store-wal -p jj-store-service -p jj-store-http -p jj-store-rpc -p jj-store-cli --all-targets --locked
```

**Notes**

Compilation reached the existing Kingfisher Vectorscan native build and stopped because cmake is not installed on this host. The collaboration-only compile lane passes and no gate was weakened.
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
pnpm contracts:generate
```

**Notes**

The HTTP OpenAPI schema and generated TypeScript contract include merged review read models and exact comment-conflict resolution commands.
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
pnpm format
```

**Notes**

Rust and tracked text artifacts were formatted after implementation and evidence updates.
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
pnpm format:check
```

**Notes**

Workspace Rust, embedded RustFS and Prettier formatting checks pass.
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
pnpm spec:validate
```

**Notes**

All four validators accept 18 canonical chapters and 102 requirements.
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
pnpm check:boundaries
```

**Notes**

All 14 package boundaries pass; the WAL replication reducer introduces no controller, hook, message or external-adapter dependency.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

The first progress check exposed a malformed literal newline at the newly appended JSON ledger closing. The ledger was repaired and formatted before rerunning the checker.
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
pnpm check:progress
```

**Notes**

The repaired 69-task milestone ledger and E-M10-004 evidence entry validate.
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
cargo clippy -p jj-agent -p jj-review -p jj-store-wal -p jj-store-replication -p jj-store-service --all-targets --locked -- -D warnings
```

**Notes**

The first focused clippy run exposed a pre-existing complex tuple signature in the M10.3 one-way test fixture. The fixture received a named type alias without changing behavior.
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
cargo fmt --all && cargo clippy -p jj-agent -p jj-review -p jj-store-wal -p jj-store-replication -p jj-store-service --all-targets --locked -- -D warnings
```

**Notes**

All affected collaboration, WAL, replication and service targets pass warnings-as-errors after the narrow fixture repair.
<!-- /run -->
<!-- /evidence-field -->

