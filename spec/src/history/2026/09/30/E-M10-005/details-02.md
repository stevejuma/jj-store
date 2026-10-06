---
schema: 1
id: E-M10-005
---

# E-M10-005 details 2

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
cargo check -p jj-store-wal -p jj-store-replication -p jj-store-service -p jj-store-rpc --all-targets --locked
```

**Notes**

The first compile stopped before compilation because the new direct package dependencies required a Cargo.lock refresh. No source test failed.
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
cargo check -p jj-store-wal -p jj-store-replication -p jj-store-service -p jj-store-rpc --all-targets --offline
```

**Notes**

The lockfile was refreshed without network access and all affected transport targets compiled.
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
cargo test -p jj-store-replication --test checkpoint --offline
```

**Notes**

The issuer-bound checkpoint rejects backwards movement and recovers an exact CAS update after a lost object-store acknowledgement.
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
cargo test -p jj-store-rpc --test replication --offline
```

**Notes**

Two authenticated writable replicas negotiate a fixed feed, transfer digest-verified native/control closure data, import one AgentStore origin, checkpoint after the durable receipt, restart and replay without another semantic effect.
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
cargo test -p jj-store-replication -p jj-store-rpc --offline
```

**Notes**

All ten replication integration cases and all six RPC unit/integration cases pass, including existing agent, review, LocalTrusted and administration regressions.
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
cargo clippy -p jj-store-wal -p jj-store-replication -p jj-store-service -p jj-store-rpc --all-targets --offline -- -D warnings
```

**Notes**

All affected WAL, replication, service and RPC targets pass warnings-as-errors.
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
cargo check -p jj-store-wal -p jj-store-replication -p jj-store-service -p jj-store-rpc --all-targets --locked
```

**Notes**

The refreshed locked dependency graph compiles for every affected target.
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
cargo test -p jj-store-wal -p jj-store-service --locked
```

**Notes**

All 52 WAL integration cases and all 28 service unit/integration cases pass, including authorization, recovery and collaboration reads.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:boundaries
```

**Notes**

The first boundary run rejected production dependencies from RPC to replication/WAL and service to replication. DTO ownership moved to the WAL publication boundary and RPC now consumes only the service transport facade; no boundary rule was changed.
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
pnpm check:boundaries && cargo check -p jj-store-wal -p jj-store-replication -p jj-store-service -p jj-store-rpc --all-targets --offline
```

**Notes**

All 14 package boundaries pass after the ownership correction and every affected target compiles.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:check
```

**Notes**

The first wider gate reached catalog:check and exposed missing M10.5 conflict and supersession fields in two existing catalog collaboration fixtures.
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
pnpm catalog:check
```

**Notes**

The repaired catalog fixtures compile with zero Svelte errors or warnings and all 64 catalog module boundaries pass.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:check
```

**Notes**

Specification validation, cold catalog build/browser acceptance and the repaired catalog type lane passed. The later browser fixture capture stopped in Kingfisher Vectorscan because cmake is unavailable.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check
```

**Notes**

Specification, boundary, architecture, progress, Rust formatting and embedded RustFS clippy lanes passed. contracts:check stopped in Kingfisher Vectorscan because cmake is unavailable.
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
pnpm test
```

**Notes**

All 29 governance tests, the storage harness self-tests and both embedded/managed RustFS restart cases passed. The LocalTrusted CLI test then stopped in Kingfisher Vectorscan because cmake is unavailable.
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
pnpm build
```

**Notes**

Storybook, web and embedded RustFS builds passed. The workspace Rust build then stopped in Kingfisher Vectorscan because cmake is unavailable.
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
cargo fmt --all && pnpm exec prettier --write catalog/controller/in-memory-store-collaboration.svelte.ts catalog/fixtures/collaboration-review.ts spec/src/milestones.md spec/src/replication.md && cargo test -p jj-store-replication -p jj-store-rpc --locked && cargo clippy -p jj-store-wal -p jj-store-replication -p jj-store-service -p jj-store-rpc --all-targets --locked -- -D warnings && pnpm format:check && pnpm spec:validate && pnpm check:progress && pnpm check:boundaries
```

**Notes**

After correcting type ownership and catalog fixtures, all ten replication cases, six RPC cases, affected warnings-as-errors targets, formatting, specification validation, progress validation and 14 package boundaries pass together.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- M10.3 still requires the bounded bidirectional worker and durable push/pull
  scheduling over this RPC surface.
- M10.4 requires distinct hydrated workspace acceptance and an explicit native
  reconciliation command.
- M10.5 still requires broader delivery permutations.
- M10.6 bootstrap/migration and M10.7 status, outage and real RustFS
  qualification remain outstanding.
- The repository-wide contracts, check, test and build lanes that compile
  Kingfisher Vectorscan still require cmake on this host.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; jj 0.45.1; tonic/prost 0.14.6; manifest format 2; replication protocol, peer checkpoint, origin and apply-origins schema v1.
<!-- /evidence-field -->

