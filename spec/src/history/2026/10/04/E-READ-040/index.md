---
schema: 1
id: E-READ-040
---

# E-READ-040

<!-- evidence-field:id markdown -->
## Id

E-READ-040
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.1
- M11.4
- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change nvpkztls
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Finite native and local code-head summaries for repository cards without commit history preparation
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1 (48a229cea); jj-lib0.45.1; stock JJ0.45.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

One additive native GET code/summary route and local operation-pinned code_summary query variant. Generated inventory118 paths (42local/76other). Version1 finite1..256 nonvirtual lexical head samples with count uncertainty and newest sampled committer metadata; cards request50. Native64-byte/Git20-byte identities, manifest/WAL, companion, cursor and domain formats/capacities unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser/code_summary.rs
- crates/jj-store-cli/tests/local_browser/prepared_points_http.rs
- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-cli/tests/native_lifecycle/code_summary.rs
- crates/jj-store-cli/tests/native_lifecycle/content.rs
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs
- crates/jj-store-http/src/contract/review_tests.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-jj/src/local_browser/code_summary.rs
- crates/jj-store-jj/src/local_browser/dispatch.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/query/content/code_summary.rs
- crates/jj-store-jj/src/query/content.rs
- crates/jj-store-service/src/reads.rs
- docs/implementation-plan.md
- schemas/openapi.json
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/mutations.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/http-local-code-data-source.ts
- web/src/lib/controller/http-repository-directory.ts
- web/src/lib/controller/local-repository-directory.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/repository-read-types.ts
- web/tests/vite/local-browser.test.ts
- web/tests/vite/projects-repositories.test.ts
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-API-001
- JST-API-004
- JST-JJ-003
- JST-JJ-005
- JST-LOCAL-004
- JST-UI-015
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/projects-repositories.test.ts#native code card reads a bounded
  head summary without requesting history
- web/tests/vite/projects-repositories.test.ts#local code card consumes pinned
  head metadata without requesting a graph
- web/tests/vite/local-browser.test.ts#local_code_summary_uses_a_finite_pin_and_forwards_cancellation
- crates/jj-store-cli/tests/native_lifecycle/code_summary.rs#native_code_summary_hydrates_only_selected_heads_without_ancestry_or_trees
- crates/jj-store-cli/tests/native_lifecycle/code_summary.rs#native_code_summary_orders_times_ties_and_complete_root_child_counts
- crates/jj-store-cli/tests/native_lifecycle/code_summary.rs#native_code_summary_excludes_the_virtual_root_and_enforces_limits
- crates/jj-store-cli/tests/native_lifecycle/code_summary.rs#native_code_summary_retains_historical_selection_and_fresh_authorization
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs#native_code_summary_http_is_pinned_strict_and_avoids_preparing_a_graph
- crates/jj-store-cli/tests/local_browser/code_summary.rs#local_code_summary_skips_damaged_indexes_missing_extras_ancestry_and_trees
- crates/jj-store-cli/tests/local_browser/code_summary.rs#local_code_summary_hydrates_only_the_lexical_window_and_marks_remaining_heads
- crates/jj-store-cli/tests/local_browser/prepared_points_http.rs#local_point_http_uses_ready_membership_and_only_falls_back_when_cold
- crates/jj-store-http/src/contract/review_tests.rs#code_summary_contract_has_strict_head_windows_and_native_local_identity_widths
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-040 details 2](details-02.md)
- [E-READ-040 details 3](details-03.md)
