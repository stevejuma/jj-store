---
schema: 1
id: E-M10-007
---

# E-M10-007

<!-- evidence-field:id markdown -->
## Id

E-M10-007
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M10
- M10.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-30
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

explicit-native-reconciliation-rpc-and-cli
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/src/commands.rs
- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/src/sync_commands.rs
- crates/jj-store-rpc/proto/native.proto
- crates/jj-store-rpc/src/application.rs
- crates/jj-store-rpc/src/client.rs
- crates/jj-store-rpc/src/lib.rs
- crates/jj-store-rpc/src/server.rs
- crates/jj-store-rpc/tests/application_mutations.rs
- crates/jj-store-service/src/mutations.rs
- docs/cli/commands.md
- docs/cli/sync.md
- spec/src/milestones.md
- spec/src/replication.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-rpc/tests/application_mutations.rs#authenticated_rpc_reconciles_only_through_explicit_receipt_backed_mutation
- crates/jj-store-rpc/tests/application_mutations.rs#local_trusted_rpc_uses_the_same_explicit_reconciliation_contract
- crates/jj-store-cli/src/sync_commands.rs#reconcile_identity_is_stable_and_snapshot_bound
<!-- /evidence-field -->

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
cargo check -p jj-store-rpc --all-targets --locked
```

**Notes**

The first compile found a protobuf RPC/message name collision, response DTOs without Deserialize and one RPC-only dependency boundary leak. The RPC was renamed CurrentSnapshot, service-owned response DTOs gained symmetric decoding and the boundary leak was removed.
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
cargo test -p jj-store-rpc --test application_mutations --locked
```

**Notes**

The first acceptance fixture tried to create previously unknown workspace names through a command that deliberately requires an existing mapping. The RPC fixture now creates two sibling operations through the owned executor without workspace mutation; existing native lifecycle reconciliation acceptance separately covers preservation and rebasing of workspace mappings.
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
cargo test -p jj-store-rpc --test application_mutations --locked
```

**Notes**

Authenticated TCP and LocalTrusted Unix RPC both preserve two explicit sibling heads across reads, publish one receipt-backed native merge only on request and return the same receipt on retry/status. Existing native lifecycle reconciliation acceptance separately proves workspace mapping preservation.
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
cargo test -p jj-store-rpc -p jj-store-service --locked
```

**Notes**

All RPC cases and initial service cases passed; the service agent-checkpoint fixture then failed to create a temporary file because the host filesystem had 153 MiB free. The workspace's ignored Cargo outputs were cleaned, freeing 25.5 GiB; no source assertion failed.
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
cargo fmt --all && pnpm exec prettier --write spec/src/milestones.md spec/src/replication.md docs/cli/commands.md docs/cli/sync.md && cargo test -p jj-store-rpc --locked && cargo clippy -p jj-store-service -p jj-store-rpc --all-targets --locked -- -D warnings && pnpm format:check && pnpm spec:validate && pnpm check:progress && pnpm check:boundaries
```

**Notes**

All nine RPC cases, service and RPC warnings-as-errors targets, formatting, 18 canonical chapters with 102 requirements, the 69-task progress ledger and 14 package boundaries pass after the final evidence correction.
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
pnpm spec:check
```

**Notes**

Specification validation and the cold catalog browser lane pass; the browser-interaction lane then stops when its native lifecycle contract fixture builds Kingfisher Vectorscan and cannot execute the unavailable cmake program.
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
pnpm check
```

**Notes**

Specification validation, package boundaries, web architecture, progress, formatting and the cold embedded RustFS clippy build pass; contracts:check then stops at the same Kingfisher Vectorscan cmake prerequisite.
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
pnpm test
```

**Notes**

Governance, storage harness, cold embedded RustFS restart and managed local storage acceptance pass; the LocalTrusted executable test then stops while building Kingfisher Vectorscan because cmake is unavailable.
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
pnpm build
```

**Notes**

Storybook, web and embedded RustFS builds pass; the final Cargo workspace build stops at the same unavailable cmake prerequisite for Kingfisher Vectorscan.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- The custom jj CLI compile and executable acceptance remain blocked in the
  existing Kingfisher Vectorscan build because cmake is unavailable on this
  host.
- M10.5 still requires every mutable AgentStore delivery permutation plus
  ReviewStore tombstone/edit and concurrent reply acceptance.
- M10.6 bootstrap/migration and M10.7 production composition, outage,
  status/search/SSE, disk-pressure and cold real RustFS qualification remain
  outstanding.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; jj 0.45.1; tonic/prost 0.14.6; application mutation schema v1; manifest format 2.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Adds CurrentSnapshot, ApplyMutation and MutationStatus to native RPC using bounded service-owned JSON mutation DTOs. The CLI derives a stable reconciliation request ID from the exact snapshot. Native object, operation, manifest, WAL, receipt, origin and replication formats are unchanged.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


