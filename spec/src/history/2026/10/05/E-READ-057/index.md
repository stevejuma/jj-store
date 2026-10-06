---
schema: 1
id: E-READ-057
---

# E-READ-057

<!-- evidence-field:id markdown -->
## Id

E-READ-057
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

enclosing Jujutsu change ryuwywxu
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Prepared native/local operation history selects compact exact JJ topology before hydrating deep-page metadata
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; jj-lib0.45.1; stock JJ0.45.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Disposable JSTOPG1 operation-history artifact:8-byte magic,64-byte selected operation,u64 count; sorted records retain full64-byte operation ID,i64 end millis,i32 timezone offset,u32 ordered parent count and full64-byte parents; trailing SHA256. Canonical JSTLOP2 fingerprints,JSTLCH1 children,native format6 index,public DTO/schema/signed cursor/RPC and authoritative persistence remain unchanged. Existing generations without topology retain their bounded reader until explicit upgrade. Readers validate framed bytes,selection,count,sorted lookup membership,parent caps,checksum and acyclic edges before use; reserve resident and pinned upstream frontier bytes before allocation. Hydrated metadata retains fresh source authority/fingerprint and exact topology checks.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Shared native/local operation graph candidate selection,explicit generation upgrades,retained graph admission and preparation fixtures. Broaden verification because independent native/local profiles share topology and cache construction/admission; complete owning JJ units,local browser and native graph/cache plus reference/admission consumers run. No UI or wire changes.
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
- JST-API-003
- JST-CLI-003
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser/preparation.rs
- crates/jj-store-cli/tests/local_browser/prepared_operation_history.rs
- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs
- crates/jj-store-cli/tests/native_lifecycle/operation_history_pages.rs
- crates/jj-store-jj/src/local_browser/graphs.rs
- crates/jj-store-jj/src/local_browser/preparation/graph.rs
- crates/jj-store-jj/src/query/cache/constructed.rs
- crates/jj-store-jj/src/query/cache.rs
- crates/jj-store-jj/src/query/graphs.rs
- crates/jj-store-jj/src/query/index.rs
- crates/jj-store-jj/src/query/language/operations.rs
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-jj/src/query/prepared/mod.rs
- crates/jj-store-jj/src/query/prepared/operations/history/tests.rs
- crates/jj-store-jj/src/query/prepared/operations/history/window.rs
- crates/jj-store-jj/src/query/prepared/operations/history.rs
- crates/jj-store-jj/src/query/prepared/operations.rs
- docs/implementation-plan.md
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser/prepared_operation_history.rs#prepared_operation_history_skips_prior_bodies_for_filtered_and_historical_pages
- crates/jj-store-cli/tests/native_lifecycle/operation_history_pages.rs#prepared_native_operation_history_hydrates_only_deep_anchor_and_page_candidates
- crates/jj-store-jj/src/query/prepared/operations/history/tests.rs#compact_operation_topology_matches_jj_for_merges_skewed_times_and_every_subhead
- crates/jj-store-jj/src/query/prepared/operations/history/tests.rs#compact_operation_topology_rejects_selection_checksum_membership_cycles_and_byte_limits
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-057 details 2](details-02.md)
- [E-READ-057 details 3](details-03.md)
