---
schema: 1
id: E-READ-045
---

# E-READ-045

<!-- evidence-field:id markdown -->
## Id

E-READ-045
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change pwnussrp
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Prepared local Unicode reference filtering through bounded disk substring postings and selected-only hydration
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1 (48a229cea 2026-09-01); jj-lib/jj-cli pinned0.45.1; redb4.3.0; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

New disposable local-reference-filter redb file and SHA-256 sidecar; JSTLRF1 metadata binds selected64-byte operation and lexical row count. Whole-string lowercase text, per-commit text lengths, lexical reference ranks and one-/two-/three-byte gram postings use sparse16-bit offsets or4096-bit masks. No native/companion/WAL/receipt/index6, public DTO/schema, cursor or CLI format change. Native writer default does not produce this local artifact. Construction admits the8MiB database cache behind16MiB retention and a64MiB bounded posting-batch lease before allocation; max65536pending grams per4096-row block, with bounded per-text normalization/dedup leases. Disk growth, scratch, final files and sidecar share pre-growth quota. Reader retains bounded cache/data only; complete-file checksum/operation/row-count admission occurs during preparation. Requests charge candidate lookup/text work and bytes before exact text inspection, retain ordinary deadlines/cancellation/output/group limits and hydrate selected source targets through fresh scoped admission.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/query/prepared/mod.rs
- crates/jj-store-jj/src/query/prepared/disk.rs
- crates/jj-store-jj/src/query/prepared/filters.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/preparation.rs
- crates/jj-store-jj/src/local_browser/preparation/graph.rs
- crates/jj-store-jj/src/local_browser/references.rs
- crates/jj-store-jj/src/local_browser/references/prepared.rs
- crates/jj-store-cli/tests/local_browser/prepared_references.rs
- crates/jj-store-cli/tests/local_browser/preparation.rs
- crates/jj-store-cli/tests/local_browser/prepared_points_http.rs
- spec/src/local-jj.md
- spec/src/jj-integration.md
- spec/src/api.md
- spec/src/architecture.md
- spec/src/compatibility.md
- spec/src/cli.md
- spec/src/milestones.md
- docs/implementation-plan.md
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue native indexed filtering and remaining operation/deep/difference/search/RPC performance delivery in this retained workspace; qualify the complete implementation separately.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Local prepared reference filters, construction/allocation and existing cold recovery. Focused prepared point/log/prefix/comparison and HTTP consumers run because their generation construction is shared; local task cache/cancellation/quota and upstream index byte-equivalence check that added derived files preserve existing ownership. Native writer checks cover the unchanged optional-projection default. No full workspace/UI/release or combined qualification.
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
- JST-LOCAL-004
- JST-API-002
- JST-API-004
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser/prepared_references.rs#prepared_filter_windows_preserve_unicode_substrings_and_hydrate_only_selected_targets
- crates/jj-store-cli/tests/local_browser/prepared_points_http.rs#local_point_http_uses_ready_membership_and_only_falls_back_when_cold
- crates/jj-store-cli/tests/local_browser/preparation.rs#local_preparation_matches_upstream_without_trees_or_source_index_writes
- crates/jj-store-jj/src/query/prepared/filters.rs#filter_seeks_compressed_windows_and_validates_selection_checksum_and_budgets
- crates/jj-store-jj/src/query/prepared/filters.rs#filter_database_refuses_unreserved_growth
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs#disk_preparation_matches_pinned_upstream_index_bytes
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-045 details 2](details-02.md)
- [E-READ-045 details 3](details-03.md)
