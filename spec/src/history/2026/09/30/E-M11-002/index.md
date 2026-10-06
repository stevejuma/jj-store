---
schema: 1
id: E-M11-002
---

# E-M11-002

<!-- evidence-field:id markdown -->
## Id

E-M11-002
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M11
- M11.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-30
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

workstore-transports-cli
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; jj-cli/jj-lib 0.45.1; pnpm 10.32.1
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Work API/RPC V1; opaque random process-retained query/repository/epoch-bound five-minute page cursors; format-3 local WAL publications preserve and freeze M10 origin ledger. CLI remains native local-service backed, with no upstream required. No Git refs or filesystem mirror.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/src/commands.rs
- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/src/work_commands/output.rs
- crates/jj-store-cli/src/work_commands/task.rs
- crates/jj-store-cli/src/work_commands.rs
- crates/jj-store-cli/tests/executable_native.rs
- crates/jj-store-cli/tests/native_lifecycle/http_work.rs
- crates/jj-store-cli/tests/native_lifecycle.rs
- crates/jj-store-http/src/contract.rs
- crates/jj-store-http/src/error.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-http/src/work.rs
- crates/jj-store-rpc/proto/native.proto
- crates/jj-store-rpc/src/client.rs
- crates/jj-store-rpc/src/lib.rs
- crates/jj-store-rpc/src/server.rs
- crates/jj-store-rpc/src/work.rs
- crates/jj-store-rpc/tests/work.rs
- crates/jj-store-server/src/replication.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/native.rs
- crates/jj-store-service/src/transport.rs
- crates/jj-store-service/src/work/commands.rs
- crates/jj-store-service/src/work/cursors.rs
- crates/jj-store-service/src/work/models.rs
- crates/jj-store-service/src/work/reads.rs
- crates/jj-store-service/src/work.rs
- crates/jj-store-service/tests/work_store.rs
- crates/jj-store-wal/src/lib.rs
- crates/jj-store-wal/src/publication/domains.rs
- crates/jj-store-wal/src/publication/replication.rs
- crates/jj-store-wal/src/publication/replication_import.rs
- crates/jj-store-wal/src/publication/replication_seed.rs
- crates/jj-store-wal/src/publication/replication_transfer.rs
- crates/jj-store-wal/src/publication/work_commands.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/tests/work_store.rs
- docs/cli/work.md
- schemas/openapi.json
- scripts/contracts.mjs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/mutations.md
- spec/src/projects.md
- spec/src/publication.md
- spec/src/replication.md
- spec/src/security.md
- spec/src/spec-governance.md
- web/src/lib/api/generated.ts
- spec/src/compatibility.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/work_store.rs#linked_repository_metadata_does_not_grant_code_access_and_cursors_expire
- crates/jj-store-rpc/tests/work.rs#current
- crates/jj-store-cli/tests/executable_native.rs#executable_workstore_creates_tasks_comments_and_recovers_without_an_upstream
- crates/jj-store-cli/tests/native_lifecycle/http_work.rs#http_workstore_pages_commands_permissions_and_contract_capture
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M11-002 details 2](details-02.md)
- [E-M11-002 details 3](details-03.md)
