---
schema: 1
id: E-LOCAL-TEST-AUDIT-001
---

# E-LOCAL-TEST-AUDIT-001

<!-- evidence-field:id markdown -->
## Id

E-LOCAL-TEST-AUDIT-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
- M3.2
- M4.1
- M18
- M33
- M33.4
- M3
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-GOV-003
- JST-GOV-007
- JST-GOV-011
- JST-UI-001
- JST-UI-018
- JST-JJ-001
- JST-JJ-003
- JST-JJ-005
- JST-ARCH-005
- JST-CLI-001
- JST-COLLAB-018
- JST-DOC-001
- JST-DOC-002
- JST-DESK-006
- JST-UI-002
- JST-UI-005
- JST-ARCH-007
- JST-LOCAL-004
- JST-API-001
- JST-API-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/vitest.config.ts
- catalog/components/store-application/StoreWorkspaces.stories.svelte
- catalog/components/store-appearance/AppearanceOverlay.stories.svelte
- spec/src/current/contracts/ui/42-application-story-parity.md
- spec/src/current/governance/09-catalog-validation.md
- spec/src/current/progress/M30/index.md
- scripts/validation/Dockerfile
- Cargo.lock
- crates/jj-store-jj/src/query/projection.rs
- crates/jj-store-jj/src/query/projection/mutable.rs
- crates/jj-store-rpc/Cargo.toml
- crates/jj-store-rpc/src/lib.rs
- crates/jj-store-rpc/src/tls.rs
- crates/jj-store-rpc/src/client.rs
- crates/jj-store-rpc/src/server.rs
- crates/jj-store-rpc/tests/tls_provider_default.rs
- crates/jj-store-rpc/tests/tls_provider_existing.rs
- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs
- spec/src/current/contracts/jj-integration/12-native-rpc-v1.md
- spec/src/current/contracts/jj-integration/15-managed-graph-cache-lifecycle.md
- spec/src/current/progress/M3/index.md
- spec/src/current/progress/M4/index.md
- spec/src/current/verification/jj.md
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/cli/05-executable-native-workspace-client.md
- crates/jj-store-server/tests/capture_import_state.rs
- spec/src/current/governance/25-test-quarantines.md
- spec/src/current/governance/index.md
- spec/src/current/governance/SUMMARY.nav.md
- spec/src/current/contracts/collaboration/09-explicit-inactive-history-import.md
- spec/src/current/progress/M18/index.md
- crates/jj-store-server/src/capture_journal_files.rs
- spec/src/current/verification/collab-02.md
- docs/site/technical/local.md
- docs/site/reference/transport.md
- docs/site/technical/lifecycle.md
- docs/maintenance/coverage.json
- spec/src/current/contracts/documentation/03-delivery-and-validation.md
- web/tests/browser/comparison-parents.spec.ts
- spec/src/current/contracts/ui/14-shared-application-continued-2.md
- spec/src/current/verification/doc.md
- crates/jj-history-service/tests/file_history/mod.rs
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/progress/M33/index.md
- spec/src/current/verification/arch.md
- scripts/validation/model.mjs
- scripts/validation/worker.mjs
- scripts/validation.test.mjs
- spec/src/current/contracts/api/09-operation-differences.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#historical_revset_difference_preserves_exact_operation_visibility
- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#cold_historical_indexes_leave_the_primary_generation_immutable
- crates/jj-store-cli/tests/native_rpc.rs#rpc_tls_checks_trust_and_server_hostname
- crates/jj-store-rpc/tests/tls_provider_default.rs#https_initialization_supports_both_compiled_crypto_backends
- crates/jj-store-rpc/tests/tls_provider_existing.rs#https_initialization_retains_the_host_crypto_provider
- crates/jj-store-server/src/capture_journal_files.rs#guard_drop_releases_lock_while_a_duplicate_descriptor_is_alive
- crates/jj-history-service/tests/file_history/mod.rs#file_history_keeps_spaces_quotes_and_pattern_characters_literal
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-LOCAL-TEST-AUDIT-001](details-02.md)
- [E-LOCAL-TEST-AUDIT-001](details-03.md)
- [E-LOCAL-TEST-AUDIT-001](details-04.md)
- [E-LOCAL-TEST-AUDIT-001](details-05.md)
- [E-LOCAL-TEST-AUDIT-001](details-06.md)
- [E-LOCAL-TEST-AUDIT-001](details-07.md)
