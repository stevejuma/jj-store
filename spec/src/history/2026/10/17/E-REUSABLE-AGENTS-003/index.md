---
schema: 1
id: E-REUSABLE-AGENTS-003
---

# E-REUSABLE-AGENTS-003

<!-- evidence-field:id markdown -->
## Id

E-REUSABLE-AGENTS-003
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M33
- M33.1
- M33.2
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-ARCH-007
- JST-COLLAB-031
- JST-COMPAT-001
- JST-DOC-001
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

- .changeset/headless-capture.md
- crates/jj-store-capture/Cargo.toml
- crates/jj-store-capture/README.md
- crates/jj-store-capture/src/agent_capture/journal.rs
- crates/jj-store-capture/src/agent_capture.rs
- crates/jj-store-capture/src/agent_capture_files.rs
- crates/jj-store-capture/src/agent_capture_lock.rs
- crates/jj-store-capture/src/codex_source.rs
- crates/jj-store-capture/src/codex_transcript.rs
- crates/jj-store-capture/src/lib.rs
- crates/jj-store-capture/src/reader_progress/source.rs
- crates/jj-store-capture/src/reader_progress.rs
- crates/jj-store-capture/src/source_registry/codex.rs
- crates/jj-store-capture/src/state_lease.rs
- crates/jj-store-capture/tests/headless_capture.rs
- crates/jj-store-capture/tests/opencode_recovery_index.rs
- docs/maintenance/coverage.json
- docs/site/technical/capture.md
- package.json
- scripts/capture-features.test.mjs
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/contracts/compatibility/13-interactive-agent-execution.md
- spec/src/current/contracts/documentation/08-interactive-agent-execution.md
- spec/src/current/governance/23-shared-ai-clients.md
- spec/src/current/progress/M33/history/M33/index.md
- spec/src/current/progress/M33/history/M33.1/index.md
- spec/src/current/progress/M33/history/M33.2/index.md
- spec/src/current/progress/M33/index.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-003/SUMMARY.nav.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-003/details-02.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-003/index.md
- spec/src/history/2026/10/17/SUMMARY.nav.md
- spec/src/history/2026/10/17/index.md
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

[]
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-capture/tests/headless_capture.rs#headless_reader_retains_exact_pending_identity_until_host_acknowledgement
- crates/jj-store-capture/tests/opencode_recovery_index.rs#malformed_or_unbound_index_withholds_registry_and_legacy_empty_index_is_readable
- crates/jj-store-capture/tests/codex_discovery.rs#discovery_withholds_rebound_roots_symlink_children_and_changed_candidates
<!-- /evidence-field -->

