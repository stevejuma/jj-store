---
schema: 1
id: E-REUSABLE-AGENTS-007
---

# E-REUSABLE-AGENTS-007

<!-- evidence-field:id markdown -->
## Id

E-REUSABLE-AGENTS-007
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M33
- M33.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-002
- JST-ARCH-007
- JST-JJ-003
- JST-JJ-005
- JST-API-001
- JST-API-004
- JST-API-006
- JST-DOC-001
- JST-DOC-002
- JST-GOV-002
- JST-GOV-011
- JST-ARCH-005
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-17
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .changeset/shared-read-admission.md
- crates/jj-store-jj/README.md
- crates/jj-store-jj/src/lib.rs
- crates/jj-store-jj/src/query/admission.rs
- crates/jj-store-jj/src/query/content.rs
- crates/jj-store-jj/src/query/language/mod.rs
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-jj/src/query/predicates.rs
- crates/jj-store-jj/src/query/workers.rs
- crates/jj-store-jj/src/read_support/accounting.rs
- crates/jj-store-jj/src/read_support/error.rs
- crates/jj-store-jj/src/read_support/limits.rs
- crates/jj-store-jj/src/read_support/mod.rs
- crates/jj-store-jj/src/read_support/workers.rs
- docs/maintenance/coverage.json
- docs/site/technical/local.md
- spec/src/current/contracts/architecture/10-pinned-history-query-ownership.md
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/contracts/documentation/03-delivery-and-validation.md
- spec/src/current/contracts/jj-integration/14-bounded-application-graphs.md
- spec/src/current/progress/M33/history/M33/index.md
- spec/src/current/progress/M33/history/M33.4/index.md
- spec/src/current/progress/M33/index.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-007/SUMMARY.nav.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-007/details-02.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-007/details-03.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-007/index.md
- spec/src/history/2026/10/17/SUMMARY.nav.md
- spec/src/history/2026/10/17/index.md
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

[]
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/src/query/workers.rs#background_observation_preserves_foreground_admission
- crates/jj-store-jj/src/query/workers.rs#retained_query_clones_capture_updated_limits_for_new_jobs
- crates/jj-store-jj/src/query/cache/proof_tests.rs#cold_proof_budget_and_cancellation_do_not_become_rebuild_success
- crates/jj-store-jj/src/query/cache/proof_tests.rs#failed_cold_recovery_admission_preserves_existing_files_and_receipt
<!-- /evidence-field -->

