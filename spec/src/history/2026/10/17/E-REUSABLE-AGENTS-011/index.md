---
schema: 1
id: E-REUSABLE-AGENTS-011
---

# E-REUSABLE-AGENTS-011

<!-- evidence-field:id markdown -->
## Id

E-REUSABLE-AGENTS-011
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M33
- M33.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-007
- JST-LOCAL-001
- JST-LOCAL-004
- JST-API-001
- JST-API-004
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

- .changeset/history-wire-integers.md
- .prettierignore
- Cargo.lock
- crates/tauri-plugin-jj-store/Cargo.toml
- crates/tauri-plugin-jj-store/README.md
- crates/tauri-plugin-jj-store/examples/schema.rs
- crates/tauri-plugin-jj-store/schema.json
- crates/tauri-plugin-jj-store/src/frames.rs
- crates/tauri-plugin-jj-store/src/lib.rs
- crates/tauri-plugin-jj-store/src/service.rs
- crates/tauri-plugin-jj-store/src/transport/commands.rs
- crates/tauri-plugin-jj-store/src/wire.rs
- crates/tauri-plugin-jj-store/tests/frames.rs
- crates/tauri-plugin-jj-store/tests/schema.rs
- crates/tauri-plugin-jj-store/tests/wire.rs
- docs/maintenance/coverage.json
- docs/site/technical/local.md
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/contracts/documentation/03-delivery-and-validation.md
- spec/src/current/contracts/local-jj/12-bounded-local-code-reads.md
- spec/src/current/governance/23-shared-ai-clients.md
- spec/src/current/progress/M33/history/M33/index.md
- spec/src/current/progress/M33/history/M33.4/index.md
- spec/src/current/progress/M33/index.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-011/SUMMARY.nav.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-011/details-02.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-011/details-03.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-011/index.md
- spec/src/history/2026/10/17/SUMMARY.nav.md
- spec/src/history/2026/10/17/index.md
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

- crates/tauri-plugin-jj-store/src/transport/commands/tests.rs
<!-- /evidence-field -->

<!-- evidence-field:removed_tests list -->
## Removed tests

- crates/tauri-plugin-jj-store/src/transport/commands/tests.rs#renderer_frames_cannot_supply_native_authority
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/tauri-plugin-jj-store/tests/frames.rs#renderer_frames_cannot_supply_native_authority
- crates/tauri-plugin-jj-store/tests/wire.rs#text_envelopes_preserve_full_width_integers_unicode_and_exact_escaping
- crates/tauri-plugin-jj-store/tests/wire.rs#envelope_version_shape_and_byte_limits_fail_closed
- crates/tauri-plugin-jj-store/tests/schema.rs#checked_schema_matches_owned_native_history_contract
<!-- /evidence-field -->

