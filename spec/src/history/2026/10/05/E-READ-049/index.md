---
schema: 1
id: E-READ-049
---

# E-READ-049

<!-- evidence-field:id markdown -->
## Id

E-READ-049
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

enclosing Jujutsu change woktsopu
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

User-requested accumulated performance landing with current main freshness
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; jj-lib/jj-cli0.45.1; redb4.3.0; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Merge retains native/index6/receipt/companion/WAL and signed cursor formats, current event-freshness authority and session-owned query leases. Generated contracts combine both parents' additive API routes. No milestone promotion, publication, installed CLI replacement, fixture migration or development-service restart.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/fixtures/projects-hub-fixture.ts
- crates/jj-store-cli/tests/local_browser/code_summary.rs
- crates/jj-store-cli/tests/local_browser/preparation.rs
- crates/jj-store-cli/tests/local_browser/prepared_comparisons.rs
- crates/jj-store-cli/tests/local_browser/prepared_operations.rs
- crates/jj-store-cli/tests/local_browser/prepared_points_http.rs
- crates/jj-store-cli/tests/local_browser/prepared_references.rs
- crates/jj-store-cli/tests/local_browser/reviews.rs
- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-cli/tests/native_lifecycle/browsing.rs
- crates/jj-store-cli/tests/native_lifecycle/code_summary.rs
- crates/jj-store-cli/tests/native_lifecycle/content.rs
- crates/jj-store-cli/tests/native_lifecycle/filter_cache.rs
- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs
- crates/jj-store-cli/tests/native_lifecycle/graph_disk_recovery.rs
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs
- crates/jj-store-cli/tests/native_lifecycle/http_collaboration_reviews.rs
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs
- crates/jj-store-cli/tests/native_lifecycle/http_work.rs
- crates/jj-store-cli/tests/native_lifecycle/query_workers.rs
- crates/jj-store-cli/tests/native_lifecycle/reference_filters.rs
- crates/jj-store-cli/tests/native_lifecycle/reference_windows.rs
- crates/jj-store-http/src/collaboration.rs
- crates/jj-store-http/src/contract/review_tests.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/contract.rs
- crates/jj-store-http/src/local_browser/reviews.rs
- crates/jj-store-http/src/local_browser/work.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-http/src/work.rs
- crates/jj-store-jj/src/local_browser/code_summary.rs
- crates/jj-store-jj/src/local_browser/dispatch.rs
- crates/jj-store-jj/src/local_browser/graphs.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/operation_changes.rs
- crates/jj-store-jj/src/local_browser/operation_store.rs
- crates/jj-store-jj/src/local_browser/operations.rs
- crates/jj-store-jj/src/local_browser/preparation/comparison.rs
- crates/jj-store-jj/src/local_browser/preparation/comparison_tests.rs
- crates/jj-store-jj/src/local_browser/preparation/graph.rs
- crates/jj-store-jj/src/local_browser/preparation.rs
- crates/jj-store-jj/src/local_browser/references/prepared.rs
- crates/jj-store-jj/src/local_browser/references.rs
- crates/jj-store-jj/src/query/access.rs
- crates/jj-store-jj/src/query/cache/constructed.rs
- crates/jj-store-jj/src/query/cache/generation.rs
- crates/jj-store-jj/src/query/cache/proof.rs
- crates/jj-store-jj/src/query/cache/proof_tests.rs
- crates/jj-store-jj/src/query/cache/recovery.rs
- crates/jj-store-jj/src/query/cache/reservation.rs
- crates/jj-store-jj/src/query/cache/resident/additional.rs
- crates/jj-store-jj/src/query/cache/resident.rs
- crates/jj-store-jj/src/query/cache.rs
- crates/jj-store-jj/src/query/content/code_summary.rs
- crates/jj-store-jj/src/query/content/references/filter.rs
- crates/jj-store-jj/src/query/content/references/prepared.rs
- crates/jj-store-jj/src/query/content/references.rs
- crates/jj-store-jj/src/query/content.rs
- crates/jj-store-jj/src/query/index/secondary.rs
- crates/jj-store-jj/src/query/index.rs
- crates/jj-store-jj/src/query/language/operations.rs
- crates/jj-store-jj/src/query/memory/retained.rs
- crates/jj-store-jj/src/query/memory.rs
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-jj/src/query/prepared/disk.rs
- crates/jj-store-jj/src/query/prepared/filters.rs
- crates/jj-store-jj/src/query/prepared/mod.rs
- crates/jj-store-jj/src/query/prepared/operations.rs
- crates/jj-store-jj/src/query/prepared/segment.rs
- crates/jj-store-jj/src/query/prepared/tables.rs
- crates/jj-store-jj/src/query/projection/changes.rs
- crates/jj-store-jj/src/query/projection/decorations.rs
- crates/jj-store-jj/src/query/projection/filter.rs
- crates/jj-store-jj/src/query/projection/references/tests.rs
- crates/jj-store-jj/src/query/projection/references.rs
- crates/jj-store-jj/src/query/projection/tests.rs
- crates/jj-store-jj/src/query/projection.rs
- crates/jj-store-jj/src/query/workers.rs
- crates/jj-store-service/src/collaboration/review_activity/cache.rs
- crates/jj-store-service/src/collaboration/review_activity/cache_tests.rs
- crates/jj-store-service/src/collaboration/review_activity/events.rs
- crates/jj-store-service/src/collaboration/review_activity/tests.rs
- crates/jj-store-service/src/collaboration/review_activity.rs
- crates/jj-store-service/src/collaboration/review_models.rs
- crates/jj-store-service/src/collaboration/review_threads.rs
- crates/jj-store-service/src/collaboration.rs
- crates/jj-store-service/src/local_repository/browser_reviews.rs
- crates/jj-store-service/src/local_repository/browser_work.rs
- crates/jj-store-service/src/local_repository/work.rs
- crates/jj-store-service/src/local_repository/work_cache/data.rs
- crates/jj-store-service/src/local_repository/work_cache/memory.rs
- crates/jj-store-service/src/local_repository/work_cache/tests.rs
- crates/jj-store-service/src/local_repository/work_cache.rs
- crates/jj-store-service/src/local_repository/work_pages.rs
- crates/jj-store-service/src/local_repository.rs
- crates/jj-store-service/src/pagination/browsing.rs
- crates/jj-store-service/src/reads.rs
- crates/jj-store-service/src/work/activity.rs
- crates/jj-store-service/src/work/browse.rs
- crates/jj-store-service/src/work/browse_tests.rs
- crates/jj-store-service/src/work/cursors.rs
- crates/jj-store-service/src/work/projection.rs
- crates/jj-store-service/src/work.rs
- crates/jj-store-service/tests/local_review.rs
- crates/jj-store-service/tests/local_work.rs
- crates/jj-store-service/tests/review_threads.rs
- crates/jj-store-service/tests/work_store/cache.rs
- docs/implementation-plan.md
- schemas/openapi.json
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/projects.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/http-local-code-data-source.ts
- web/src/lib/controller/http-local-review-reader.ts
- web/src/lib/controller/http-local-store-data-source.ts
- web/src/lib/controller/http-local-work-data-source.ts
- web/src/lib/controller/http-repository-directory.ts
- web/src/lib/controller/http-work-data-source.ts
- web/src/lib/controller/local-repository-directory.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/projects-hub-types.ts
- web/src/lib/controller/projects-repository-types.ts
- web/src/lib/controller/repository-read-types.ts
- web/src/lib/controller/work-types.ts
- web/src/lib/ui/store-work/repository-directory-presentation.ts
- web/tests/vite/local-browser.test.ts
- web/tests/vite/projects-repositories.test.ts
- web/tests/vite/projects-repository-visibility.test.ts
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit the verified landing, advance codex/bounded-reads-landed, and continue high-priority native embedded-operation reference windows in the retained bounded-reads workspace based on this merge. Keep remaining performance delivery and complete qualification pending.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Explicit user-requested main landing and verification. Accumulated changes touch native and local query packages, public routes/generated clients and repository-directory consumers. Run the directly affected JJ/HTTP/service library and native/local browser test binaries, registered response-contract check and affected UI/type/boundary lanes once on the combined source. These wider relevant lanes are justified by the merged independent consumers and shared public contracts. Root aggregates, Linux/two-million/near-capacity fixtures, measured disk/RSS/release latency, complete cold production browser/fault/RustFS and registry/visual release qualification remain deferred.
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-GOV-002
- JST-GOV-003
- JST-GOV-005
- JST-JJ-003
- JST-JJ-005
- JST-API-001
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-http/src/contract/tests.rs#contract_distinguishes_optional_inputs_from_nullable_outputs
- crates/jj-store-cli/tests/native_lifecycle/graph_disk_recovery.rs#existing_disk_projection_recovery_releases_namespace_and_preserves_generations
- crates/jj-store-jj/src/query/cache/proof_tests.rs#cold_cache_admission_does_not_charge_full_file_hashing_under_namespace
- web/tests/vite/projects-repositories.test.ts#native code card reads a bounded
  head summary without requesting history
- crates/jj-store-http/src/contract/tests.rs#contract_inventory_covers_security_methods_media_and_task_bodies
- web/tests/vite/local-browser.test.ts#local_code_summary_cancellation_releases_only_the_callers_active_query_lease
- web/tests/vite/local-browser.test.ts#local_review_activity_cancellation_releases_only_the_callers_active_query_lease
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-049 details 2](details-02.md)
- [E-READ-049 details 3](details-03.md)
- [E-READ-049 details 4](details-04.md)
