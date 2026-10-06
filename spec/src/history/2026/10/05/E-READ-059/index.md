---
schema: 1
id: E-READ-059
---

# E-READ-059

<!-- evidence-field:id markdown -->
## Id

E-READ-059
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

enclosing Jujutsu change tznyqrus
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Bounded data-only historical operation traversal checkpoints resume pages before lookahead
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1;jj-lib0.45.1;stock JJ0.45.1;macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No disposable/authoritative persisted format,DTO/schema/route/cursor/RPC change. Each admitted operation graph retains at most32 checkpoints keyed by resolved historical start and anchor. Checkpoints retain immutable frontier/pending chunk arrays,offset and visited bitmap using owned process-pool leases;no source,body,filter,credentials or request capability survives. Pinned JJ chunk traversal is polled over immediate compact data and fails closed on unexpected suspension. Admit conservative lookahead scratch before upstream entry;transfer completed arrays to resident leases while scratch still covers them;visited copy-on-write and checkpoint metadata reserve before growth. Save at final emitted row before the probe. Reuse rehydrates and validates anchor/current filter;missing,evicted or independently reclaimed checkpoints use bounded compact fallback.
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
- JST-API-002
- JST-API-004
- JST-LOCAL-004
- JST-CLI-003
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Shared native/local prepared historical operation continuations and direct cache/source-admission consumers. Broaden to complete owning JJ units,local browser,native graph/cache and native reference/admission tests because the shared traversal spans independently served native/local profiles. UI,public contract generation,aggregate and complete scale/release lanes are out of scope.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser/prepared_operation_history.rs
- crates/jj-store-cli/tests/native_lifecycle/operation_history_pages.rs
- crates/jj-store-jj/src/query/prepared/operations/history/checkpoint.rs
- crates/jj-store-jj/src/query/prepared/operations/history/tests.rs
- crates/jj-store-jj/src/query/prepared/operations/history/traversal.rs
- crates/jj-store-jj/src/query/prepared/operations/history/window.rs
- crates/jj-store-jj/src/query/prepared/operations/history.rs
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

- crates/jj-store-jj/src/query/prepared/operations/history/tests.rs#historical_operation_continuations_resume_without_replaying_prior_edges
- crates/jj-store-jj/src/query/prepared/operations/history/tests.rs#compact_operation_topology_matches_jj_for_merges_skewed_times_and_every_subhead
- crates/jj-store-jj/src/query/prepared/operations/history/tests.rs#selected_operation_continuation_seeks_under_a_fixed_work_budget
- crates/jj-store-cli/tests/native_lifecycle/operation_history_pages.rs#prepared_native_operation_history_hydrates_only_deep_anchor_and_page_candidates
- crates/jj-store-cli/tests/local_browser/prepared_operation_history.rs#prepared_operation_history_skips_prior_bodies_for_filtered_and_historical_pages
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-059 details 2](details-02.md)
- [E-READ-059 details 3](details-03.md)
