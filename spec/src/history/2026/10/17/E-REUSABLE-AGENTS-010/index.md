---
schema: 1
id: E-REUSABLE-AGENTS-010
---

# E-REUSABLE-AGENTS-010

<!-- evidence-field:id markdown -->
## Id

E-REUSABLE-AGENTS-010
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

- .changeset/reusable-workspace-discovery.md
- crates/jj-store-cli/src/local_browser.rs
- crates/jj-store-jj/README.md
- crates/jj-store-jj/src/local_repository/discovery.rs
- crates/jj-store-jj/src/local_repository.rs
- crates/jj-store-jj/tests/local_discovery.rs
- docs/maintenance/coverage.json
- docs/site/cli/index.md
- docs/site/technical/local.md
- scripts/local-history-features.test.mjs
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/contracts/cli/35-interactive-agent-execution.md
- spec/src/current/contracts/documentation/03-delivery-and-validation.md
- spec/src/current/contracts/jj-integration/14-bounded-application-graphs.md
- spec/src/current/contracts/local-jj/12-bounded-local-code-reads.md
- spec/src/current/governance/23-shared-ai-clients.md
- spec/src/current/progress/M33/history/M33/index.md
- spec/src/current/progress/M33/history/M33.4/index.md
- spec/src/current/progress/M33/index.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-010/SUMMARY.nav.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-010/details-02.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-010/details-03.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-010/index.md
- spec/src/history/2026/10/17/SUMMARY.nav.md
- spec/src/history/2026/10/17/index.md
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

[]
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser.rs#local_folder_inspection_preserves_uninitialized_metadata_and_dirty_files
- crates/jj-store-cli/tests/local_browser.rs#filesystem_runtime_registers_and_reads_without_native_storage
- crates/jj-store-jj/tests/local_discovery.rs#discovery_and_default_carrier_open_preserve_dirty_files_identity_and_repository_config
- crates/jj-store-jj/tests/local_discovery.rs#linked_workspaces_resolve_shared_storage_without_reconciliation_or_initialization
- crates/jj-store-jj/tests/local_discovery.rs#discovery_rejects_relative_roots_oversized_links_and_changed_observations
- crates/jj-store-jj/tests/local_discovery.rs#discovery_keeps_original_symlink_rejection
<!-- /evidence-field -->

