---
schema: 1
id: E-READ-001
---

# E-READ-001

<!-- evidence-field:id markdown -->
## Id

E-READ-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change pmnkmqxz
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Manifest-root application read membership and large-repository implementation scope
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; jj-lib 0.45.1; Node 26.9.0; pnpm 10.32.1; native V1 objects and existing manifest formats.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No persisted or HTTP/RPC wire encoding changes. ReadAccess adds asynchronous point membership and explicit complete enumeration. Cursor pins retain checked manifests/root/head identities rather than full object inventories. Mutation and adapter guards remain current; source closure audits remain in publication/recovery. Repository directory expectation now includes already-shipped native_v1/ready discriminants.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle.rs
- crates/jj-store-cli/tests/native_lifecycle/content.rs
- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs
- crates/jj-store-cli/tests/native_lifecycle/query_resources.rs
- crates/jj-store-jj/examples/content_qualification.rs
- crates/jj-store-jj/examples/query_qualification.rs
- crates/jj-store-jj/src/commands/access.rs
- crates/jj-store-jj/src/commands/execute.rs
- crates/jj-store-jj/src/query/access.rs
- crates/jj-store-jj/src/query/content.rs
- crates/jj-store-jj/src/query/graphs.rs
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-jj/src/query/predicates.rs
- crates/jj-store-service/src/adapter/read.rs
- crates/jj-store-service/src/commands.rs
- crates/jj-store-service/src/pagination.rs
- crates/jj-store-service/src/pagination/cursors.rs
- crates/jj-store-service/src/reads.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/publication/reader.rs
- crates/jj-store-wal/src/publication/reader_tests.rs
- docs/implementation-plan.md
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/publication.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-API-001
- JST-API-002
- JST-JJ-003
- JST-JJ-005
- JST-GOV-002
- JST-GOV-003
- JST-GOV-005
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle.rs#read_session_point_lookup_does_not_materialize_inventory
- crates/jj-store-cli/tests/native_lifecycle.rs#read_session_enforces_inventory_and_elapsed_budgets
- crates/jj-store-wal/src/publication/reader_tests.rs#point_membership_reads_radix_paths_instead_of_the_inventory
- crates/jj-store-wal/src/publication/reader_tests.rs#point_membership_checks_traversed_dependencies_and_descriptor_keys
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-001 details 2](details-02.md)
