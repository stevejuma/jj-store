---
schema: 1
id: E-REUSABLE-AGENTS-002
---

# E-REUSABLE-AGENTS-002

<!-- evidence-field:id markdown -->
## Id

E-REUSABLE-AGENTS-002
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

- .changeset/reusable-session-acquisition.md
- Cargo.lock
- Cargo.toml
- crates/jj-store-capture/Cargo.toml
- crates/jj-store-capture/README.md
- crates/jj-store-capture/src/codex_discovery/scanner.rs
- crates/jj-store-capture/src/codex_discovery.rs
- crates/jj-store-capture/src/codex_transcript/file.rs
- crates/jj-store-capture/src/profile.rs
- docs/maintenance/coverage.json
- docs/site/technical/capture.md
- scripts/acquisition-boundaries.test.mjs
- scripts/check-boundaries.mjs
- scripts/ci-packages.mjs
- scripts/ci-selection.test.mjs
- scripts/lapis-ai-source.json
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/contracts/compatibility/13-interactive-agent-execution.md
- spec/src/current/contracts/documentation/08-interactive-agent-execution.md
- spec/src/current/governance/23-shared-ai-clients.md
- spec/src/current/progress/M33/history/M33/index.md
- spec/src/current/progress/M33/history/M33.1/index.md
- spec/src/current/progress/M33/history/M33.2/index.md
- spec/src/current/progress/M33/index.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-002/SUMMARY.nav.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-002/details-02.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-002/index.md
- spec/src/history/2026/10/17/SUMMARY.nav.md
- spec/src/history/2026/10/17/index.md
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

[]
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-capture/tests/codex_discovery.rs#discovery_withholds_rebound_roots_symlink_children_and_changed_candidates
- crates/jj-store-capture/tests/cursor_transcript.rs#cursor_default_discovery_skips_generated_project_payloads
- crates/jj-store-capture/src/codex_transcript/file.rs#codex_witness_retains_exact_header_bytes_and_native_file_identity
- crates/jj-store-capture/src/codex_discovery.rs#current_binding_distinguishes_absence_from_unknown_io_outcomes
<!-- /evidence-field -->

