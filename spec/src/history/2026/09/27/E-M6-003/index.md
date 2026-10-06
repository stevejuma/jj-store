---
schema: 1
id: E-M6-003
---

# E-M6-003

<!-- evidence-field:id markdown -->
## Id

E-M6-003
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M6
- M6.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native-tantivy-extraction
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-adapters/src/source.rs
- crates/jj-store-adapters/tests/delivery.rs
- crates/jj-store-cli/Cargo.toml
- crates/jj-store-cli/tests/native_lifecycle/adapter_projection.rs
- crates/jj-store-cli/tests/native_lifecycle/content.rs
- crates/jj-store-service/src/adapter.rs
- crates/jj-store-service/src/adapter/read.rs
- crates/jj-store-tantivy/Cargo.toml
- crates/jj-store-tantivy/src/disk.rs
- crates/jj-store-tantivy/src/document.rs
- crates/jj-store-tantivy/src/lib.rs
- crates/jj-store-tantivy/src/native.rs
- crates/jj-store-tantivy/src/search.rs
- crates/jj-store-tantivy/tests/support/mod.rs
- crates/jj-store-wal/src/publication/reader.rs
- docs/evidence/m6-native-extraction.md
- spec/src/adapters.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/publication.md
- spec/src/security.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-adapters/tests/delivery.rs#adapter_history_and_prior_inventory_stop_at_the_delivered_snapshot
- crates/jj-store-cli/tests/native_lifecycle/adapter_projection.rs#native_adapter_projection_reader_decodes_committed_trees_and_files
- crates/jj-store-cli/tests/native_lifecycle/adapter_projection.rs#native_tantivy_extracts_committed_metadata_and_explicit_source_scopes
- crates/jj-store-cli/tests/native_lifecycle/adapter_projection.rs#native_tantivy_diffs_replay_equivalently_and_keep_exact_scope_context
- crates/jj-store-cli/tests/native_lifecycle/adapter_projection.rs#native_tantivy_clears_ambiguous_scopes_and_respects_source_disable
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M6-003 details 2](details-02.md)
- [E-M6-003 details 3](details-03.md)
