---
schema: 1
id: E-REUSABLE-AGENTS-001
---

# E-REUSABLE-AGENTS-001

<!-- evidence-field:id markdown -->
## Id

E-REUSABLE-AGENTS-001
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

- .changeset/native-agent-privacy.md
- .github/actions/setup/action.yml
- Cargo.lock
- Cargo.toml
- crates/jj-store-capture/Cargo.toml
- crates/jj-store-capture/README.md
- crates/jj-store-capture/src/agent_sanitization.rs
- crates/jj-store-capture/src/transcript_sanitization/stream_text.rs
- crates/jj-store-capture/src/transcript_sanitization.rs
- crates/jj-store-capture/tests/native_privacy_embedding.rs
- docs/local-compute.md
- docs/maintenance/coverage.json
- docs/site/technical/capture.md
- scripts/agent-redaction-source.json
- scripts/check-boundaries.mjs
- scripts/ci-lapis-source.mjs
- scripts/ci-packages.mjs
- scripts/ci-pinned-source.mjs
- scripts/ci-redaction-source.mjs
- scripts/ci-selection.test.mjs
- scripts/ci-source.test.mjs
- scripts/governance.test.mjs
- spec/src/current/contracts/architecture/03-requirements.md
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/contracts/architecture/SUMMARY.nav.md
- spec/src/current/contracts/architecture/index.md
- spec/src/current/contracts/collaboration/21-capture-sanitization-boundary.md
- spec/src/current/contracts/compatibility/13-interactive-agent-execution.md
- spec/src/current/contracts/documentation/08-interactive-agent-execution.md
- spec/src/current/governance/23-shared-ai-clients.md
- spec/src/current/progress/M33/SUMMARY.nav.md
- spec/src/current/progress/M33/history/M33/SUMMARY.nav.md
- spec/src/current/progress/M33/history/M33/index.md
- spec/src/current/progress/M33/history/M33.1/SUMMARY.nav.md
- spec/src/current/progress/M33/history/M33.1/index.md
- spec/src/current/progress/M33/history/M33.2/SUMMARY.nav.md
- spec/src/current/progress/M33/history/M33.2/index.md
- spec/src/current/progress/M33/history/M33.3/SUMMARY.nav.md
- spec/src/current/progress/M33/history/M33.3/index.md
- spec/src/current/progress/M33/history/M33.4/SUMMARY.nav.md
- spec/src/current/progress/M33/history/M33.4/index.md
- spec/src/current/progress/M33/history/M33.5/SUMMARY.nav.md
- spec/src/current/progress/M33/history/M33.5/index.md
- spec/src/current/progress/M33/history/M33.6/SUMMARY.nav.md
- spec/src/current/progress/M33/history/M33.6/index.md
- spec/src/current/progress/M33/history/M33.7/SUMMARY.nav.md
- spec/src/current/progress/M33/history/M33.7/index.md
- spec/src/current/progress/M33/history/M33.8/SUMMARY.nav.md
- spec/src/current/progress/M33/history/M33.8/index.md
- spec/src/current/progress/M33/history/SUMMARY.nav.md
- spec/src/current/progress/M33/history/index.md
- spec/src/current/progress/M33/index.md
- spec/src/current/progress/SUMMARY.nav.md
- spec/src/current/progress/index.md
- spec/src/current/verification/arch.md
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

- crates/jj-store-capture/src/transcript_sanitization/stream_text.rs
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-capture/tests/native_privacy_embedding.rs#jj_payload_adapters_preserve_independent_native_results
- crates/jj-store-capture/tests/transcript_sanitization.rs#streaming_paths_are_private_at_every_chunk_boundary
- crates/jj-store-capture/tests/transcript_sanitization.rs#transcript_sanitization_precedes_durability_and_covers_path_keys_and_values
<!-- /evidence-field -->

