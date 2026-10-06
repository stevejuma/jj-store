---
schema: 1
id: E-READ-047
---

# E-READ-047

<!-- evidence-field:id markdown -->
## Id

E-READ-047
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change uvwwwsmn
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Native prepared substring windows with quota-owned immutable filter readers
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; jj-lib/jj-cli source pinned0.45.1; redb4.3.0; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native/index6/receipt/companion/WAL, public DTO/schema, RPC or signed cursor format changes. Eligible explicit native preparation shares the existing disposable JSTLRF1 description/reference projection, complete SHA256 sidecar and bounded redb reader lease. Pin bookkeeping is an empty root reader-pin marker excluded from immutable receipts; it owns no source authority. Sorted postings only nominate candidates; exact selected visibility, Unicode-lowercased text and bookmark added/removed terms decide rows before selected hydration. Idle-only reclamation runs outside namespace/cache locks; active readers retain their files, full slot quota and allocation leases. Cold pinned recovery stays in place under shared ownership; unpinned recovery remains privately reserved. Index-only generations upgrade only through explicit preparation. Small-cache memory-only policies preserve ordinary bounded fallback. Resident retention uses framed JJ allocation bytes and bounded filter-cache memory; physical database bytes stay in disk quota. Prior index-allocation charges remain attached when an existing resident is wrapped in fresh request ownership.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/browsing.rs
- crates/jj-store-cli/tests/native_lifecycle/filter_cache.rs
- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs
- crates/jj-store-cli/tests/native_lifecycle/reference_filters.rs
- crates/jj-store-jj/src/query/access.rs
- crates/jj-store-jj/src/query/cache/constructed.rs
- crates/jj-store-jj/src/query/cache/generation.rs
- crates/jj-store-jj/src/query/cache/recovery.rs
- crates/jj-store-jj/src/query/cache/resident/additional.rs
- crates/jj-store-jj/src/query/cache/resident.rs
- crates/jj-store-jj/src/query/cache.rs
- crates/jj-store-jj/src/query/content/references/filter.rs
- crates/jj-store-jj/src/query/content/references/prepared.rs
- crates/jj-store-jj/src/query/content/references.rs
- crates/jj-store-jj/src/query/content.rs
- crates/jj-store-jj/src/query/index/secondary.rs
- crates/jj-store-jj/src/query/index.rs
- crates/jj-store-jj/src/query/memory/retained.rs
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-jj/src/query/prepared/filters.rs
- crates/jj-store-jj/src/query/prepared/mod.rs
- crates/jj-store-jj/src/query/projection/decorations.rs
- crates/jj-store-jj/src/query/projection/filter.rs
- crates/jj-store-jj/src/query/projection.rs
- docs/implementation-plan.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Move cold full receipt hashing outside namespace ownership, then different embedded operation references and indexed deep/comparison/search/RPC delivery. Keep complete qualification separate.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Private native filtering, shared resident/cache ownership and direct graph/content/HTTP consumers. Exact regressions and focused package checks only; no combined integration, full contracts, UI, large fixtures or release qualification. Final admission adjustment reruns its exact arithmetic regression, affected cache units, native filtering/cache/graph/preparation/HTTP consumers and direct package checks.
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/reference_filters.rs#native_filtered_references_seek_pinned_text_before_selected_target_reads
- crates/jj-store-cli/tests/native_lifecycle/filter_cache.rs#native_filter_upgrade_and_pinned_cold_recovery_preserve_exact_generation
- crates/jj-store-cli/tests/native_lifecycle/filter_cache.rs#native_filter_quota_reclaims_idle_residents_and_preserves_live_reader_files
- crates/jj-store-jj/src/query/cache/generation.rs#readonly_generation_pins_preserve_files_and_quota_until_last_reader_exits
- crates/jj-store-jj/src/query/cache/generation.rs#pinned_slots_stay_charged_and_eviction_skips_to_a_removable_generation
- crates/jj-store-jj/src/query/memory/retained.rs#selective_idle_reclamation_preserves_active_and_unmatched_values
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs#disk_preparation_matches_pinned_upstream_index_bytes
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs#http_reads_index_task_and_signed_cursor_preserve_snapshot_across_publication
- crates/jj-store-jj/src/query/cache/resident.rs#native_resident_admission_counts_index_allocations_and_bounded_filter_cache
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-047 details 2](details-02.md)
- [E-READ-047 details 3](details-03.md)
- [E-READ-047 details 4](details-04.md)
