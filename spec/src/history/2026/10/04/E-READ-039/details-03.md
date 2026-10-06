---
schema: 1
id: E-READ-039
---

# E-READ-039 details 3

<!-- evidence-field:next markdown -->
## Next

Continue lightweight code summaries and root-bound review activity reuse, then remaining indexed lookup/reference/deep/RPC delivery; complete combined qualification in the later slice.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One cross-layer compact repository-card review activity behavior and its direct native/local service, HTTP/schema, directory and local transport consumers.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/src/collaboration/review_activity/tests.rs#activity_reads_only_window_events_and_uses_maximum_across_pages
- crates/jj-store-service/src/collaboration/review_activity/tests.rs#merged_activity_reads_no_comment_conflicts_and_uses_every_event_time
- crates/jj-store-service/tests/review_threads.rs#review_activity_matches_the_pinned_catalog_window_without_publishing
- crates/jj-store-service/tests/local_review.rs#local_review_activity_retains_historical_selection_and_partial_counts
- crates/jj-store-http/src/contract/review_tests.rs#review_activity_contract_is_compact_bounded_and_independently_pinned
- crates/jj-store-cli/tests/native_lifecycle/http_collaboration_reviews.rs#local_trusted_http_pages_root_pinned_review_threads_and_resolves_detail
- crates/jj-store-cli/tests/local_browser.rs#filesystem_runtime_registers_and_reads_without_native_storage
- web/tests/vite/projects-repositories.test.ts#native card review activity
  avoids thread presentation and keeps sample uncertainty
- web/tests/vite/local-browser.test.ts#local_review_activity_uses_explicit_pin_and_rejects_foreign_results
<!-- /evidence-field -->

