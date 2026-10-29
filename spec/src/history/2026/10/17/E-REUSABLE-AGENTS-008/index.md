---
schema: 1
id: E-REUSABLE-AGENTS-008
---

# E-REUSABLE-AGENTS-008

<!-- evidence-field:id markdown -->
## Id

E-REUSABLE-AGENTS-008
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M33
- M33.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-002
- JST-ARCH-005
- JST-ARCH-007
- JST-JJ-003
- JST-JJ-005
- JST-LOCAL-001
- JST-LOCAL-003
- JST-LOCAL-004
- JST-API-001
- JST-API-004
- JST-API-006
- JST-DOC-001
- JST-DOC-002
- JST-GOV-002
- JST-GOV-011
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

- .changeset/local-history-profile.md
- crates/jj-store-jj/Cargo.toml
- crates/jj-store-jj/README.md
- crates/jj-store-jj/src/factories.rs
- crates/jj-store-jj/src/lib.rs
- crates/jj-store-jj/src/local_browser/git_read/extras.rs
- crates/jj-store-jj/src/local_browser/git_read.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/query/access.rs
- crates/jj-store-jj/src/query/admission.rs
- crates/jj-store-jj/src/query/cache/generation.rs
- crates/jj-store-jj/src/query/cache.rs
- crates/jj-store-jj/src/query/code_index/merge_files.rs
- crates/jj-store-jj/src/query/code_index.rs
- crates/jj-store-jj/src/query/content/paths.rs
- crates/jj-store-jj/src/query/content.rs
- crates/jj-store-jj/src/query/memory.rs
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-jj/src/query/prepared/mod.rs
- crates/jj-store-jj/src/query/prepared/operations/children.rs
- crates/jj-store-jj/src/query/prepared/operations/history.rs
- crates/jj-store-jj/src/query/prepared/operations.rs
- crates/jj-store-jj/src/query/workers.rs
- crates/jj-store-jj/src/read_support/limits.rs
- crates/jj-store-jj/src/read_support/mod.rs
- crates/jj-store-jj/tests/local_history.rs
- docs/maintenance/coverage.json
- docs/site/technical/local.md
- package.json
- scripts/local-history-features.test.mjs
- spec/src/current/contracts/architecture/10-pinned-history-query-ownership.md
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/contracts/documentation/03-delivery-and-validation.md
- spec/src/current/contracts/jj-integration/14-bounded-application-graphs.md
- spec/src/current/contracts/local-jj/12-bounded-local-code-reads.md
- spec/src/current/governance/23-shared-ai-clients.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M33/history/M33/index.md
- spec/src/current/progress/M33/history/M33.4/index.md
- spec/src/current/progress/M33/index.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-008/SUMMARY.nav.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-008/details-02.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-008/details-03.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-008/index.md
- spec/src/history/2026/10/17/SUMMARY.nav.md
- spec/src/history/2026/10/17/index.md
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

[]
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/local_history.rs#standalone_history_preserves_source_pins_diffs_cancellation_and_current_admission
- scripts/local-history-features.test.mjs#standalone local history excludes
  native publication and host runtimes
- scripts/local-history-features.test.mjs#default native retains every
  integration binary and headless gates native-only tests
<!-- /evidence-field -->

