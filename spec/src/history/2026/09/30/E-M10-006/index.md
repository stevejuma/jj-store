---
schema: 1
id: E-M10-006
---

# E-M10-006

<!-- evidence-field:id markdown -->
## Id

E-M10-006
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M10
- M10.3
- M10.7
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-30
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

bounded-bidirectional-replication-worker
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-replication/Cargo.toml
- crates/jj-store-replication/src/lib.rs
- crates/jj-store-replication/src/worker.rs
- crates/jj-store-rpc/tests/replication.rs
- spec/src/milestones.md
- spec/src/replication.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-rpc/tests/replication.rs#bounded_worker_retries_with_stable_identity_and_converges_both_directions
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 12
hasNote: true
noteEncoding: markdown

```

```shell
cargo check -p jj-store-replication --all-targets --locked
```

**Notes**

The first compile found the new endpoint failure lacked the intended Error conversion and one unsupported thiserror field expression. The error boundary was corrected without weakening endpoint classification.
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
cargo check -p jj-store-replication --all-targets --locked
```

**Notes**

The bounded worker, independent direction circuits, durable barrier status and endpoint interface compile for every replication target.
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
cargo test -p jj-store-replication -p jj-store-rpc --test replication --locked
```

**Notes**

The first worker acceptance rejected a correctly forwarded origin because it incorrectly equated original author identity with the feed delivery replica. Validation now binds the cursor to the delivery issuer while preserving the original origin key.
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
cargo test -p jj-store-replication -p jj-store-rpc --test replication --locked
```

**Notes**

Both authenticated RPC cases pass, including two independent writers, bounded alternating push and pull, forwarded origins, a failed pre-send apply, stable retry identities, durable cursors and a captured finite barrier.
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
cargo test -p jj-store-replication -p jj-store-rpc --locked
```

**Notes**

All eleven replication integration cases and all seven RPC unit/integration cases pass together.
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
cargo clippy -p jj-store-replication -p jj-store-rpc --all-targets --locked -- -D warnings
```

**Notes**

Warnings-as-errors requested a derived Default implementation for the private circuit state; the mechanical correction was applied.
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
cargo clippy -p jj-store-replication -p jj-store-rpc --all-targets --locked -- -D warnings && pnpm check:boundaries
```

**Notes**

The worker and RPC targets pass warnings-as-errors and all 14 package boundaries remain valid.
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
pnpm spec:check
```

**Notes**

Specification validation and cold catalog browser acceptance passed; the later browser fixture capture stopped in the existing Kingfisher Vectorscan build because cmake is unavailable.
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
pnpm check
```

**Notes**

Specification, boundary, architecture, progress, Rust formatting and embedded RustFS clippy lanes passed; contracts:check stopped in Kingfisher Vectorscan because cmake is unavailable.
<!-- /run -->

<!-- run -->
### Run 10

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

All 29 governance tests, storage harness self-tests and embedded/managed RustFS restart cases passed; LocalTrusted compilation stopped in Kingfisher Vectorscan because cmake is unavailable.
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
pnpm build
```

**Notes**

Storybook, web and embedded RustFS builds passed; the workspace Rust build stopped in Kingfisher Vectorscan because cmake is unavailable.
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
cargo fmt --all && pnpm exec prettier --write spec/src/milestones.md spec/src/replication.md && cargo test -p jj-store-replication -p jj-store-rpc --locked && cargo clippy -p jj-store-replication -p jj-store-rpc --all-targets --locked -- -D warnings && pnpm format:check && pnpm spec:validate && pnpm check:progress && pnpm check:boundaries
```

**Notes**

All eleven replication integration cases, seven RPC cases, warnings-as-errors targets, formatting, specification validation, progress validation and 14 package boundaries pass together after the worker and ledger updates.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- M10.4 distinct hydrated workspace and explicit native reconciliation
  acceptance remain outstanding.
- M10.5 still requires broader AgentStore and ReviewStore delivery permutations.
- M10.6 seed, snapshot-plus-tail bootstrap and non-destructive migration remain
  outstanding.
- M10.7 still requires production lifecycle composition, foreground outage
  proof, SSE/search status integration, disk-pressure qualification and cold
  real RustFS evidence.
- The repository-wide contracts, check, test and build lanes that compile
  Kingfisher Vectorscan require cmake on this host.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; jj 0.45.1; tonic/prost 0.14.6; manifest format 2; replication protocol, peer checkpoint, origin and apply-origins schema v1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Adds a transport-neutral bidirectional worker with fixed-bound pages, stable derived apply identities, bounded disk-spooled native transfer, exact control transfer, independent push/pull retry circuits, deterministic jitter, pause/wake status and finite-frontier barriers.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


